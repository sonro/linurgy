mod basic;
mod buffer;

pub use basic::EditorBuilder as BasicBuilder;

mod build_macro {
    macro_rules! impl_common_builder {
        () => {
            pub fn prepare(&mut self) {
                // avoid re-allocation
                self.replace.clear();

                match self.edit_type {
                    EditType::Append => {
                        self.replace.push_str(self.newline.as_str());
                        self.replace.push_str(self.text);
                    }
                    EditType::Insert => {
                        self.replace.push_str(self.text);
                        self.replace.push_str(self.newline.as_str());
                    }
                    EditType::Replace => self.replace.push_str(self.text),
                }

                // prevent re-allocation
                self.dirty = false;
            }

            pub fn newline_trigger(&mut self, trigger: u8) -> &mut Self {
                self.dirty = true;
                self.trigger = trigger;
                self
            }

            pub fn text(&mut self, text: &'a str) -> &mut Self {
                self.dirty = true;
                self.text = text;
                self
            }

            pub fn append(&mut self) -> &mut Self {
                self.dirty = true;
                self.edit_type = EditType::Append;
                self
            }

            pub fn insert(&mut self) -> &mut Self {
                self.dirty = true;
                self.edit_type = EditType::Insert;
                self
            }

            pub fn replace(&mut self) -> &mut Self {
                self.dirty = true;
                self.edit_type = EditType::Replace;
                self
            }

            pub fn newline_lf(&mut self) -> &mut Self {
                self.dirty = true;
                self.newline = NewlineType::Lf;
                self
            }

            pub fn newline_crlf(&mut self) -> &mut Self {
                self.dirty = true;
                self.newline = NewlineType::Crlf;
                self
            }
        };
    }

    pub(super) use impl_common_builder;
}
