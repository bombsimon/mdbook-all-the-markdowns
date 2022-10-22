use mdbook::errors::Error;
use std::convert::TryFrom;
use toml::value::Table;

#[derive(Debug)]
pub struct Section {
    pub title: Option<String>,
    pub base: String,
    pub ignore: Vec<String>,
}

#[derive(Debug)]
pub struct Config {
    pub sections: Vec<Section>,
    pub draft_folders: bool,
    pub index_filenames: Vec<String>,
}

impl Default for Section {
    fn default() -> Section {
        Section {
            title: None,
            base: "./".into(),
            ignore: vec![],
        }
    }
}

impl Default for Config {
    fn default() -> Config {
        Config {
            sections: vec![],
            draft_folders: false,
            index_filenames: vec!["README.md".to_string()],
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

        if let Some(df) = mdbook_cfg.get("draft-folders") {
            cfg.draft_folders = df.as_bool().unwrap_or(false);
        }

        if let Some(indexes) = mdbook_cfg.get("index-names") {
            let mut filenames: Vec<String> = vec![];

            if let Some(indexes) = indexes.as_array() {
                for index in indexes {
                    if let Some(i) = index.as_str() {
                        filenames.push(i.into())
                    }
                }
            }

            cfg.index_filenames = filenames;
        }

        if let Some(sections) = mdbook_cfg.get("section") {
            let section_array = sections.as_array().unwrap();

            for section in section_array {
                let mut s = Section::default();

                if let Some(title) = section.get("title") {
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

                    s.title = title;
                }

                if let Some(base) = section.get("base") {
                    let base = match base.as_str() {
                        Some(m) => m,
                        None => {
                            return Err(Error::msg(format!(
                                "'base' {:?} is not a valid string",
                                base
                            )))
                        }
                    };

                    s.base = base.into();
                }

                if let Some(ignore) = section.get("ignore") {
                    let mut ignore_list: Vec<String> = vec![];

                    if let Some(ignore_array) = ignore.as_array() {
                        for path in ignore_array {
                            if let Some(p) = path.as_str() {
                                ignore_list.push(p.into())
                            }
                        }
                    }

                    s.ignore = ignore_list;
                }

                cfg.sections.push(s);
            }
        }

        log::info!("PARSED CONFIG: {:#?}", cfg);

        Ok(cfg)
    }
}
