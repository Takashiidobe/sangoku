use sangoku_test_support::{Result, emu::Emulator, ensure};
use std::{fs, process::Command};

#[test]
fn bounds_check_aborts_without_a_panic_handler() -> Result<()> {
    let directory = sangoku_test_support::tempfile::tempdir()?;
    let rom = directory.path().join("immediate_abort.gb");
    let output = Command::new(env!("CARGO_BIN_EXE_sangoku"))
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .args(["build", "tests/core_rom/immediate_abort.rs"])
        .arg(&rom)
        .output()?;
    ensure!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let mut emulator = Emulator::new(&fs::read(&rom)?, 0x100)?;
    emulator.set_sp(0xe000);
    emulator.write8(0xc000, 1);
    for _ in 0..100_000 {
        let before = emulator.pc;
        emulator.step()?;
        if emulator.pc == before {
            ensure!(emulator.pc == 0x11, "did not reach the abort trap");
            ensure!(emulator.serial.is_empty(), "abort wrote serial output");
            return Ok(());
        }
    }
    sangoku_test_support::bail!("bounds check did not abort")
}

#[test]
fn program_using_core_links_and_runs() -> Result<()> {
    let directory = sangoku_test_support::tempfile::tempdir()?;
    let rom = directory.path().join("core_rom.gb");
    let output = Command::new(env!("CARGO_BIN_EXE_sangoku"))
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .args(["build", "tests/core_rom/rom.rs"])
        .arg(&rom)
        .output()?;
    ensure!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let mut emulator = Emulator::new(&fs::read(&rom)?, 0x100)?;
    emulator.set_sp(0xe000);
    for _ in 0..2_000_000 {
        let before = emulator.pc;
        emulator.step()?;
        if emulator.pc == before {
            ensure!(emulator.a == 28, "gbc_main returned {}", emulator.a);
            return Ok(());
        }
    }
    sangoku_test_support::bail!("program did not halt")
}

fn run_core_rom(source: &str, opt_level: &str) -> Result<(u8, Vec<u8>)> {
    let directory = sangoku_test_support::tempfile::tempdir()?;
    let rom = directory.path().join("core_rom.gb");
    let output = Command::new(env!("CARGO_BIN_EXE_sangoku"))
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .args(["build", source])
        .arg(&rom)
        .args(["--opt-level", opt_level])
        .output()?;
    ensure!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let mut emulator = Emulator::new(&fs::read(&rom)?, 0x100)?;
    emulator.set_sp(0xe000);
    for _ in 0..2_000_000 {
        let before = emulator.pc;
        emulator.step()?;
        if emulator.pc == before {
            return Ok((emulator.a, emulator.serial));
        }
    }
    sangoku_test_support::bail!("{source} did not halt")
}

#[test]
fn utf8_iteration_matches_at_every_width_boundary() -> Result<()> {
    let (result, _) = run_core_rom("tests/core_rom/utf8.rs", "z")?;
    ensure!(result == 25, "gbc_main returned {result}");
    Ok(())
}

#[test]
fn unicode_properties_match_upstream() -> Result<()> {
    let (result, _) = run_core_rom("tests/core_rom/unicode_properties.rs", "z")?;
    ensure!(result == 3, "gbc_main returned {result}");
    Ok(())
}

#[test]
fn inlined_volatile_poll_rereads_the_register() -> Result<()> {
    let (result, serial) = run_core_rom("tests/core_rom/volatile_poll.rs", "2")?;
    ensure!(
        serial == b"ok\n",
        "serial {:?}",
        String::from_utf8_lossy(&serial)
    );
    ensure!(result == 7, "gbc_main returned {result}");
    Ok(())
}

fn build_runtime_rom(source: &str) -> Result<(Vec<u8>, String)> {
    build_mapped_rom(source, "0")
}

fn build_mapped_rom(source: &str, opt_level: &str) -> Result<(Vec<u8>, String)> {
    let directory = sangoku_test_support::tempfile::tempdir()?;
    let rom = directory.path().join("runtime.gb");
    let output = Command::new(env!("CARGO_BIN_EXE_sangoku"))
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .args(["build", source, "--opt-level", opt_level])
        .arg(&rom)
        .output()?;
    let report = String::from_utf8(output.stderr)?;
    ensure!(output.status.success(), "{report}");
    let artifacts = report
        .lines()
        .find_map(|line| line.strip_prefix("Build artifacts: "))
        .ok_or("build did not report its artifact directory")?;
    let map = fs::read_to_string(std::path::Path::new(artifacts).join("game.map"))?;
    Ok((fs::read(rom)?, map))
}

