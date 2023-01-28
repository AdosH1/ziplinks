use crate::data::http::content_type::ContentType;
use crate::data::link::Link;
use crate::io::file::load_file;
use crate::libs::html::generate::open_links;
use random_string::generate;

pub fn generate_bad_request() -> anyhow::Result<Vec<u8>> {
    let template = load_file("empty.html", &ContentType::TextHtml)?;
    let mut webpage = String::from_utf8(template)?;

    let body = "
  <center>
  <br/>
  <h1>400: Bad Request</h1>
  <p>Sorry, something went wrong with your request.</p>
  </center>";

    webpage.insert_str(1414, body);

    Ok(webpage.into_bytes())
}

pub fn generate_homepage() -> anyhow::Result<Vec<u8>> {
    let template = load_file("empty.html", &ContentType::TextHtml)?;
    let mut webpage = String::from_utf8(template)?;

    let body = "
<center>
<br/>
<p>Please enter the full url (including https://) in the field below, with a new link on each line.</p>
<form method='POST' action='/generate'>
  <div class='mdl-textfield mdl-js-textfield' style='background-color: #E7E7CE; padding: 20px; border-radius: 25px; width: 50%; height: 40%'>
    <textarea id='links' name='links' class='mdl-textfield__input' type='text' rows= '10' ></textarea>
  </div>
  <br/>
  <br/>
  <p><b>Disclaimer:</b> There are no guarantees on the longevity or stableness of ziplinks. Data may be deleted or removed at any point in time.</p>
  <button class='mdl-button mdl-js-button mdl-button--raised mdl-button--accent'>
    Submit
  </button>
</form>
</center>";

    webpage.insert_str(1414, body);

    Ok(webpage.into_bytes())
}

pub fn generate_not_found() -> anyhow::Result<Vec<u8>> {
    let template = load_file("empty.html", &ContentType::TextHtml)?;
    let mut webpage = String::from_utf8(template)?;

    let body = "
  <center>
  <br/>
  <h1>404: Not Found</h1>
  <p>Sorry, the page you are looking for can't be found.</p>
  </center>";

    webpage.insert_str(1414, body);

    Ok(webpage.into_bytes())
}

pub fn generate_internal_error() -> anyhow::Result<Vec<u8>> {
    let template = load_file("empty.html", &ContentType::TextHtml)?;
    let mut webpage = String::from_utf8(template)?;

    let body = "
  <center>
  <br/>
  <h1>500: Internal Error</h1>
  <p>Sorry, something went wrong on our side.</p>
  </center>";

    webpage.insert_str(1414, body);

    Ok(webpage.into_bytes())
}

pub fn generate_link_page(sub_url: &String) -> anyhow::Result<Vec<u8>> {
    let template = load_file("empty.html", &ContentType::TextHtml)?;
    let mut webpage = String::from_utf8(template)?;

    let body = format!(
        "
  <center>
  <br/>
  <br/>
  <br/>
    <p>Please find your links here: <a id='link' href='/{}'></a></p>
    <script>document.getElementById('link').innerHTML = window.location.host + '/{}';</script>
  </center>",
        &sub_url, &sub_url
    );
    webpage.insert_str(1414, &body);

    Ok(webpage.into_bytes())
}

pub fn generate_link_opening_page(links: &Vec<Link>) -> anyhow::Result<Vec<u8>> {
    let template = load_file("empty.html", &ContentType::TextHtml)?;
    let mut webpage = String::from_utf8(template)?;

    let body = format!("
  <br/>
  <center>
      <h2>Thank you for using ziplinks!</h2>
      <img src='/resource/images/marauder' alt='A marauder from the game Starcraft 2 dancing.'
        style='background-color: #E7E7CE; padding: 20px; border-radius: 25px; width: 25%; height: 20%'/>
  </center>");
    webpage.insert_str(1414, &body);

    // Insert header
    webpage.insert_str(360, &open_links(links).1);

    Ok(webpage.into_bytes())
}

pub fn generate_sub_url() -> String {
    let charset = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let sub_url = generate(6, charset);

    // todo : check if its unique against database (when we have one), if not - regen
    // worth figuring our when collision gets bad, increase length or better way of avoiding collisions

    format!("{}", sub_url)
}
