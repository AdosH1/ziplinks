use crate::data::link::Link;

/// Creates windows.open(<url>) based on links, followed by a window.close() at the end.  
/// The entire string is encapsulated in a <script> tag that is inserted into the body of the given string.
/// Returns a tuple of the number of characters generated and the new string.
pub fn open_links(links: &Vec<Link>) -> (usize, String) {
    let mut links_str = String::new();

    links_str.push_str("<script type='text/javascript'>");
    for link in links {
        let t = format!("window.open('{}');", link.url);
        links_str.push_str(&t);
    }
    links_str.push_str("</script>");

    (links_str.chars().count(), links_str)
}