#[test]
fn empty_rom_excludes_unreferenced_arithmetic_runtime() -> Result<()> {
    let (_, map) = build_runtime_rom("benchmarks/empty.rs")?;
    for section in [
        "32-bit runtime",
        "Unsigned 16-bit division",
        "16-bit multiplication",
    ] {
        ensure!(
            !map.contains(&format!("[\"{section}\"]")),
            "retained {section}"
        );
    }
    ensure!(map.contains("__rust_gbc_main"), "missing entry point");
    ensure!(
        !map.contains("__gbc_"),
        "empty ROM retained a runtime helper"
    );
    Ok(())
}

#[test]
fn referenced_32_bit_runtime_links_and_executes() -> Result<()> {
    let (rom, map) = build_runtime_rom("tests/core_rom/runtime32.rs")?;
    for symbol in ["__gbc_udiv32", "__gbc_urem32", "__gbc_div32_core"] {
        ensure!(map.contains(symbol), "missing {symbol}");
    }
    for symbol in [
        "__gbc_sdiv32",
        "__gbc_srem32",
        "__gbc_mul32",
        "__gbc_shift8",
        "__gbc_popcnt32",
    ] {
        ensure!(!map.contains(symbol), "retained unrelated helper {symbol}");
    }
    for (dividend, divisor) in [(0x12345678u32, 12345u32), (u32::MAX, 65537), (0, 7)] {
        let mut emulator = Emulator::new(&rom, 0x100)?;
        emulator.memory[0xc000..0xc004].copy_from_slice(&dividend.to_le_bytes());
        emulator.memory[0xc004..0xc008].copy_from_slice(&divisor.to_le_bytes());
        let mut halted = false;
        for _ in 0..100_000 {
            let before = emulator.pc;
            emulator.step()?;
            if emulator.pc == before {
                halted = true;
                break;
            }
        }
        ensure!(halted, "32-bit division did not halt");
        let quotient = u32::from_le_bytes(emulator.memory[0xc008..0xc00c].try_into()?);
        let remainder = u32::from_le_bytes(emulator.memory[0xc00c..0xc010].try_into()?);
        ensure!(
            quotient == dividend / divisor,
            "wrong quotient for {dividend}/{divisor}"
        );
        ensure!(
            remainder == dividend % divisor,
            "wrong remainder for {dividend}/{divisor}"
        );
    }
    Ok(())
}

fn check_single_runtime_helper(
    name: &str,
    symbol: &str,
    output_bytes: usize,
    operation: fn(u32, u32) -> u32,
) -> Result<()> {
    let (rom, map) = build_runtime_rom(&format!("tests/core_rom/runtime_{name}.rs"))?;
    for helper in [
        "__gbc_mul32",
        "__gbc_div32_core",
        "__gbc_udiv32",
        "__gbc_urem32",
        "__gbc_sdiv32",
        "__gbc_srem32",
        "__gbc_popcnt32",
        "__gbc_clz32",
        "__gbc_ctz32",
        "__gbc_bitrev32",
        "__gbc_shift8",
        "__gbc_shift16",
    ] {
        ensure!(
            map.contains(helper) == (helper == symbol),
            "unexpected retention of {helper} for {name}"
        );
    }
    for (lhs, rhs) in [(0u32, 0u32), (0x80000001, 3), (u32::MAX, 7)] {
        let mut emulator = Emulator::new(&rom, 0x100)?;
        emulator.memory[0xc000..0xc004].copy_from_slice(&lhs.to_le_bytes());
        emulator.memory[0xc004..0xc008].copy_from_slice(&rhs.to_le_bytes());
        emulator.memory[0xc008..0xc00c].fill(0xa5);
        let mut halted = false;
        for _ in 0..100_000 {
            let before = emulator.pc;
            emulator.step()?;
            if emulator.pc == before {
                halted = true;
                break;
            }
        }
        ensure!(halted, "{name} did not halt");
        let mut bytes = [0u8; 4];
        bytes[..output_bytes].copy_from_slice(&emulator.memory[0xc008..0xc008 + output_bytes]);
        ensure!(
            u32::from_le_bytes(bytes) == operation(lhs, rhs),
            "wrong {name} result for {lhs}, {rhs}"
        );
    }
    Ok(())
}

