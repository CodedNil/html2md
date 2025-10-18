use crate::extended::base::iframe::{INSTAGRAM_PATTERN, VK_PATTERN, YOUTUBE_PATTERN};
use lol_html::html_content::ContentType::Text;
use lol_html::html_content::Element;

/// Handle the conversion to iframes.
pub fn handle_iframe(element: &mut Element) {
    if let Some(src) = element.get_attribute("src") {
        if let Some(capture) = YOUTUBE_PATTERN.captures(&src) {
            let media_id = capture.get(1).map_or("", |m| m.as_str());
            element.replace(
                &format!("[![Embedded YouTube video](https://img.youtube.com/vi/{media_id}/0.jpg)](https://www.youtube.com/watch?v={media_id})"),
                Text
            );
            return;
        }

        if let Some(capture) = INSTAGRAM_PATTERN.captures(&src) {
            let media_id = capture.get(1).map_or("", |m| m.as_str());
            element.replace(
                &format!("[![Embedded Instagram post](https://www.instagram.com/p/{media_id}/media/?size=m)](https://www.instagram.com/p/{media_id}/embed/)"),
                Text
            );
            return;
        }

        if let Some(capture) = VK_PATTERN.captures(&src) {
            let owner_id = capture.get(1).map_or("", |m| m.as_str());
            let video_id = capture.get(2).map_or("", |m| m.as_str());
            element.replace(
                &format!("[![Embedded VK video](https://st.vk.com/images/icons/video_empty_2x.png)](https://vk.com/video{owner_id}_{video_id})"),
                Text,
            );
        }
    }
}

/// Handle the conversion to iframes.
pub fn handle_iframe_send(element: &mut lol_html::send::Element) {
    if let Some(src) = element.get_attribute("src") {
        if let Some(capture) = YOUTUBE_PATTERN.captures(&src) {
            let media_id = capture.get(1).map_or("", |m| m.as_str());
            element.replace(
                &format!("[![Embedded YouTube video](https://img.youtube.com/vi/{media_id}/0.jpg)](https://www.youtube.com/watch?v={media_id})"),
                Text
            );
            return;
        }

        if let Some(capture) = INSTAGRAM_PATTERN.captures(&src) {
            let media_id = capture.get(1).map_or("", |m| m.as_str());
            element.replace(
                &format!("[![Embedded Instagram post](https://www.instagram.com/p/{media_id}/media/?size=m)](https://www.instagram.com/p/{media_id}/embed/)"),
                Text
            );
            return;
        }

        if let Some(capture) = VK_PATTERN.captures(&src) {
            let owner_id = capture.get(1).map_or("", |m| m.as_str());
            let video_id = capture.get(2).map_or("", |m| m.as_str());
            element.replace(
                &format!("[![Embedded VK video](https://st.vk.com/images/icons/video_empty_2x.png)](https://vk.com/video{owner_id}_{video_id})"),
                Text,
            );
        }
    }
}
