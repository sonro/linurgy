mod basic;
mod buffer;

pub use basic::EditorBuilder as BasicBuilder;

mod build_macro {
    macro_rules! impl_common_builder {
        () => {
            /// Prepare the builder after updating values.
            ///
            /// This only needs to be called once, when all values have been
            /// set. After preparation, the builder can be used to create
            /// [`BasicEditor`] instances with the
            /// [`build_prepared`](Self::build_prepared) method, which only
            /// needs a shared reference.
            #[inline]
            pub fn prepare(&mut self) -> &mut Self {
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

                // prevent re-allocation on build method
                self.dirty = false;

                self
            }

            /// Set the number of newlines to trigger edits.
            ///
            /// By default, this is `0`, causing no editing to be triggered.
            #[inline]
            pub fn newline_trigger(&mut self, trigger: u8) -> &mut Self {
                self.dirty = true;
                self.trigger = trigger;
                self
            }

            /// Set the text used when editing.
            ///
            /// By default, this is an empty string: `""`.
            ///
            /// If the edit type is set to [`append`](Self::append), the
            /// text will be added *after* the newlines. If the edit
            /// type is set to [`insert`](Self::insert), the text will be
            /// inserted *before* the newlines. If the edit type is left
            /// as the default: [`replace`](Self::replace), matching
            /// newlines will be *replaced* with the text.
            #[inline]
            pub fn text(&mut self, text: &'a str) -> &mut Self {
                self.dirty = true;
                self.text = text;
                self
            }

            /// Set the editor to append the edit text *after* newlines.
            #[inline]
            pub fn append(&mut self) -> &mut Self {
                self.dirty = true;
                self.edit_type = EditType::Append;
                self
            }

            /// Set the editor to insert the edit text *before* newlines.
            #[inline]
            pub fn insert(&mut self) -> &mut Self {
                self.dirty = true;
                self.edit_type = EditType::Insert;
                self
            }

            /// Set the editor to replace newlines with the edit text.
            ///
            /// This is the default.
            #[inline]
            pub fn replace(&mut self) -> &mut Self {
                self.dirty = true;
                self.edit_type = EditType::Replace;
                self
            }

            /// Set the editor to use `\n` as the line ending.
            ///
            /// This is the default.
            #[inline]
            pub fn newline_lf(&mut self) -> &mut Self {
                self.dirty = true;
                self.newline = NewlineType::Lf;
                self
            }

            /// Set the editor to use `\r\n` as the line ending.
            #[inline]
            pub fn newline_crlf(&mut self) -> &mut Self {
                self.dirty = true;
                self.newline = NewlineType::Crlf;
                self
            }
        };
    }

    pub(super) use impl_common_builder;
}
