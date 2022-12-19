// checks if string could possibly be a valid URL, and if it isn't, formats it into a valid URL
pub fn format_url(url: String) -> String {
    if url.contains("://") {
        url
    } else {
        format!("http://{}", url)
    }
}