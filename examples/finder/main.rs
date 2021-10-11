use mdbook_all_the_markdowns::finder;

fn main() -> Result<(), &'static str> {
    println!("Finder example");

    let markdowns =
        finder::find_markdown_files("./examples/example-folder-structures/slim/".into(), vec![]);

    println!("{:#?}", markdowns);

    Ok(())
}
