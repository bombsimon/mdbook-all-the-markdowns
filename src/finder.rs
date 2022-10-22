use inflector::Inflector;
use std::path::PathBuf;
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
    move |entry: &DirEntry| -> bool {
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
            && !is_hidden(entry)
    }
}

/// is_hidden returns true if the file is a hidden file. We use it to ignore these files when
/// looking for markdown files.
fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)
}

/// Find all markdown files by iterating from the `root` and store all folders and markdown files
/// in a list to determine what to render.
pub fn find_markdown_files(
    index_filenames: &[String],
    root: String,
    ignore: Vec<String>,
) -> Vec<MarkdownFile> {
    let mut filenames = vec![];

    for entry in WalkDir::new(&root)
        .follow_links(false)
        .into_iter()
        .filter_entry(ignore_matches(ignore))
        .filter_map(|e| e.ok())
    {
        // Skip directories, we'll resolve them once we found all markdown files.
        if entry.path().is_dir() {
            continue;
        }

        // Skip starting directory.
        if let Some(path) = entry.path().to_str() {
            if path == root {
                continue;
            }
        }

        // A file is found but it's not a markdown file, move on.
        if let Some(ex) = entry.path().extension() {
            if ex != "md" {
                continue;
            }
        } else {
            // Not even a file extension? Skip it!
            continue;
        }

        //  Add markdown file to filenames.
        filenames.push(entry.path().to_path_buf());
    }

    // We want to add all folders leading up to a markdown file but so we create a set of all
    // parent paths leading up to each markdown file. This way we can omit empty directories or
    // paths not leading to any markdown file. In the end we want to mark each section of a path as
    // a (sub) chapter.
    // F.ex. the file foo/bar/README.md should have section 1.1.1 where foo is 1 and
    // bar is 1.1.
    let mut parents = std::collections::HashSet::new();
    let mut folder_has_index_md = std::collections::HashSet::new();

    for path in filenames.iter() {
        let mut path_buf = PathBuf::new();

        for c in path.components() {
            path_buf = path_buf.join(c);

            // Don't add any paths until we've passed the root.
            if path_buf.to_string_lossy().len() <= root.len() {
                continue;
            }

            if let Some(ex) = path_buf.extension() {
                if ex == "md" {
                    if index_filenames
                        .iter()
                        .any(|filename| path_buf.ends_with(filename))
                    {
                        folder_has_index_md.insert(path.parent().unwrap().to_path_buf());
                    }

                    break;
                }
            }

            parents.insert(path_buf.clone());
        }
    }

    for parent in parents {
        if !folder_has_index_md.contains(&parent) {
            filenames.push(parent);
        }
    }

    // Sort the file names to get deterministic order of the index.
    filenames.sort_by(|a, b| {
        let a_parent = a.parent().unwrap();
        let b_parent = b.parent().unwrap();

        // If the paths are not the same use regular alphanumeric sorting.
        if a_parent != b_parent {
            return a.partial_cmp(b).unwrap();
        }

        // If one of them is the parent dir itself continue with regular sorting.
        if a.is_dir() || b.is_dir() {
            return a.partial_cmp(b).unwrap();
        }

        // If the paths are the same, ensure we sort indexes first so the section number is correct
        // even if there are files that would be sorted alphanumerically before. For example; if we
        // have two files /foo/README.md and /foo/INSTALLATION.md we want to sort README.md first
        // because it will automatically get assigned section number [1] where INSTALLATION.md
        // would be [1, 1].
        match a.file_name() {
            Some(v)
                if index_filenames
                    .iter()
                    .any(|filename| v == filename.as_str()) =>
            {
                std::cmp::Ordering::Less
            }
            _ => std::cmp::Ordering::Greater,
        }
    });

    let mut sections: Vec<String> = vec!["".into(); MAX_FOLDER_DEPTH];
    let mut section_ids = vec![0; MAX_FOLDER_DEPTH];
    let mut markdowns: Vec<MarkdownFile> = vec![];

    for path in filenames.iter() {
        let mut sections_for_file = 0;
        let is_folder = path.is_dir();
        let path_witout_prefix = match path.strip_prefix(&root) {
            Ok(v) => v,
            Err(_) => continue,
        };

        for (i, c) in path_witout_prefix.components().enumerate() {
            let section_name = c.as_os_str().to_str().unwrap();

            // If this is README.md, don't increment any IDs, treat this as the folder.
            if index_filenames
                .iter()
                .any(|filename| section_name == filename)
            {
                continue;
            }

            // If the section is new, increment the ID.
            if sections[i] != section_name || section_name.ends_with(".md") {
                sections[i] = section_name.to_string();
                section_ids[i] += 1;

                // If we update index i to a new ID we must reset whatever comes after to
                // restart counting at 0 and mark every sub folder as not seen.
                let reset_vec = vec![0; MAX_FOLDER_DEPTH - i];
                section_ids = section_ids[0..i + 1].to_vec();
                section_ids.extend(reset_vec);

                let reset_vec_sections = vec!["".into(); MAX_FOLDER_DEPTH - i];
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
            name: path.file_name().unwrap().to_string_lossy().to_string(),
            filename: path.as_os_str().to_str().unwrap().to_string(),
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
                .replace('-', " ")
                .replace('_', " ")
                .to_title_case();
            let content = format!("# {}", title);

            return Ok((title, content));
        }

        let contents = fs::read_to_string(self.filename.clone())?;
        let raw_title = contents
            .lines()
            .find(|l| !l.is_empty())
            .ok_or(std::io::ErrorKind::InvalidData)?;

        let re = regex::Regex::new(r"^#+\s*").unwrap();
        let title = re.replace_all(raw_title, "").to_string();

        Ok((title, contents))
    }
}
