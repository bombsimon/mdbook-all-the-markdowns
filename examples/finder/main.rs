use mdbook_render_all_markdown::finder;

fn main() -> Result<(), &'static str> {
    println!("Finder example");

    finder::find_markdown_files("./examples".into(), vec!["./examples/book1".into()]);

    Ok(())
}
