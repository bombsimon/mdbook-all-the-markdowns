use std::convert::{TryFrom, TryInto};
use std::path::PathBuf;

use mdbook::book::{Book, BookItem, Chapter, SectionNumber};
use mdbook::errors::Error;
use mdbook::preprocess::{Preprocessor, PreprocessorContext};

use toml::value::Table;

pub mod finder;

#[derive(Debug)]
pub struct Config {
    pub base: String,
    pub ignore: Vec<String>,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            base: "./".into(),
            ignore: vec![],
        }
    }
}

impl<'a> TryFrom<Option<&'a Table>> for Config {
    type Error = Error;

    fn try_from(mdbook_cfg: Option<&Table>) -> Result<Config, Error> {
        let mut cfg = Config::default();
        let mdbook_cfg = match mdbook_cfg {
            Some(c) => c,
            None => return Ok(cfg),
        };

        if let Some(base) = mdbook_cfg.get("base") {
            let base = match base.as_str() {
                Some(m) => m,
                None => {
                    return Err(Error::msg(format!(
                        "'base' {:?} is not a valid string",
                        base
                    )))
                }
            };

            cfg.base = base.into();
        }

        if let Some(ignore) = mdbook_cfg.get("ignore") {
            let mut ignore_list: Vec<String> = vec![];

            if let Some(ignore_array) = ignore.as_array() {
                for path in ignore_array {
                    if let Some(p) = path.as_str() {
                        ignore_list.push(p.into())
                    }
                }
            }

            cfg.ignore = ignore_list;
        }

        Ok(cfg)
    }
}

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
        let cfg: Config = ctx.config.get_preprocessor(self.name()).try_into().unwrap();

        let mut b = book.clone();
        b.push_item(BookItem::PartTitle("Auto generated".into()));

        finder::find_markdown_files(cfg.base, cfg.ignore)
            .iter()
            .for_each(|file| {
                let (title, content) = file.content().unwrap();

                let mut chapter = Chapter::new(
                    title.as_str(),
                    content,
                    PathBuf::from(file.filename.clone()),
                    vec![],
                );

                chapter.number = Some(SectionNumber(file.section.clone().into()));

                b.push_item(BookItem::Chapter(chapter));
            });

        Ok(b)
    }

    fn supports_renderer(&self, _renderer: &str) -> bool {
        true
    }
}
