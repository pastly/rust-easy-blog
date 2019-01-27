use super::post::file::File as PostFile;

pub fn begin_html(title: &str) -> String {
    format!("
<!DOCTYPE html>
<html>
<head>
    <title>{title}</title>
    <link href='/static/style.css' rel='stylesheet' type='text/css' />
    <link rel='icon' type='image/png' href='/static/img/favicon.png' />
    <meta charset='utf-8' />
</head>
<body>
<div id='page_content'>\n", title=title)
}

pub fn end_html() -> String {
        format!("
</div> <!-- page_content -->
</body>
</html>\n")
}

pub fn page_header(title: &str, subtitle: &str) -> String {
    format!("
<header>
    <h1 id='blog_title'>{title}</h1>
    <h2 id='blog_subtitle'>{subtitle}</h2>
    <img id='blog_img' src='/static/img/header.jpg' />
</header>\n", title=title, subtitle=subtitle)
}

pub fn page_footer() -> String {
    format!("
<footer>
</footer>\n")
}

pub fn post_header(title: String, author: String, link: Option<String>) -> String {
    let mut s = String::new();
    s += "<div class='post_header'>\n";
    s += &if link.is_some() {
        format!(
            "<h1 class='post_title'><a href='{link}'>{title}</a></h1>\n",
            link=link.unwrap(), title=title)
    } else {
        format!(
            "<h1 class='post_title'>{title}</h1>\n",
            title=title)
    };
    s += &format!(
        "<p class='post_author'>{author}</p>\n",
        author=author);
    // post date
    // post mod date
    // post permalink
    s += "</div> <!-- post_header -->\n";
    s
}

pub fn post_footer() -> String {
    String::new()
}

pub fn css() -> String {
    format!("
body {{
    font-family: Georgia, 'Times New Roman', Times, serif;
    margin: 0;
    padding: 0;
    background-color: #F3F3F3;
}}
header,
footer,
article {{
    background-color: #FFF;
    border: 1px solid #CCC;
}}
header {{
    display: grid;
    grid-template-columns: auto 150px;
    grid-template-rows: 1fr auto auto 6fr;
    grid-template-areas:
        '.        img'
        'title    img'
        'subtitle img'
        '.        img';
    justify-items: center;
}}
article {{
    padding: 20px 40px 20px 40px;
}}
#page_content {{
    padding: 5px;
    background-color: #DDD;
    max-width: 900px;
    margin: 24px auto;
}}
a {{
    text-decoration: none;
    color: #336699;
}}
a:hover {{
    color: #5588bb;
}}
#blog_title {{
    grid-area: title;
}}
#blog_subtitle {{
    grid-area: subtitle;
    font-size: medium;
    font-weight: normal;
}}
#blog_img {{
    grid-area: img;
    align-self: center;
}}
img {{
    max-width: 100%;
}}
")
}
