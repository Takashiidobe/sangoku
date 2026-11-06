use cranelift_codegen::{
    cursor::{Cursor, FuncCursor},
    dominator_tree::DominatorTree,
    flowgraph::ControlFlowGraph,
    ir::{AbiParam, Function, InstBuilder, Signature, UserFuncName, types},
    isa::{self, CallConv},
    settings::{self, Configurable},
};
use cranelift_control::ControlPlane;
use serde_json::json;
use std::{fs, path::PathBuf};
fn main() {
    let directory = PathBuf::from(std::env::args().nth(1).expect("artifact directory"));
    fs::create_dir_all(&directory).unwrap();
    let full = std::env::args().any(|arg| arg == "--full");
    let mut rows = vec![];
    for width in [16u32, 32] {
        for op in ["shl", "shr", "sar", "rol", "ror"] {
            let counts: Vec<_> = if full {
                (0..width).chain([width, width + 1, 255]).collect()
            } else {
                vec![0, 1, 8, width - 1, width + 1]
            };
            for count in counts.into_iter().map(Some).chain([None]) {
                for live in [false, true] {
                    if live
                        && count
                            .is_some_and(|n| ![1, 8, width - 1, width, width + 1, 255].contains(&n))
                    {
                        continue;
                    }
                    let name = format!(
                        "{op}{width}-{}-{}",
                        count.map_or("var".into(), |n| n.to_string()),
                        if live { "live" } else { "plain" }
                    );
                    let ty = if width == 16 { types::I16 } else { types::I32 };
                    let mut sig = Signature::new(CallConv::Fast);
                    sig.params.push(AbiParam::new(ty));
                    if count.is_none() {
                        sig.params.push(AbiParam::new(types::I8));
                    }
                    sig.returns.push(AbiParam::new(ty));
                    let mut f = Function::with_name_signature(UserFuncName::user(0, 0), sig);
                    let block = f.dfg.make_block();
                    let value = f.dfg.append_block_param(block, ty);
                    let amount = count
                        .is_none()
                        .then(|| f.dfg.append_block_param(block, types::I8));
                    let mut c = FuncCursor::new(&mut f);
                    c.insert_block(block);
                    let n =
                        amount.unwrap_or_else(|| c.ins().iconst(types::I8, count.unwrap() as i64));
                    let shifted = match op {
                        "shl" => c.ins().ishl(value, n),
                        "shr" => c.ins().ushr(value, n),
                        "sar" => c.ins().sshr(value, n),
                        "rol" => c.ins().rotl(value, n),
                        _ => c.ins().rotr(value, n),
                    };
                    let result = if live {
                        c.ins().iadd(value, shifted)
                    } else {
                        shifted
                    };
                    c.ins().return_(&[result]);
                    drop(c);
                    fs::write(directory.join(format!("{name}.clif")), f.to_string()).unwrap();
                    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        let mut flags = settings::builder();
                        flags.set("opt_level", "speed_and_size").unwrap();
                        flags.set("regalloc_checker", "true").unwrap();
                        let backend = isa::lookup(
                            "z80-unknown-none"
                                .parse::<target_lexicon::Triple>()
                                .unwrap(),
                        )
                        .unwrap()
                        .finish(settings::Flags::new(flags))
                        .unwrap();
                        let cfg = ControlFlowGraph::with_function(&f);
                        let dom = DominatorTree::with_function(&f, &cfg);
                        backend
                            .compile_function(
                                &f,
                                &dom,
                                &mut regalloc2::Ctx::default(),
                                true,
                                &mut ControlPlane::default(),
                            )
                            .map_err(|e| e.to_string())
                    }));
                    let mut row =
                        json!({"name":name,"operation":op,"width":width,"count":count,"live":live});
                    match result {
                        Ok(Ok(code)) => {
                            let code = code.apply_params(&f.params);
                            fs::write(directory.join(format!("{name}.bin")), code.buffer.data())
                                .unwrap();
                            fs::write(
                                directory.join(format!("{name}.vcode")),
                                code.vcode.as_deref().unwrap_or(""),
                            )
                            .unwrap();
                            row["status"] = json!("compiled");
                            row["code_bytes"] = json!(code.buffer.data().len());
                        }
                        Ok(Err(e)) => {
                            row["status"] = json!("unsupported");
                            row["error"] = json!(e);
                        }
                        Err(_) => {
                            row["status"] = json!("panic");
                        }
                    }
                    rows.push(row);
                }
            }
        }
    }
    fs::write(
        directory.join("results.json"),
        serde_json::to_string_pretty(&rows).unwrap(),
    )
    .unwrap();
}
