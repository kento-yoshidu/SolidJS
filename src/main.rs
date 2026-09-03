use std::{fmt::format, fs};

use pulldown_cmark::{CowStr, Event, HeadingLevel, Tag, TagEnd};

struct HeadingInfo {
    level: HeadingLevel,
    id: String,
    text: String,
}

fn convert_markdown_to_html(markdown: &str) -> String {
    let mut options = pulldown_cmark::Options::empty();
    options.insert(pulldown_cmark::Options::ENABLE_TABLES);

    let mut events: Vec<Event> = pulldown_cmark::Parser::new_ext(markdown, options).collect();

    // 目次の生成
    let mut headings = Vec::new();

    let mut current_text = String::new();

    let mut in_heading: Option<HeadingLevel> = None;

    let mut count = 0;

    for event in events.iter_mut() {
        match event {
            Event::Start(Tag::Heading { level, id, classes, attrs }) if *level <= HeadingLevel::H3 => {
                count += 1;
                *id = Some(CowStr::from(format!("h-{count}")));
                in_heading = Some(*level);
                current_text.clear();
            },
            Event::Text(text) if in_heading.is_some() => current_text.push_str(text),
            Event::End(TagEnd::Heading(level)) if in_heading == Some(*level) => {
                headings.push(HeadingInfo {
                    level: *level,
                    id: format!("h-{count}"),
                    text: current_text.clone(),
                });
                in_heading = None;
            },
            _ => {},
        }
    }

    let mut toc = String::from("<nav class=\"toc\"><ul>");
    for h in &headings {
        toc.push_str(&format!(
            "<li class=\"toc-{:?}\"><a href=\"#{}\">{}</a></li>",
            h.level, h.id, h.text
        ));
    }
    toc.push_str("</ul></nav>");

    let mut buffer = String::new();
    pulldown_cmark::html::push_html(&mut buffer, events.into_iter());

    format!("{toc}<main class=\"main\">{buffer}</main>")
}

fn build_page(md_path: &str, out_path: &str, css_href: &str) -> std::io::Result<()> {
    let content = fs::read_to_string(md_path)?;

    let res = convert_markdown_to_html(&content);

    let html = format!(
        "<html>
            <head>
                <meta charset=\"utf-8\">
                <link rel=\"preconnect\" href=\"https://fonts.googleapis.com\">
                <link rel=\"preconnect\" href=\"https://fonts.gstatic.com\" crossorigin>
                <link href=\"https://fonts.googleapis.com/css2?family=Noto+Sans+JP:wght@400;700&family=Roboto:wght@400;700&display=swap\" rel=\"stylesheet\">
                <link rel=\"stylesheet\" href=\"{css_href}\">
            </head>
            <body>
                <div class=\"wrapper\">
                    {res}
                </div>
            </body>
        </html>"
    );

    if let Some(parent) = std::path::Path::new(out_path).parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(out_path, html)?;

    Ok(())
}

fn main() -> std::io::Result<()> {
    if fs::exists("dist")? {
        fs::remove_dir_all("dist")?;
    }

    fs::create_dir_all("dist")?;

    build_page("content/index.md", "dist/index.html", "style.css")?;

    let mut dirs: Vec<String> = Vec::new();

    for entry in fs::read_dir("content")? {
        let entry = entry?;

        let path = entry.path();

        if path.is_dir() && path.join("index.md").exists() {
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                dirs.push(name.to_string());
            }
        }
    }

    dirs.sort();

    for dir in dirs.iter() {
        let md = format!("content/{dir}/index.md");
        let out = format!("dist/{dir}/index.html");
        build_page(&md, &out, "../style.css")?;
    }

    fs::copy("static/style.css", "dist/style.css")?;

    Ok(())
}
