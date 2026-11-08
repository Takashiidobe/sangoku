use std::collections::{HashMap, HashSet, VecDeque};

pub const ENTRY: &str = "__rust_gbc_main";

#[derive(Clone, Copy)]
pub struct Section<'a> {
    pub name: &'a str,
    pub text: &'a str,
}

pub fn split_sections(text: &str) -> Vec<Section<'_>> {
    let mut starts: Vec<usize> = text
        .match_indices("SECTION \"")
        .map(|(index, _)| index)
        .filter(|&index| index == 0 || text.as_bytes()[index - 1] == b'\n')
        .collect();
    starts.push(text.len());
    starts
        .windows(2)
        .map(|bounds| {
            let body = &text[bounds[0]..bounds[1]];
            let name = &body["SECTION \"".len()..];
            let name = &name[..name.find('"').unwrap_or(name.len())];
            Section { name, text: body }
        })
        .collect()
}

fn is_identifier_start(byte: u8) -> bool {
    byte.is_ascii_alphabetic() || byte == b'_'
}

fn is_identifier_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'$' | b'#' | b'@')
}

fn identifiers(text: &str) -> impl Iterator<Item = &str> {
    let bytes = text.as_bytes();
    let mut index = 0;
    std::iter::from_fn(move || {
        while index < bytes.len() {
            if is_identifier_start(bytes[index]) {
                let start = index;
                while index < bytes.len() && is_identifier_byte(bytes[index]) {
                    index += 1;
                }
                if start == 0 || bytes[start - 1] != b'.' {
                    return Some(&text[start..index]);
                }
                continue;
            }
            index += 1;
        }
        None
    })
}

fn exported_labels(text: &str) -> impl Iterator<Item = &str> {
    text.lines().filter_map(|line| {
        let end = line
            .bytes()
            .take_while(|&byte| is_identifier_byte(byte))
            .count();
        (end > 0 && line[end..].starts_with("::")).then_some(&line[..end])
    })
}

struct Dependencies<'a> {
    defined: HashMap<&'a str, usize>,
    edges: Vec<Vec<usize>>,
}

fn definitions<'a>(sections: &[Section<'a>]) -> HashMap<&'a str, usize> {
    let mut defined: HashMap<&str, usize> = HashMap::new();
    for (index, section) in sections.iter().enumerate() {
        for label in exported_labels(section.text) {
            defined.insert(label, index);
        }
    }
    defined
}

fn dependencies<'a>(sections: &[Section<'a>]) -> Dependencies<'a> {
    let defined = definitions(sections);
    let edges = sections
        .iter()
        .map(|section| {
            let mut targets: Vec<_> = identifiers(section.text)
                .filter_map(|token| defined.get(token).copied())
                .collect();
            targets.sort_unstable();
            targets.dedup();
            targets
        })
        .collect();
    Dependencies { defined, edges }
}

pub fn reachable<'a>(sections: &[Section<'a>], roots: &[&str]) -> Vec<usize> {
    let defined = definitions(sections);
    let mut kept = HashSet::new();
    let mut pending: Vec<usize> = roots
        .iter()
        .filter_map(|root| defined.get(root).copied())
        .collect();
    while let Some(index) = pending.pop() {
        if !kept.insert(index) {
            continue;
        }
        pending.extend(
            identifiers(sections[index].text)
                .filter_map(|token| defined.get(token).copied())
                .filter(|target| !kept.contains(target)),
        );
    }
    let mut kept: Vec<usize> = kept.into_iter().collect();
    kept.sort_unstable();
    kept
}

pub struct FormattingDependency {
    pub path: Vec<String>,
    pub bytes: usize,
}

fn symbol_name(symbol: &str) -> String {
    let symbol = symbol.strip_prefix("__rust_").unwrap_or(symbol);
    format!("{:#}", rustc_demangle::demangle(symbol))
}

fn brings_formatting(name: &str) -> bool {
    name.starts_with("core::fmt::write")
        || name.contains("core::fmt::Formatter")
        || name.contains("core::fmt::rt::Argument>::new_")
        || (name.contains(" as core::fmt::") && name.ends_with(">::fmt"))
}

