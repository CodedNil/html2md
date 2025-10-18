use lol_html::html_content::ContentType::Text;
use lol_html::html_content::Element;
use regex_lite::Regex;
use std::sync::LazyLock;

/// Pattern that detects iframes with Youtube embedded videos<br/>
/// Examples:
/// * `https://www.youtube.com/embed/zE-dmXZp3nU?wmode=opaque`
/// * `https://www.youtube-nocookie.com/embed/5yo6exIypkY`
/// * `https://www.youtube.com/embed/TXm6IXrbQuM`
pub static YOUTUBE_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"www\.youtube(?:-nocookie)?\.com/embed/([-\w]+)").expect("valid regex pattern")
});

/// Pattern that detects iframes with Instagram embedded photos<br/>
/// Examples:
/// * `https://www.instagram.com/p/B1BKr9Wo8YX/embed/`
/// * `https://www.instagram.com/p/BpKjlo-B4uI/embed/`
pub static INSTAGRAM_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"www\.instagram\.com/p/([-\w]+)/embed").expect("valid regex pattern")
});

/// Pattern that detects iframes with `VKontakte` embedded videos<br/>
/// Examples:
/// * `https://vk.com/video_ext.php?oid=-49423435&id=456245092&hash=e1611aefe899c4f8`
/// * `https://vk.com/video_ext.php?oid=-76477496&id=456239454&hash=ebfdc2d386617b97`
pub static VK_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"vk\.com/video_ext\.php\?oid=(-?\d+)&id=(\d+)&hash=(.*)")
        .expect("valid regex pattern")
});

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
