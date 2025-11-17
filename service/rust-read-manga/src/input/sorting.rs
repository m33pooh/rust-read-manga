use regex::Regex;

pub fn sort_numerically(mut files: Vec<String>) -> Vec<String> {
    let re = Regex::new(r"(\d+)").unwrap(); // Regex to capture numbers

    files.sort_by(|a, b| {
        let mut a_parts = re.find_iter(a).map(|m| m.as_str().parse::<usize>().unwrap_or(0));
        let mut b_parts = re.find_iter(b).map(|m| m.as_str().parse::<usize>().unwrap_or(0));

        loop {
            match (a_parts.next(), b_parts.next()) {
                (Some(a_num), Some(b_num)) => {
                    if a_num != b_num {
                        return a_num.cmp(&b_num);
                    }
                },
                (Some(_), None) => return std::cmp::Ordering::Greater,
                (None, Some(_)) => return std::cmp::Ordering::Less,
                (None, None) => return a.cmp(b), // Fallback to lexicographical if numbers are equal or absent
            }
        }
    });
    files
}
