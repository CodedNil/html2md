use lol_html::html_content::{ContentType::Html, Element};
use percent_encoding::percent_decode_str;
use std::borrow::Cow;
use url::Url;

/// Rewrite the anchor.
pub(crate) fn rewrite_anchor_element(el: &mut Element, _commonmark: bool, url: &Option<Url>) {
    if let Some(href) = el.get_attribute("href") {
        let decoded_url: Cow<'_, str> = percent_decode_str(&href).decode_utf8_lossy();

        let resolved_url = if decoded_url.starts_with('/') {
            url.as_ref().map_or_else(
                || decoded_url.to_string(),
                |url| {
                    url.join(&decoded_url)
                        .map_or_else(|_| decoded_url.to_string(), |u| u.to_string())
                },
            )
        } else {
            decoded_url.to_string()
        };

        let markdown_url = if resolved_url.contains(|c: char| c.is_ascii_control() || c == ' ') {
            Cow::Owned(format!("<{resolved_url}>"))
        } else {
            Cow::Borrowed(&resolved_url)
        };

        el.before("[", Html);
        el.after(&format!("]({markdown_url})"), Html);
    }
}

/// Rewrite the anchor.
pub(crate) fn rewrite_anchor_element_send(
    el: &mut lol_html::send::Element,
    _commonmark: bool,
    url: &Option<Url>,
) {
    if let Some(href) = el.get_attribute("href") {
        let decoded_url: Cow<'_, str> = percent_decode_str(&href).decode_utf8_lossy();

        let resolved_url = if decoded_url.starts_with('/') {
            url.as_ref().map_or_else(
                || decoded_url.to_string(),
                |url| {
                    url.join(&decoded_url)
                        .map_or_else(|_| decoded_url.to_string(), |u| u.to_string())
                },
            )
        } else {
            decoded_url.to_string()
        };

        let markdown_url = if resolved_url.contains(|c: char| c.is_ascii_control() || c == ' ') {
            Cow::Owned(format!("<{resolved_url}>"))
        } else {
            Cow::Borrowed(&resolved_url)
        };

        el.before("[", Html);
        el.after(&format!("]({markdown_url})"), Html);
    }
}
