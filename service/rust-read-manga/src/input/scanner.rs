use walkdir::WalkDir;

pub fn find_images(path: &str) -> Vec<String> {
    WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file())
        .filter(|e| {
            matches!(
                e.path().extension().and_then(|x| x.to_str()),
                Some("jpg" | "png" | "jpeg" | "webp")
            )
        })
        .map(|e| e.path().to_string_lossy().to_string())
        .collect()
}
