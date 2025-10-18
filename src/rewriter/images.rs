use lol_html::html_content::Element;
use percent_encoding::{AsciiSet, CONTROLS, utf8_percent_encode};
use std::fmt::Write;
use url::Url;

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

/// Rewrite the image.
pub fn rewrite_image_element(el: &mut Element, commonmark: bool, url: &Option<Url>) {
    let src = el.get_attribute("src").unwrap_or_default();
    let alt = el.get_attribute("alt").unwrap_or_default();
    let title = el.get_attribute("title").unwrap_or_default();

    let height = el.get_attribute("height");
    let width = el.get_attribute("width");
    let align = el.get_attribute("align");

    if commonmark && (height.is_some() || width.is_some() || align.is_some()) {
        let mut img_tag = format!("<img src=\"{src}\"");

        if let Some(alt) = el.get_attribute("alt") {
            write!(img_tag, " alt=\"{alt}\"").unwrap();
        }
        if let Some(title) = el.get_attribute("title") {
            write!(img_tag, " title=\"{title}\"").unwrap();
        }
        if let Some(height) = height {
            write!(img_tag, " height=\"{height}\"").unwrap();
        }
        if let Some(width) = width {
            write!(img_tag, " width=\"{width}\"").unwrap();
        }
        if let Some(align) = align {
            write!(img_tag, " align=\"{align}\"").unwrap();
        }

        img_tag.push_str(" />");
        el.set_inner_content(&img_tag, lol_html::html_content::ContentType::Html);
    } else {
        let mut img_url = if src.contains(' ') {
            utf8_percent_encode(&src, FRAGMENT).to_string()
        } else {
            src
        };

        if img_url.starts_with('/')
            && let Some(u) = url
            && let Ok(n) = u.join(&img_url)
        {
            img_url = n.to_string();
        }

        el.replace(
            &format!(
                "![{}]({}{})",
                alt,
                img_url,
                if title.is_empty() {
                    String::new()
                } else {
                    format!(" \"{title}\"")
                }
            ),
            lol_html::html_content::ContentType::Html,
        );
    }
}

/// Rewrite the image.
pub fn rewrite_image_element_send(
    el: &mut lol_html::send::Element,
    commonmark: bool,
    url: &Option<Url>,
) {
    let src = el.get_attribute("src").unwrap_or_default();
    let alt = el.get_attribute("alt").unwrap_or_default();
    let title = el.get_attribute("title").unwrap_or_default();

    let height = el.get_attribute("height");
    let width = el.get_attribute("width");
    let align = el.get_attribute("align");

    if commonmark && (height.is_some() || width.is_some() || align.is_some()) {
        let mut img_tag = format!("<img src=\"{src}\"");

        if let Some(alt) = el.get_attribute("alt") {
            write!(img_tag, " alt=\"{alt}\"").unwrap();
        }
        if let Some(title) = el.get_attribute("title") {
            write!(img_tag, " title=\"{title}\"").unwrap();
        }
        if let Some(height) = height {
            write!(img_tag, " height=\"{height}\"").unwrap();
        }
        if let Some(width) = width {
            write!(img_tag, " width=\"{width}\"").unwrap();
        }
        if let Some(align) = align {
            write!(img_tag, " align=\"{align}\"").unwrap();
        }

        img_tag.push_str(" />");
        el.set_inner_content(&img_tag, lol_html::html_content::ContentType::Html);
    } else {
        let mut img_url = if src.contains(' ') {
            utf8_percent_encode(&src, FRAGMENT).to_string()
        } else {
            src
        };

        if img_url.starts_with('/')
            && let Some(u) = url
            && let Ok(n) = u.join(&img_url)
        {
            img_url = n.to_string();
        }

        el.replace(
            &format!(
                "![{}]({}{})",
                alt,
                img_url,
                if title.is_empty() {
                    String::new()
                } else {
                    format!(" \"{title}\"")
                }
            ),
            lol_html::html_content::ContentType::Html,
        );
    }
}
