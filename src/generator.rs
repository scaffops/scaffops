//! Project generators.

use handlebars::{no_escape, Handlebars};

/// Represents project generators.
pub struct Generator<'g> {
    /// The renderer to use.
    pub renderer: Handlebars<'g>,
}

impl Generator<'_> {
    /// Constructs [`Self`].
    pub fn new() -> Self {
        let mut renderer = Handlebars::new();

        renderer.set_strict_mode(true);

        renderer.register_escape_fn(no_escape);

        Self { renderer }
    }
}
