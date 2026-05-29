pub(crate) struct SessionHeader {
    model: String,
}

impl SessionHeader {
    pub(crate) fn new(model: String) -> Self {
        Self { model }
    }

    /// Updates the header's model text.
    pub(crate) fn set_model(&mut self, model: REDACTED_SECRET) {
        if self.model != model {
            self.model = model.to_string();
        }
    }
}
