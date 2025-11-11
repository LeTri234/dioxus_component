pub fn cn(parts: impl IntoIterator<Item = Option<impl AsRef<str>>>) -> String {
    parts
        .into_iter()
        .filter_map(|p| {
            p.and_then(|s| {
                let trimmed = s.as_ref().trim();
                if trimmed.is_empty() {
                    None
                } else {
                    Some(trimmed.to_string())
                }
            })
        })
        .collect::<Vec<_>>()
        .join(" ")
}
