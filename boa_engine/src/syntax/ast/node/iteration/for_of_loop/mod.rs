use crate::syntax::ast::node::{iteration::IterableLoopInitializer, Node};
use boa_interner::{Interner, Sym, ToInternedString};

#[cfg(feature = "deser")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "deser", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct ForOfLoop {
    init: Box<IterableLoopInitializer>,
    iterable: Box<Node>,
    body: Box<Node>,
    label: Option<Sym>,
    r#await: bool,
}

impl ForOfLoop {
    /// Creates a new "for of" loop AST node.
    pub fn new<I, B>(init: IterableLoopInitializer, iterable: I, body: B, r#await: bool) -> Self
    where
        I: Into<Node>,
        B: Into<Node>,
    {
        Self {
            init: Box::new(init),
            iterable: Box::new(iterable.into()),
            body: Box::new(body.into()),
            label: None,
            r#await,
        }
    }

    pub fn init(&self) -> &IterableLoopInitializer {
        &self.init
    }

    pub fn iterable(&self) -> &Node {
        &self.iterable
    }

    pub fn body(&self) -> &Node {
        &self.body
    }

    pub fn label(&self) -> Option<Sym> {
        self.label
    }

    pub fn set_label(&mut self, label: Sym) {
        self.label = Some(label);
    }

    /// Returns true if this "for...of" loop is an "for await...of" loop.
    pub(crate) fn r#await(&self) -> bool {
        self.r#await
    }

    /// Converts the "for of" loop to a string with the given indentation.
    pub(in crate::syntax::ast::node) fn to_indented_string(
        &self,
        interner: &Interner,
        indentation: usize,
    ) -> String {
        let mut buf = if let Some(label) = self.label {
            format!("{}: ", interner.resolve_expect(label))
        } else {
            String::new()
        };
        buf.push_str(&format!(
            "for ({} of {}) {}",
            self.init.to_interned_string(interner),
            self.iterable.to_interned_string(interner),
            self.body().to_indented_string(interner, indentation)
        ));

        buf
    }
}

impl ToInternedString for ForOfLoop {
    fn to_interned_string(&self, interner: &Interner) -> String {
        self.to_indented_string(interner, 0)
    }
}

impl From<ForOfLoop> for Node {
    fn from(for_of: ForOfLoop) -> Self {
        Self::ForOfLoop(for_of)
    }
}