#[test]
fn shift8_links_only_its_runtime_helper() -> Result<()> {
    check_single_runtime_helper("shift8", "__gbc_shift8", 1, |lhs, rhs| {
        (lhs as u8).wrapping_shl(rhs) as u32
    })
}

#[test]
fn popcnt32_links_only_its_runtime_helper() -> Result<()> {
    check_single_runtime_helper("popcnt32", "__gbc_popcnt32", 4, |lhs, _| lhs.count_ones())
}

#[test]
fn mul32_links_only_its_runtime_helper() -> Result<()> {
    check_single_runtime_helper("mul32", "__gbc_mul32", 4, u32::wrapping_mul)
}

#[test]
fn formatted_panic_traps_without_formatting_and_keeps_argument_effects() -> Result<()> {
    let (rom, map) = build_runtime_rom("tests/core_rom/panic_format.rs")?;
    ensure!(!map.contains("4core3fmt"), "panic retained formatting");
    for mode in 0..4 {
        let mut emulator = Emulator::new(&rom, 0x100)?;
        emulator.write16(0xc000, 12345);
        emulator.memory[0xc020] = mode;
        let mut halted = false;
        for _ in 0..100_000 {
            let before = emulator.pc;
            emulator.step()?;
            if emulator.pc == before {
                halted = true;
                break;
            }
        }
        ensure!(
            halted && emulator.pc == 0x11,
            "panic mode {mode} did not trap"
        );
        ensure!(
            emulator.memory[0xc010] == 0x5a,
            "panic argument effect was lost"
        );
    }
    Ok(())
}

#[test]
fn range_checks_trap_without_retaining_formatting() -> Result<()> {
    let (rom, map) = build_runtime_rom("tests/core_rom/range_check.rs")?;
    ensure!(
        !map.contains("4core3fmt"),
        "range check retained formatting"
    );
    for end in [2u8, 4] {
        let mut emulator = Emulator::new(&rom, 0x100)?;
        emulator.memory[0xc000] = end;
        let mut halted = false;
        for _ in 0..100_000 {
            let before = emulator.pc;
            emulator.step()?;
            if emulator.pc == before {
                halted = true;
                break;
            }
        }
        ensure!(halted, "range check did not halt");
        if end <= 3 {
            ensure!(
                emulator.a == end && emulator.pc != 0x11,
                "valid range failed"
            );
        } else {
            ensure!(emulator.pc == 0x11, "out-of-range access did not trap");
        }
    }
    Ok(())
}

#[test]
fn explicit_formatting_still_executes() -> Result<()> {
    let (rom, map) = build_mapped_rom("tests/core_rom/format_value.rs", "z")?;
    ensure!(map.contains("4core3fmt"), "explicit formatting was removed");
    for value in [0u8, 123, u8::MAX] {
        let mut emulator = Emulator::new(&rom, 0x100)?;
        emulator.memory[0xc000] = value;
        let mut halted = false;
        for _ in 0..2_000_000 {
            let before = emulator.pc;
            emulator.step()?;
            if emulator.pc == before {
                halted = true;
                break;
            }
        }
        ensure!(
            halted && emulator.a == 0,
            "explicit formatting did not return"
        );
        ensure!(
            emulator.serial == format!("value={value}\n").as_bytes(),
            "explicit formatting failed for {value}"
        );
    }
    Ok(())
}

#[test]
fn const_panics_remain_compile_errors() -> Result<()> {
    let output = Command::new(env!("CARGO_BIN_EXE_sangoku"))
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .args(["frontend", "tests/core_rom/const_panic.rs", "-"])
        .output()?;
    let report = String::from_utf8(output.stderr)?;
    ensure!(!output.status.success(), "constant panic was accepted");
    ensure!(
        report.contains("E0080") && report.contains("constant panic remains an error"),
        "{report}"
    );
    Ok(())
}
