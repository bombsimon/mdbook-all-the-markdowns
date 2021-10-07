use mdbook::errors::Error;
use std::convert::TryFrom;
use toml::value::Table;

#[derive(Debug)]
pub struct Config {
    pub title: Option<String>,
    pub base: String,
    pub ignore: Vec<String>,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            title: None,
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

        if let Some(title) = mdbook_cfg.get("section_title") {
            let title = match title.as_str() {
                Some(m) => {
                    if !m.is_empty() {
                        Some(m.to_string())
                    } else {
                        None
                    }
                }
                None => {
                    return Err(Error::msg(format!(
                        "'title' {:?} is not a valid string",
                        title
                    )))
                }
            };

            cfg.title = title;
        }

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
