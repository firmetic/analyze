use firmetic_analyze::evidence::Evidence;
use firmetic_analyze::section::Section;

#[test]
fn holds_the_sections() {
    let evidence = Evidence::new(vec![Section::new(0)]);

    assert_eq!(evidence.sections()[0].id(), "section:0");
}
