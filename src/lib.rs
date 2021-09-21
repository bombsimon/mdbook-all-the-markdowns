use mdbook::book::Book;
use mdbook::errors::Error;
use mdbook::preprocess::{Preprocessor, PreprocessorContext};

pub struct AllMarkdown;

impl AllMarkdown {
    pub fn new() -> AllMarkdown {
        AllMarkdown
    }
}

impl Preprocessor for AllMarkdown {
    fn name(&self) -> &str {
        "render-all-markdown"
    }

    fn run(&self, ctx: &PreprocessorContext, book: Book) -> Result<Book, Error> {
        let mut b = book.clone();

        let mut c1 = mdbook::book::Chapter::new(
            "Some chapter",
            "Hello, World".into(),
            std::path::PathBuf::from("chapter_1.md"),
            vec![],
        );

        c1.number = Some(mdbook::book::SectionNumber([1, 1].into()));

        let mut c2 = mdbook::book::Chapter::new(
            "Some chapter AGAIN",
            "{{#include SUMMARY.md}}".into(),
            std::path::PathBuf::from("chapter_2.md"),
            vec![],
        );

        c2.number = Some(mdbook::book::SectionNumber([1, 2].into()));

        let mut c3 = mdbook::book::Chapter::new(
            "Some chapter three",
            "Foo me".into(),
            std::path::PathBuf::from("sub/chapter_3.md"),
            vec!["sub".into()],
        );

        c3.number = Some(mdbook::book::SectionNumber([1, 2, 1].into()));

        b.push_item(mdbook::book::BookItem::Chapter(c1));
        b.push_item(mdbook::book::BookItem::Chapter(c2));
        b.push_item(mdbook::book::BookItem::Chapter(c3));

        // In testing we want to tell the preprocessor to blow up by setting a
        // particular config value
        if let Some(nop_cfg) = ctx.config.get_preprocessor(self.name()) {
            if nop_cfg.contains_key("blow-up") {
                anyhow::bail!("Boom!!1!");
            }
        }

        Ok(b)
    }

    fn supports_renderer(&self, _renderer: &str) -> bool {
        true
    }
}
