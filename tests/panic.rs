use sangoku_test_support::{Result, ensure, harness};

#[test]
fn panic_handler_halts_without_returning() -> Result<()> {
    let program = harness::assemble_tiny(
        &[
            (
                "stop.asm",
                "SECTION \"reserved\", ROM0[$0000]\nds $100, 0\ndone::\nnop\n".into(),
            ),
            ("panic.asm", harness::read("runtime/panic.asm")?),
        ],
        None,
    )?;
    let mut e = program.emulator("__rust__RNvCsbOwsw4zHH1Z_7___rustc17rust_begin_unwind")?;
    e.set_sp(0xdf00);
    e.write16(0xdf00, program.symbols["done"]);
    let entry = e.pc;
    for _ in 0..1000 {
        e.step()?;
    }
    ensure!(e.pc != program.symbols["done"], "handler returned");
    ensure!(e.pc > entry, "handler did not reach the halt loop");
    ensure!(e.sp() == 0xdf00, "handler touched the stack");
    Ok(())
}
