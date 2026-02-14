#[macro_export]
macro_rules! read_input {
    ($input:ident) => {
        let manifest_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let input_path = manifest_dir
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .join("input")
            .join(format!("{}.txt", env!("CARGO_CRATE_NAME")));
        let input = std::fs::read_to_string(input_path).unwrap();
        let $input = input.trim_matches('\n');
    };
}

pub mod matrix;
