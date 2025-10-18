use extended::sifter::{WhitespaceSifter, WhitespaceSifterBytes};
use lazy_static::lazy_static;
use regex::Regex;

// we want to just use the rewriter instead for v0.1.
pub mod extended;

pub mod rewriter;

lazy_static! {
    static ref MARKDOWN_MIDDLE_KEYCHARS: Regex = Regex::new(r"[<>*\\_~]").expect("valid regex pattern"); // for Markdown escaping
    static ref MARKDOWN_MIDDLE_KEYCHARS_SET: regex::RegexSet = regex::RegexSet::new([
        r"[<>*\\_~]",  // Matches any single markdown character
        r"&nbsp;"      // Matches the entire "&nbsp;" string
    ]).expect("valid regex set");
}

/// Main function of this library to come. Rewrites incoming HTML, converts it into Markdown
/// and returns converted string. Incomplete work in progress for major performance increases.
/// # Arguments
/// `html` is source HTML as `String`
pub fn rewrite_html(html: &str, commonmark: bool) -> String {
    rewriter::writer::convert_html_to_markdown(html, &None, commonmark, &None).unwrap_or_default()
}

/// Called after all processing has been finished
///
/// Clears excessive punctuation that would be trimmed by renderer anyway
pub fn clean_markdown(input: &str) -> String {
    input.sift()
}

/// Called after all processing has been finished
///
/// Clears excessive punctuation that would be trimmed by renderer anyway
pub fn clean_markdown_bytes(input: &Vec<u8>) -> String {
    input.sift_bytes()
}

/// Replace the markdown chars cleanly.
pub fn replace_markdown_chars(input: &str) -> String {
    use crate::MARKDOWN_MIDDLE_KEYCHARS_SET;

    if !MARKDOWN_MIDDLE_KEYCHARS_SET.is_match(input) {
        return input.to_string();
    }

    let mut output = String::new();
    let mut chars = input.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '&' {
            let mut entity = String::new();
            entity.push(ch);
            while let Some(&next_ch) = chars.peek() {
                entity.push(next_ch);
                chars.next();
                if entity == "&nbsp;" {
                    entity.clear(); // discard &nbsp;
                    break;
                } else if next_ch == ';' || entity.len() > 6 {
                    output.push_str(&entity);
                    break;
                }
            }
            if !entity.is_empty() {
                output.push_str(&entity);
            }
        } else if "<>*\\_~".contains(ch) {
            output.push('\\');
            output.push(ch);
        } else {
            output.push(ch);
        }
    }

    output
}
