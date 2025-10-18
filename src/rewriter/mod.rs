mod counter;
mod handle;
mod iframes;
mod images;
mod lists;
mod quotes;
pub mod sifter;
mod styles;
pub mod writer;

/// Insert a new line after
fn insert_newline_after(element: &mut lol_html::html_content::Element) {
    element.after("\n", lol_html::html_content::ContentType::Text);
}

/// Insert a new line before
fn insert_newline_before(element: &mut lol_html::html_content::Element) {
    element.before("\n", lol_html::html_content::ContentType::Text);
}
