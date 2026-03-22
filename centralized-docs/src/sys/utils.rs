pub fn extract_last_path_segment(url_str: &str) -> Option<String> {
    url::Url::parse(url_str).ok().and_then(|u| {
        u.path_segments()
            .map(|segments| segments.collect::<Vec<_>>())
            .and_then(|vec| vec.into_iter().next_back().map(String::from))
    })
}
