mod support;

use std::fs;
use std::process::Command;

const BINARY: &str = env!("CARGO_BIN_EXE_firmetic-analyze");

#[test]
fn lists_a_section_id_for_every_header_in_header_order() {
    let dir = support::workspace("section-ids");
    let elf = support::linked_clean_elf(&dir);
    let output = Command::new(BINARY)
        .arg(&elf)
        .current_dir(&dir)
        .output()
        .expect("run firmetic-analyze");

    assert!(
        output.status.success(),
        "analyser failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let document = fs::read_to_string(dir.join("evidence.json")).expect("evidence.json");
    assert_eq!(ids(&document), expected_ids());
}

#[test]
fn reports_a_missing_argument() {
    let dir = support::workspace("missing-argument");
    let output = Command::new(BINARY)
        .current_dir(&dir)
        .output()
        .expect("run firmetic-analyze");

    assert_eq!(output.status.code(), Some(1));
    assert!(!output.stderr.is_empty(), "stderr should explain the usage");
    assert!(output.stdout.is_empty(), "stdout should be empty");
    assert!(
        !dir.join("evidence.json").exists(),
        "nothing should be written"
    );
}

#[test]
fn reports_an_unreadable_elf() {
    let dir = support::workspace("unreadable-elf");
    fs::write(dir.join("bad.elf"), b"not an ELF file").expect("write bad.elf");

    let output = Command::new(BINARY)
        .arg("bad.elf")
        .current_dir(&dir)
        .output()
        .expect("run firmetic-analyze");

    assert_eq!(output.status.code(), Some(1));
    assert!(
        !output.stderr.is_empty(),
        "stderr should explain the failure"
    );
    assert!(output.stdout.is_empty(), "stdout should be empty");
    assert!(
        !dir.join("evidence.json").exists(),
        "nothing should be written"
    );
}

fn expected_ids() -> Vec<String> {
    // Null, .text, .data, .symtab, .strtab, .shstrtab.
    (0..6).map(|index| format!("section:{index}")).collect()
}

fn ids(document: &str) -> Vec<String> {
    let value: serde_json::Value =
        serde_json::from_str(document).expect("evidence.json is valid JSON");

    value["sections"]
        .as_array()
        .expect("sections is a list")
        .iter()
        .map(|section| {
            section["id"]
                .as_str()
                .expect("every section has a string id")
                .to_owned()
        })
        .collect()
}
