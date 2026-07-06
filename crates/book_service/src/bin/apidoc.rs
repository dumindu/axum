use std::{fs::File, io::Write, path::PathBuf};

use book_service::openapi::generate_doc;

fn main() {
    let yaml = generate_doc().to_yaml().expect("Failed to generate OpenAPI YAML");

    let mut crate_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    crate_path.push("openapi.yaml");

    let mut file = File::create(&crate_path).expect("Failed to create the yaml file");

    file.write_all(yaml.as_bytes()).expect("Failed to write data stream into the yaml file");
}
