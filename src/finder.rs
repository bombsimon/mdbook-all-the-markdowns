use std::{fs, io};
use walkdir::{DirEntry, WalkDir};

#[derive(Clone, Debug)]
pub struct MarkdownFile {
    pub filename: String,
    pub section: Vec<u32>,
}

fn ignore_matches(ignore_patterns: Vec<String>) -> impl Fn(&DirEntry) -> bool {
    let ignore = move |entry: &DirEntry| -> bool {
        !entry
            .path()
            .to_str()
            .map(|s| {
                for i in &ignore_patterns {
                    if s.starts_with(i.as_str()) {
                        return true;
                    }
                }

                false
            })
            .unwrap_or(false)
    };

    ignore
}

pub fn find_markdown_files(root: String, ignore: Vec<String>) -> Vec<MarkdownFile> {
    let mut filenames: Vec<String> = vec![];

    for entry in WalkDir::new(&root)
        .follow_links(true)
        .into_iter()
        .filter_entry(ignore_matches(ignore))
        .filter_map(|e| e.ok())
    {
        let f_name = entry.file_name().to_string_lossy();
        if !f_name.ends_with(".md") {
            continue;
        }

        let path: String = entry.path().to_str().unwrap().into();

        filenames.push(path);
    }

    // Sort the file names to get deterministic order of the index.
    filenames.sort();

    let mut seen = 0;
    filenames
        .iter()
        .map(|path| {
            seen += 1;
            let v = if seen < 2 { vec![1] } else { vec![1, 1] };

            MarkdownFile {
                filename: path.to_string(),
                section: v,
            }
        })
        .collect()
}

impl MarkdownFile {
    pub fn content(&self) -> io::Result<(String, String)> {
        let contents = fs::read_to_string(self.filename.clone())?;
        let title = contents
            .lines()
            .next()
            .ok_or(std::io::Error::new(
                std::io::ErrorKind::Other,
                "could not get header",
            ))?
            .replace("# ", "");

        Ok((title.into(), contents))
    }
}
