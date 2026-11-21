use crate::section::Section;

pub struct Evidence {
    sections: Vec<Section>,
}

impl Evidence {
    pub fn new(sections: Vec<Section>) -> Self {
        Self { sections }
    }

    pub fn sections(&self) -> &[Section] {
        &self.sections
    }
}
