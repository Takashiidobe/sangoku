use std::collections::{HashMap, HashSet};

pub const ENTRY: &str = "__rust_gbc_main";

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

pub fn reachable<'a>(sections: &[Section<'a>], roots: &[&str]) -> Vec<usize> {
    let mut defined: HashMap<&str, usize> = HashMap::new();
    for (index, section) in sections.iter().enumerate() {
        for label in exported_labels(section.text) {
            defined.insert(label, index);
        }
    }
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

pub fn parse_map(map: &str) -> (HashMap<String, usize>, usize) {
    let mut sizes = HashMap::new();
    let mut rom0_free = 0;
    for line in map.lines().map(str::trim) {
        if let Some(rest) = line.strip_prefix("SECTION: ")
            && let (Some(open), Some(close)) = (rest.find("($"), rest.find(" bytes)"))
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

pub fn runtime_objects(assembly: &str) -> Vec<&'static str> {
    let float32 = assembly
        .replace("__gbc_f64", "")
        .replace("__gbc_f128", "")
        .contains("__gbc_f");
    let memory = ["__gbc_mem", "memcpy", "memmove"]
        .iter()
        .any(|name| assembly.contains(name));
    let panic = assembly.contains("rust_begin_unwind") && !assembly.contains("rust_begin_unwind::");
    [
        ("udiv16", true),
        ("mul16", true),
        ("wide32", true),
        ("softfloat32", float32),
        ("softfloat64", assembly.contains("__gbc_f64")),
        ("softfloat128", assembly.contains("__gbc_f128")),
        ("convert", assembly.contains("__gbc_convert")),
        ("memory", memory),
        ("panic", panic),
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
        let map = "ROM0: 1288 bytes used / 15096 free\n\tSECTION: $0000-$0008 ($0009 bytes) [\"Startup\"]\n";
        let (sizes, free) = parse_map(map);
        assert_eq!(sizes["Startup"], 9);
        assert_eq!(free, 15096);
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
    fn selects_runtime_objects_from_the_assembly() {
        assert_eq!(
            runtime_objects("call __gbc_f64_add"),
            ["udiv16", "mul16", "wide32", "softfloat64"]
        );
        assert!(runtime_objects("call memcpy").contains(&"memory"));
        assert!(runtime_objects("call __rust__Rx_rust_begin_unwind").contains(&"panic"));
        assert!(!runtime_objects("__rust_x_rust_begin_unwind::").contains(&"panic"));
    }
}