pub fn formatting_dependency(
    sections: &[Section<'_>],
    sizes: &HashMap<String, usize>,
) -> Option<FormattingDependency> {
    let graph = dependencies(sections);
    let root = *graph.defined.get(ENTRY)?;
    let names: Vec<_> = sections.iter().map(|s| symbol_name(s.name)).collect();
    let mut previous = vec![None; sections.len()];
    let mut seen = vec![false; sections.len()];
    let mut queue = VecDeque::from([root]);
    seen[root] = true;
    let mut target = None;
    let mut bytes = 0;
    while let Some(index) = queue.pop_front() {
        if brings_formatting(&names[index]) {
            target.get_or_insert(index);
            bytes += sizes.get(sections[index].name).copied().unwrap_or(0);
        }
        for &next in &graph.edges[index] {
            if !seen[next] {
                seen[next] = true;
                previous[next] = Some(index);
                queue.push_back(next);
            }
        }
    }
    let mut index = target?;
    let mut path = vec![names[index].clone()];
    while let Some(parent) = previous[index] {
        path.push(names[parent].clone());
        index = parent;
    }
    path.reverse();
    Some(FormattingDependency { path, bytes })
}

pub fn parse_map(map: &str) -> (HashMap<String, usize>, usize) {
    let mut sizes = HashMap::new();
    let mut rom0_free = 0;
    for line in map.lines().map(str::trim) {
        if let Some(rest) = line.strip_prefix("SECTION: ")
            && let (Some(open), Some(close)) = (rest.find("($"), rest.find(" byte"))
            && let Ok(size) = usize::from_str_radix(&rest[open + 2..close], 16)
            && let (Some(first), Some(last)) = (rest.find("[\""), rest.rfind("\"]"))
        {
            sizes.insert(rest[first + 2..last].to_owned(), size);
        } else if let Some(rest) = line.strip_prefix("ROM0: ")
            && let Some((_, free)) = rest.split_once(" / ")
        {
            rom0_free = free
                .split_whitespace()
                .next()
                .and_then(|n| n.parse().ok())
                .unwrap_or(0);
        }
    }
    (sizes, rom0_free)
}

pub fn bank_one(text: &str) -> String {
    text.replace(", ROM0", ", ROMX")
}

pub fn largest_sections(sizes: &HashMap<String, usize>, limit: usize) -> String {
    let mut sections: Vec<_> = sizes.iter().collect();
    sections.sort_by(|(left_name, left_size), (right_name, right_size)| {
        right_size.cmp(left_size).then(left_name.cmp(right_name))
    });
    sections
        .into_iter()
        .take(limit)
        .map(|(name, size)| format!("{size} bytes: {}", symbol_name(name)))
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn place(sections: &[&Section], sizes: &HashMap<String, usize>, budget: usize) -> String {
    let mut used = 0;
    let mut output = String::new();
    for section in sections {
        let size = sizes.get(section.name).copied().unwrap_or(0);
        if used + size <= budget {
            used += size;
            output.push_str(section.text);
        } else {
            let (header, rest) = section.text.split_once('\n').unwrap_or((section.text, ""));
            output.push_str(&header.replacen(", ROM0", ", ROMX", 1));
            output.push('\n');
            output.push_str(rest);
        }
    }
    output
}

pub fn panic_symbol(assembly: &str) -> Option<&str> {
    identifiers(assembly)
        .find(|name| name.contains("rust_begin_unwind"))
        .filter(|name| !exported_labels(assembly).any(|label| label == *name))
}

pub fn runtime_objects(assembly: &str) -> Vec<&'static str> {
    let float32 = assembly
        .replace("__gbc_f64", "")
        .replace("__gbc_f128", "")
        .contains("__gbc_f");
    let memory = ["__gbc_mem", "memcpy", "memmove"]
        .iter()
        .any(|name| assembly.contains(name));
    let panic = panic_symbol(assembly).is_some();
    [
        ("udiv16", true),
        ("mul16", true),
        ("wide32", true),
        (
            "mulo128",
            assembly.contains("__gbc_u128_mulo") || assembly.contains("__gbc_i128_mulo"),
        ),
        ("softfloat32", float32),
        ("softfloat64", assembly.contains("__gbc_f64")),
        ("softfloat128", assembly.contains("__gbc_f128")),
        ("convert", assembly.contains("__gbc_convert")),
        ("memory", memory),
        ("panic", panic),
        (
            "heap",
            assembly.contains("___rust_alloc") || assembly.contains("__gbc_heap_init"),
        ),
    ]
    .into_iter()
    .filter_map(|(name, needed)| needed.then_some(name))
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const ASM: &str = "SECTION \"main\", ROM0\nmain::\n  call helper\n  ld hl, table\n\
SECTION \"helper\", ROM0\nhelper::\n.loop:\n  jr .loop\n\
SECTION \"table\", ROM0, ALIGN[0]\ntable::\ndw other\n\
SECTION \"other\", ROM0\nother::\n  ret\n\
SECTION \"unused\", ROM0\nunused::\n  call other\n";

    #[test]
    fn keeps_only_sections_reachable_from_the_roots() {
        let sections = split_sections(ASM);
        let names: Vec<_> = reachable(&sections, &["main"])
            .into_iter()
            .map(|i| sections[i].name)
            .collect();
        assert_eq!(names, ["main", "helper", "table", "other"]);
    }

    #[test]
    fn ignores_local_labels_that_share_a_global_name() {
        let text = "SECTION \"a\", ROM0\na::\n  jr .b\nSECTION \"b\", ROM0\nb::\n  ret\n";
        let sections = split_sections(text);
        assert_eq!(reachable(&sections, &["a"]), [0]);
    }

    #[test]
    fn reads_section_sizes_and_free_rom0_from_a_map() {
        let map = "ROM0: 1288 bytes used / 15096 free\n\tSECTION: $0000-$0008 ($0009 bytes) [\"Startup\"]\n\tSECTION: $0009-$0009 ($0001 byte) [\"data\"]\n";
        let (sizes, free) = parse_map(map);
        assert_eq!(sizes["Startup"], 9);
        assert_eq!(free, 15096);
        assert_eq!(sizes["data"], 1);
        assert_eq!(largest_sections(&sizes, 1), "9 bytes: Startup");
    }

    #[test]
    fn reports_formatting_through_data_references_and_ignores_unreachable_code() {
        let text = "SECTION \"entry\", ROM0\n__rust_gbc_main::\n call helper\n\
SECTION \"helper\", ROM0\nhelper::\n ld hl, table\n call __rust_gbc_main\n\
SECTION \"table\", ROM0\ntable::\n dw __rust__RNvNtC4core3fmt5write\n\
SECTION \"__rust__RNvNtC4core3fmt5write\", ROM0\n__rust__RNvNtC4core3fmt5write::\n ret\n\
SECTION \"__rust__RNvNtC4core3fmt9write_fmt\", ROM0\n__rust__RNvNtC4core3fmt9write_fmt::\n ret\n";
        let sections = split_sections(text);
        let sizes = HashMap::from([
            ("__rust__RNvNtC4core3fmt5write".into(), 12),
            ("__rust__RNvNtC4core3fmt9write_fmt".into(), 100),
        ]);
        let dependency = formatting_dependency(&sections, &sizes).unwrap();
        assert_eq!(dependency.bytes, 12);
        assert_eq!(
            dependency.path,
            ["entry", "helper", "table", "core::fmt::write"]
        );
    }

    #[test]
    fn literal_arguments_and_serial_write_trait_do_not_imply_general_formatting() {
        for name in [
            "<core::fmt::Arguments>::from_str",
            "<primes::Serial as core::fmt::Write>::write_str",
            "core::panicking::panic_fmt",
        ] {
            assert!(!brings_formatting(name), "{name}");
        }
        assert!(brings_formatting("<usize as core::fmt::Display>::fmt"));
        assert!(brings_formatting(
            "<core::fmt::rt::Argument>::new_display::<usize>"
        ));
        assert!(formatting_dependency(&split_sections(ASM), &HashMap::new()).is_none());
    }

    #[test]
    fn spills_sections_that_exceed_the_rom0_budget() {
        let sections = split_sections(ASM);
        let chosen: Vec<_> = sections.iter().take(2).collect();
        let sizes = HashMap::from([("main".into(), 6), ("helper".into(), 6)]);
        let placed = place(&chosen, &sizes, 8);
        assert!(placed.contains("SECTION \"main\", ROM0"));
        assert!(placed.contains("SECTION \"helper\", ROMX"));
        assert!(placed.contains("jr .loop"));
    }

    #[test]
    fn exported_runtime_entries_select_their_objects() {
        let directory = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("runtime");
        for entry in std::fs::read_dir(directory).unwrap() {
            let path = entry.unwrap().path();
            if path.extension().and_then(|s| s.to_str()) != Some("asm") {
                continue;
            }
            let object = path.file_stem().unwrap().to_str().unwrap();
            if object == "panic" {
                continue;
            }
            for line in std::fs::read_to_string(&path).unwrap().lines() {
                let Some(name) = line.strip_suffix("::") else {
                    continue;
                };
                if name.starts_with("__gbc_")
                    || name.starts_with("__rust___gbc_")
                    || ["memcpy", "memmove"].contains(&name)
                {
                    assert!(
                        runtime_objects(&format!("call {name}")).contains(&object),
                        "{name} did not select {object}"
                    );
                }
            }
        }
    }

    #[test]
    fn selects_runtime_objects_from_the_assembly() {
        assert_eq!(
            runtime_objects("call __gbc_f64_add"),
            ["udiv16", "mul16", "wide32", "softfloat64"]
        );
        assert!(runtime_objects("call memcpy").contains(&"memory"));
        assert!(runtime_objects("call __rust___gbc_u128_mulo").contains(&"mulo128"));
        assert!(!runtime_objects("ret").contains(&"mulo128"));
        assert!(runtime_objects("call __rust__Rx_rust_begin_unwind").contains(&"panic"));
        assert!(!runtime_objects("__rust_x_rust_begin_unwind::").contains(&"panic"));
        assert_eq!(
            panic_symbol("call __rust_Rx_rust_begin_unwind"),
            Some("__rust_Rx_rust_begin_unwind")
        );
    }
}
