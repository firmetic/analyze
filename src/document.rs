use serde::Serialize;

use crate::evidence::Evidence;

#[derive(Serialize)]
struct Document<'a> {
    sections: Vec<SectionView<'a>>,
}

impl<'a> Document<'a> {
    fn new(evidence: &'a Evidence) -> Self {
        Self {
            sections: evidence.sections().iter().map(SectionView::new).collect(),
        }
    }
}

#[derive(Serialize)]
struct SectionView<'a> {
    id: &'a str,
}

impl<'a> SectionView<'a> {
    fn new(section: &'a crate::section::Section) -> Self {
        Self { id: section.id() }
    }
}

pub fn render(evidence: &Evidence) -> String {
    let document = Document::new(evidence);

    let mut text = serde_json::to_string(&document).expect("a valid document serializes");
    text.push('\n');
    text
}
