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
    let mut sections = Vec::new();
    let mut start = 0;

    while let Some(rel_pos) = text[start..]
        .find("\n\n")
        .or_else(|| text[start..].find("\r\n\r\n"))
    {
        let pos = start + rel_pos;

        // Determine which separator was found
        let separator_len = if text[pos..].starts_with("\r\n\r\n") {
            4
        } else {
            2
        };

        sections.push(&text[start..pos]);
        start = pos + separator_len;
    }

    // Add the final section
    sections.push(&text[start..]);

    sections
}

pub fn split_at_triple_newline(text: &str) -> Vec<&str> {
    let mut sections = Vec::new();
    let mut start = 0;

    while let Some(rel_pos) = text[start..]
        .find("\n\n\n")
        .or_else(|| text[start..].find("\r\n\r\n\r\n"))
    {
        let pos = start + rel_pos;

        // Determine which separator was found
        let separator_len = if text[pos..].starts_with("\r\n\r\n\r\n") {
            6
        } else {
            3
        };

        sections.push(&text[start..pos]);
        start = pos + separator_len;
    }

    // Add the final section
    sections.push(&text[start..]);

    sections
}
