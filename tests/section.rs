use firmetic_analyze::section::Section;

#[test]
fn derives_the_id_from_the_index() {
    let cases = [
        (0, "section:0"),
        (42, "section:42"),
        (4_294_967_294, "section:4294967294"),
    ];

    for (index, expected) in cases {
        assert_eq!(Section::new(index).id(), expected);
    }
}
