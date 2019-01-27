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
    <h1>{title}</h1>
    <h2>{subtitle}</h2>
</header>\n", title=title, subtitle=subtitle)
}

pub fn page_footer() -> String {
    format!("
<footer>
</footer>\n")
}

pub fn post_header(title: String, author: String) -> String {
    let mut s = String::new();
    s += &format!("
<div class='post_header'>
<h1 class='post_title'>{title}</h1>
<p class='post_author'>{author}</p>\n",
        title=title,
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
article {{
    padding: 20px 40px 20px 40px;
}}
#page_content {{
    padding: 5px;
    background-color: #DDD;
    max-width: 900px;
    margin: 24px auto;
}}")
}
