use crate::config::Config;
use std::convert::TryInto;
use std::path::PathBuf;

use mdbook::book::{Book, BookItem, Chapter, SectionNumber};
use mdbook::errors::Error;
use mdbook::preprocess::{Preprocessor, PreprocessorContext};

pub mod config;
pub mod finder;

pub struct AllMarkdown;

impl AllMarkdown {
    pub fn new() -> AllMarkdown {
        AllMarkdown
    }
}

impl Default for AllMarkdown {
    fn default() -> AllMarkdown {
        AllMarkdown
    }
}

impl Preprocessor for AllMarkdown {
    fn name(&self) -> &str {
        "all-the-markdowns"
    }

    fn run(&self, ctx: &PreprocessorContext, book: Book) -> Result<Book, Error> {
        let cfg: Config = ctx.config.get_preprocessor(self.name()).try_into().unwrap();
        let mut b = book;

        for section in cfg.sections {
            if let Some(title) = section.title {
                b.push_item(BookItem::PartTitle(title));
            }

            finder::find_markdown_files(section.base, section.ignore)
                .iter()
                .for_each(|file| {
                    let (title, content) = file
                        .content()
                        .unwrap_or(("UNKNOWN".into(), "Could not get file content".into()));

                    let mut chapter = Chapter::new(
                        title.as_str(),
                        content,
                        PathBuf::from(file.filename.clone()),
                        vec![],
                    );

                    chapter.number = Some(SectionNumber(file.section.clone()));

                    b.push_item(BookItem::Chapter(chapter));
                });
        }

        Ok(b)
    }

    fn supports_renderer(&self, _renderer: &str) -> bool {
        true
    }
}
