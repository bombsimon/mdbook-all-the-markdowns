use inflector::Inflector;
use std::path::Path;
use std::{fs, io};
use walkdir::{DirEntry, WalkDir};

/// The max folder depth to support.
const MAX_FOLDER_DEPTH: usize = 24;

#[derive(Debug)]
pub struct MarkdownFile {
    pub name: String,
    pub filename: String,
    pub section: Vec<u32>,
    pub is_folder: bool,
}

/// ignore_matches will be used as a filter when walking down the folder structure by iterating
/// over all the directories added as ignore.
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

/// Find all markdown files by iterating from the `root` and store all folders and markdown files
/// in a list to determine what to render.
pub fn find_markdown_files(root: String, ignore: Vec<String>) -> Vec<MarkdownFile> {
    let mut filenames: Vec<String> = vec![];

    for entry in WalkDir::new(&root)
        .follow_links(false)
        .into_iter()
        .filter_entry(ignore_matches(ignore))
        .filter_map(|e| e.ok())
    {
        let path_name = entry.path().to_str().unwrap();

        // Skip starting directory.
        if path_name == root {
            continue;
        }

        // This is a directory, add it to paths to render properly. We want all the paths to be
        // numbered. F.ex. the file foo/bar/README.md should have section 1.1.1 where foo is 1 and
        // bar is 1.1.
        if entry.path().is_dir() {
            filenames.push(path_name.to_string());
            continue;
        }

        // A file is found but it's not a markdown file, move on.
        if !path_name.ends_with(".md") {
            continue;
        }

        //  Add markdown file to filenames.
        filenames.push(path_name.to_string());
    }

    // Sort the file names to get deterministic order of the index.
    filenames.sort_by(|a, b| a.partial_cmp(&b).unwrap());

    let mut sections: Vec<String> = vec!["".into(); MAX_FOLDER_DEPTH];
    let mut section_ids = vec![0; MAX_FOLDER_DEPTH];
    let mut markdowns: Vec<MarkdownFile> = vec![];

    for path in filenames.iter() {
        let mut sections_for_file = 0;
        let is_folder = !path.ends_with(".md");

        let trimmed = path.replace(&root, "");
        let p = Path::new(trimmed.as_str());

        for (i, c) in p.components().enumerate() {
            let section_name = c.as_os_str().to_str().unwrap();

            // If the section is new, increment the ID.
            if sections[i] != section_name || section_name.ends_with(".md") {
                sections[i] = section_name.to_string();
                section_ids[i] += 1;

                // If we update index i to a new ID we must reset whatever comes after to
                // restart counting at 0 and mark every sub folder as not seen.
                let reset_vec = vec![0; MAX_FOLDER_DEPTH - i];
                section_ids = section_ids[0..i + 1].to_vec();
                section_ids.extend(reset_vec);

                let reset_vec_sections = vec!["".into(); 24 - i];
                sections = sections[0..i + 1].to_vec();
                sections.extend(reset_vec_sections);
            }

            // Increment new sections seen to know how many sub sections to add for the current
            // file.
            sections_for_file += 1;
            if sections_for_file >= MAX_FOLDER_DEPTH {
                panic!("too deep folder structure - not supported!");
            }
        }

        markdowns.push(MarkdownFile {
            name: p.file_name().unwrap().to_string_lossy().to_string(),
            filename: path.to_string(),
            section: section_ids[0..sections_for_file].to_vec(),
            is_folder,
        });
    }

    markdowns
}

impl MarkdownFile {
    /// Get the title and the content from a markdown file. If the markdown file is actually a
    /// folder, title case the folder name after replacing `_` and `-` with a space.
    pub fn content(&self) -> io::Result<(String, String)> {
        if self.is_folder {
            // Seems good enough: https://stackoverflow.com/a/27086669/2274551
            let title = self
                .name
                .replace("-", " ")
                .replace("_", " ")
                .to_title_case();
            let content = format!("# {}", title.clone());

            return Ok((title, content));
        }

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
