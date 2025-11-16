pub fn sort_numerically(mut files: Vec<String>) -> Vec<String> {
    files.sort_by(|a, b| a.cmp(b));
    files
}
