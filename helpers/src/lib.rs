#[macro_export]
macro_rules! input_file {
    ($part_num:expr) => {
        std::fs::read_to_string(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("data")
                .join(format!("input_{}.txt", $part_num)),
        )
        .unwrap()
    };
}

#[macro_export]
macro_rules! sample_file {
    ($part_num:expr) => {
        std::fs::read_to_string(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("data")
                .join(format!("sample_{}.txt", $part_num)),
        )
        .unwrap()
    };
}

pub fn split_at_double_newline(text: &str) -> Vec<&str> {
    let unix_idx = text.find("\n\n");
    let win_idx = text.find("\r\n\r\n");

    match (unix_idx, win_idx) {
        // Both found, split at the one that appears first
        (Some(u), Some(w)) => {
            if u < w {
                vec![&text[..u], &text[u + 2..]]
            } else {
                vec![&text[..w], &text[w + 4..]]
            }
        }
        // Only Unix style found
        (Some(u), None) => vec![&text[..u], &text[u + 2..]],
        // Only Windows style found
        (None, Some(w)) => vec![&text[..w], &text[w + 4..]],
        // No double newline found at all
        (None, None) => vec![],
    }
}