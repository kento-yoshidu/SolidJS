use std::fs;

fn convert_markdown_to_html(markdown: &str) -> String {
    let mut options = pulldown_cmark::Options::empty();
    options.insert(pulldown_cmark::Options::ENABLE_TABLES);

    let parser = pulldown_cmark::Parser::new_ext(markdown, options);

    let mut buffer = String::new();

    pulldown_cmark::html::push_html(&mut buffer, parser);

    buffer
}

fn main() -> std::io::Result<()> {
    let content = fs::read_to_string("content/index.md")?;

    let res = convert_markdown_to_html(&content);

    let html = format!(
        "<html><head><meta charset=\"utf-8\"><link rel=\"preconnect\" href=\"https://fonts.googleapis.com\"><link rel=\"preconnect\" href=\"https://fonts.gstatic.com\" crossorigin><link href=\"https://fonts.googleapis.com/css2?family=Noto+Sans+JP:wght@400;700&family=Roboto:wght@400;700&display=swap\" rel=\"stylesheet\"><link rel=\"stylesheet\" href=\"style.css\"></head><body>{res}</body></html>"
    );

    fs::create_dir_all("dist")?;

    fs::write("dist/index.html", html)?;
    fs::copy("static/style.css", "dist/style.css")?;

    Ok(())
}
