use crate::data::link::Link;

pub fn parse_body_to_links(body: String) -> Vec<Link> {
    let b: String = body.chars().skip(6).collect();
    let v: Vec<&str> = b.split("\r\n").collect();
    let mut links = Vec::new();

    for link in v.iter() {
        let owned = link.to_owned().to_owned();
        let l = Link::try_create(owned);
        if let Some(valid_link) = l {
            links.push(valid_link);
        }
    }
    links
}
