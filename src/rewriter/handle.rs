use super::anchors::rewrite_anchor_element_send;
use super::iframes::{handle_iframe, handle_iframe_send};
use super::images::{rewrite_image_element, rewrite_image_element_send};
use super::lists::{handle_list_or_item, handle_list_or_item_send};
use super::quotes::{rewrite_blockquote_element, rewrite_blockquote_element_send};
use super::styles::{rewrite_style_element, rewrite_style_element_send};
use super::{
    insert_newline_after, insert_newline_after_send, insert_newline_before,
    insert_newline_before_send,
};
use lol_html::html_content::ContentType::{Html, Text};
use lol_html::html_content::Element;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use url::Url;

/// Handle the `lol_html` tag.
#[inline]
pub fn handle_tag(
    element: &mut Element,
    commonmark: bool,
    url: &Option<Url>,
    list_type: &mut Option<String>,
    order_counter: &mut usize,
    quote_depth: &Rc<AtomicUsize>,
    inside_table: &mut bool,
) {
    let element_name = element.tag_name();

    let remove_attrs =
        commonmark && (element_name.as_str() == "sub" || element_name.as_str() == "sup");

    // check common mark includes.
    if remove_attrs {
        let attrs = element
            .attributes()
            .iter()
            .map(lol_html::html_content::Attribute::name)
            .collect::<Vec<String>>();

        for attr in &attrs {
            element.remove_attribute(attr);
        }
    } else {
        element.remove_and_keep_content();
    }

    // Add the markdown equivalents before the element.
    match element_name.as_str() {
        "h1" => {
            element.before("# ", Text);
            insert_newline_after(element);
        }
        "h2" => {
            element.before("## ", Text);
            insert_newline_after(element);
        }
        "h3" => {
            element.before("### ", Text);
            insert_newline_after(element);
        }
        "h4" => {
            element.before("#### ", Text);
            insert_newline_after(element);
        }
        "h5" => {
            element.before("##### ", Text);
            insert_newline_after(element);
        }
        "h6" => {
            element.before("###### ", Text);
            insert_newline_after(element);
        }
        "p" | "div" | "section" | "header" | "footer" => {
            insert_newline_before(element);
            insert_newline_after(element);
        }
        "hr" => {
            insert_newline_before(element);
            element.append("---", Text);
            insert_newline_after(element);
        }
        "br" => insert_newline_after(element),
        "a" | "img" => {
            rewrite_image_element(element, commonmark, url);
        }
        "table" => {
            *inside_table = true;
        }
        "tr" => {
            insert_newline_after(element);
        }
        "th" => {
            // add the first table row start
            if *inside_table {
                element.before("|", Html);
                *inside_table = false;
            }
            if commonmark {
                element.before("** ", Html);
                element.after("** |", Html);
            } else {
                element.after("|", Html);
            }
        }
        "td" => {
            element.after("|", Html);
        }
        "iframe" => {
            handle_iframe(element);
        }
        "b" | "i" | "s" | "strong" | "em" | "del" => {
            rewrite_style_element(element);
        }
        "ol" | "ul" | "menu" | "li" => {
            handle_list_or_item(element, list_type, order_counter);
        }
        "q" | "cite" | "blockquote" => {
            rewrite_blockquote_element(element, quote_depth);
        }
        "pre" => {
            element.before("\n```\n", Html);
            element.after("\n```\n", Html);
        }
        "code" | "samp" => {
            element.before("`", Html);
            element.after("`", Html);
        }
        _ => (),
    }
}

/// Handle the `lol_html` tag.
#[inline]
pub fn handle_tag_send(
    element: &mut lol_html::send::Element,
    commonmark: bool,
    url: &Option<Url>,
    list_type: &mut Option<String>,
    order_counter: &mut usize,
    quote_depth: &Arc<AtomicUsize>,
    inside_table: &mut bool,
) {
    let element_name = element.tag_name();

    let remove_attrs =
        commonmark && (element_name.as_str() == "sub" || element_name.as_str() == "sup");

    // check common mark includes.
    if remove_attrs {
        let attrs = element
            .attributes()
            .iter()
            .map(lol_html::html_content::Attribute::name)
            .collect::<Vec<String>>();

        for attr in &attrs {
            element.remove_attribute(attr);
        }
    } else {
        element.remove_and_keep_content();
    }

    // Add the markdown equivalents before the element.
    match element_name.as_str() {
        "h1" => {
            element.before("# ", Text);
            insert_newline_after_send(element);
        }
        "h2" => {
            element.before("## ", Text);
            insert_newline_after_send(element);
        }
        "h3" => {
            element.before("### ", Text);
            insert_newline_after_send(element);
        }
        "h4" => {
            element.before("#### ", Text);
            insert_newline_after_send(element);
        }
        "h5" => {
            element.before("##### ", Text);
            insert_newline_after_send(element);
        }
        "h6" => {
            element.before("###### ", Text);
            insert_newline_after_send(element);
        }
        "p" | "div" | "section" | "header" | "footer" => {
            insert_newline_before_send(element);
            insert_newline_after_send(element);
        }
        "hr" => {
            insert_newline_before_send(element);
            element.append("---", Text);
            insert_newline_after_send(element);
        }
        "br" => insert_newline_after_send(element),
        "a" => {
            rewrite_anchor_element_send(element, commonmark, url);
        }
        "img" => {
            rewrite_image_element_send(element, commonmark, url);
        }
        "table" => *inside_table = true,
        "tr" => {
            insert_newline_after_send(element);
        }
        "th" => {
            if *inside_table {
                element.before("|", Html);
                *inside_table = false;
            }
            if commonmark {
                element.before("** ", Html);
                element.after("** |", Html);
            } else {
                element.after("|", Html);
            }
        }
        "td" => {
            element.after("|", Html);
        }
        "iframe" => {
            handle_iframe_send(element);
        }
        "b" | "i" | "s" | "strong" | "em" | "del" => {
            rewrite_style_element_send(element);
        }
        "ol" | "ul" | "menu" | "li" => {
            handle_list_or_item_send(element, list_type, order_counter);
        }
        "q" | "cite" | "blockquote" => {
            rewrite_blockquote_element_send(element, quote_depth);
        }
        "pre" => {
            element.before("\n```\n", Html);
            element.after("\n```\n", Html);
        }
        "code" | "samp" => {
            element.before("`", Html);
            element.after("`", Html);
        }
        _ => (),
    }
}
