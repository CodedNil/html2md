use lol_html::html_content::{ContentType, Element, TextChunk};
use std::rc::Rc;
use std::sync::atomic::AtomicUsize;

// Function to handle <blockquote> elements
pub fn rewrite_blockquote_element(el: &mut Element, quote_depth: &Rc<AtomicUsize>) {
    quote_depth.fetch_add(1, std::sync::atomic::Ordering::Relaxed);

    if let Some(end_tag_handlers) = el.end_tag_handlers() {
        end_tag_handlers.push(Box::new({
            let quote_depth = quote_depth.clone();
            move |_end| {
                quote_depth.fetch_sub(1, std::sync::atomic::Ordering::Relaxed);

                Ok(())
            }
        }));
    }
}

// Function to handle text within <blockquote> elements
pub fn rewrite_blockquote_text(text_chunk: &mut TextChunk<'_>, quote_depth: &Rc<AtomicUsize>) {
    let depth = quote_depth.load(std::sync::atomic::Ordering::Relaxed);
    let quote_prefix = "> ".repeat(depth);
    let lines: Vec<&str> = text_chunk.as_str().lines().collect();
    let total_lines = lines.len();

    let last = text_chunk.last_in_text_node();

    let modified_text = lines
        .iter()
        .enumerate()
        .map(|(i, line)| {
            if i >= 1 && i == total_lines - 1 {
                (*line).to_string()
            } else {
                format!("{quote_prefix}{line}")
            }
        })
        .collect::<String>();

    text_chunk.replace(&modified_text, ContentType::Html);

    if last {
        text_chunk.after("\n", ContentType::Text);
    }
}
