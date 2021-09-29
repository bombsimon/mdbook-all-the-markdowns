use mdbook_all_the_markdowns::finder;

fn main() -> Result<(), &'static str> {
    println!("Finder example");

    let markdowns = finder::find_markdown_files(
        "./examples/".into(),
        vec!["./examples/book1".into(), "./examples/finder".into()],
    );

    println!("{:#?}", markdowns);

    Ok(())
}
