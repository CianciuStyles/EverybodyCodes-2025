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
