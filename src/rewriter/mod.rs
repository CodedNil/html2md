pub mod anchors;
pub mod counter;
pub mod handle;
pub mod iframes;
pub mod images;
pub mod lists;
pub mod quotes;
pub mod styles;
pub mod writer;

/// Insert a new line after
#[inline]
pub fn insert_newline_after(element: &mut lol_html::html_content::Element) {
    element.after("\n", lol_html::html_content::ContentType::Text);
}

/// Insert a new line before
#[inline]
pub fn insert_newline_before(element: &mut lol_html::html_content::Element) {
    element.before("\n", lol_html::html_content::ContentType::Text);
}

/// Insert a new line after
#[inline]
pub fn insert_newline_after_send(element: &mut lol_html::send::Element) {
    element.after("\n", lol_html::html_content::ContentType::Text);
}

/// Insert a new line before
#[inline]
pub fn insert_newline_before_send(element: &mut lol_html::send::Element) {
    element.before("\n", lol_html::html_content::ContentType::Text);
}
