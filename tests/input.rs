mod support;

use firmetic_analyze::input::load;
use support::Elf32;

#[test]
fn lists_a_section_id_for_every_header() {
    let cases = [
        ("no table", Elf32::headerless().into_bytes(), 0u32),
        ("minimum", Elf32::with_headers(1).into_bytes(), 1),
        ("typical", Elf32::with_names(6).into_bytes(), 6),
        ("maximum", Elf32::with_headers(0xfeff).into_bytes(), 0xfeff),
        (
            "first extended",
            Elf32::extended(0xff00).into_bytes(),
            0xff00,
        ),
    ];

    for (label, bytes, headers) in cases {
        let expected: Vec<String> = (0..headers)
            .map(|index| format!("section:{index}"))
            .collect();

        assert_eq!(ids(&load(&bytes).expect(label)), expected, "{label}");
    }
}

#[test]
fn rejects_unparseable_input() {
    let cases = [
        ("empty", Vec::new()),
        ("shorter than a header", b"\x7fELF".to_vec()),
        ("not an elf", b"not an ELF file".to_vec()),
        ("truncated section table", {
            let mut bytes = Elf32::with_headers(1).into_bytes();
            bytes.truncate(Elf32::HEADER_SIZE + Elf32::SECTION_SIZE - 1);
            bytes
        }),
    ];

    for (label, bytes) in cases {
        assert!(load(&bytes).is_err(), "{label}");
    }
}

#[test]
fn reads_a_big_endian_elf() {
    let evidence = load(&Elf32::with_headers_big_endian(1).into_bytes()).expect("big-endian");

    assert_eq!(ids(&evidence), ["section:0"]);
}

#[test]
fn accepts_a_claimed_count_without_a_table() {
    let cases = [("one", 1u16), ("many", 0xfeff)];

    for (label, count) in cases {
        let evidence = load(&Elf32::with_missing_table(count).into_bytes()).expect(label);

        assert!(ids(&evidence).is_empty(), "{label}");
    }
}

#[test]
fn accepts_a_reserved_literal_count() {
    let cases = [("first reserved", 0xff00u16), ("last reserved", 0xffff)];

    for (label, count) in cases {
        let evidence = load(&Elf32::with_reserved_count(count).into_bytes()).expect(label);

        assert_eq!(ids(&evidence).len(), usize::from(count), "{label}");
    }
}

fn ids(evidence: &firmetic_analyze::evidence::Evidence) -> Vec<String> {
    evidence
        .sections()
        .iter()
        .map(|section| section.id().to_owned())
        .collect()
}
