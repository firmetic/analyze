use firmetic_analyze::document::render;
use firmetic_analyze::evidence::Evidence;
use firmetic_analyze::section::Section;

#[test]
fn renders_the_section_ids() {
    let cases = [("minimum", 0), ("typical", 2), ("maximum", 65_279)];

    for (label, count) in cases {
        let ids = ids(&render(&evidence(count)));
        let expected: Vec<String> = (0..count).map(|index| format!("section:{index}")).collect();

        assert_eq!(ids, expected, "{label}");
    }
}

fn evidence(count: u32) -> Evidence {
    Evidence::new((0..count).map(Section::new).collect())
}

fn ids(document: &str) -> Vec<String> {
    let value: serde_json::Value = serde_json::from_str(document).expect("valid JSON");

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
