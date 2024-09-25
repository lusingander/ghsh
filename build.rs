use std::path::PathBuf;

fn main() {
    let schema_path = PathBuf::from("./src/github/graphql/schema.graphql");

    if !schema_path.exists() {
        let schema = download().unwrap();
        std::fs::write(&schema_path, schema).unwrap();
    }
}

fn download() -> Result<String, Box<dyn std::error::Error>> {
    let text =
        reqwest::blocking::get("https://docs.github.com/public/fpt/schema.docs.graphql")?.text()?;
    Ok(text)
}
