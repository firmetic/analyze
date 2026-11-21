pub struct Section {
    id: String,
}

impl Section {
    pub fn new(index: u32) -> Self {
        Self {
            id: format!("section:{index}"),
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }
}
