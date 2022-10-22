use mdbook_all_the_markdowns::finder;

fn main() -> Result<(), &'static str> {
    println!("Finder example");

    let index_filenames = vec!["README.md".into()];

    let markdowns = finder::find_markdown_files(
        &index_filenames,
        "./examples/example-folder-structures/slim/".into(),
        vec![],
    );

    println!("{:#?}", markdowns);

    Ok(())
}
