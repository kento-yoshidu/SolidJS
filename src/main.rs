use std::fs;

fn convert_markdown_to_html(markdown: &str) -> String {
    let parser = pulldown_cmark::Parser::new(markdown);

    let mut buffer = String::new();

    pulldown_cmark::html::push_html(&mut buffer, parser);

    buffer
}

fn main() -> std::io::Result<()> {
    let content = fs::read_to_string("content/index.md")?;

    let res = convert_markdown_to_html(&content);

    let html = format!("<html><body>{res}</body></html>");

    fs::create_dir_all("dist")?;

    fs::write("dist/index.html", html)?;

    Ok(())
}
