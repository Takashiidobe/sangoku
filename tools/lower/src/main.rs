#[allow(dead_code)]
#[path = "../../../third_party/rustc_codegen_cranelift/src/rgbds.rs"]
mod rgbds;

use std::collections::HashMap;
use std::io::Write;
use std::{env, fs, process::ExitCode};

use cranelift_codegen::Context;
use cranelift_codegen::print_errors::pretty_error;
use cranelift_reader::{ParseOptions, parse_test};

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("sangoku-lower: {error}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), String> {
    let mut args = env::args().skip(1);
    let input = args
        .next()
        .ok_or("usage: sangoku-lower <file.clif> [output.s|-]")?;
    let output = args.next().unwrap_or_else(|| "-".into());
    let text =
        fs::read_to_string(&input).map_err(|error| format!("could not read {input}: {error}"))?;
    let asm = lower(&text, &input)?;
    if output == "-" {
        std::io::stdout()
            .write_all(asm.as_bytes())
            .map_err(|error| error.to_string())
    } else {
        fs::write(&output, asm).map_err(|error| format!("could not write {output}: {error}"))
    }
}

fn symbols_by_function_name(text: &str) -> HashMap<String, String> {
    let mut symbols = HashMap::new();
    let mut lines = text.lines();
    while let Some(line) = lines.next() {
        let Some(rest) = line.strip_prefix("function ") else {
            continue;
        };
        let name = rest.split('(').next().unwrap_or(rest).to_owned();
        if let Some(symbol) = lines.next().and_then(|next| next.strip_prefix("; symbol ")) {
            symbols.insert(name, symbol.trim().to_owned());
        }
    }
    symbols
}

fn without_repeated_headers(text: &str) -> String {
    let mut seen_function = false;
    let mut kept = String::with_capacity(text.len());
    for line in text.lines() {
        seen_function |= line.starts_with("function ");
        if seen_function && (line.starts_with("set ") || line.starts_with("target ")) {
            continue;
        }
        kept.push_str(line);
        kept.push('\n');
    }
    kept
}

fn lower(text: &str, input: &str) -> Result<String, String> {
    let text = &without_repeated_headers(text);
    let parsed = parse_test(text, ParseOptions::default()).map_err(|error| error.to_string())?;
    let isa = parsed
        .isa_spec
        .unique_isa()
        .ok_or("CLIF needs a single `target` line")?;
    let symbols = symbols_by_function_name(text);
    let symbol_of = |name: &str| {
        let name = name.strip_prefix('%').unwrap_or(name);
        symbols
            .get(name)
            .cloned()
            .unwrap_or_else(|| name.to_owned())
    };

    let stem = std::path::Path::new(input)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("clif");
    let mut source = rgbds::new_source(stem);
    let testcase_symbols: Vec<_> = parsed
        .functions
        .iter()
        .map(|(func, _)| func.name.to_string())
        .map(|name| {
            (
                format!("TestCase({name})"),
                rgbds::symbol_name(&symbol_of(&name)),
            )
        })
        .collect();
    for (func, _) in parsed.functions {
        let name = symbol_of(&func.name.to_string());
        let external: HashMap<String, String> = func
            .params
            .user_named_funcs()
            .iter()
            .map(|(user_ref, user)| {
                (
                    format!("User({user_ref:?})"),
                    rgbds::symbol_name(&symbol_of(&user.to_string())),
                )
            })
            .chain(testcase_symbols.iter().cloned())
            .chain(rgbds::runtime_libcalls())
            .collect();

        let mut context = Context::new();
        context.func = func;
        context.want_disasm = true;
        context
            .compile(&*isa, &mut Default::default())
            .map_err(|error| pretty_error(&error.func, error.inner))?;
        let vcode = context
            .compiled_code()
            .and_then(|code| code.vcode.as_deref())
            .ok_or("backend produced no VCode")?;
        rgbds::append_function(&mut source, &name, vcode, &external);
    }
    Ok(source)
}
