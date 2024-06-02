use nasa_download::testable_find_table_in_content;
use scraper::Html;
use std::fs;
use std::io;
use std::path::PathBuf;

fn load_test_file(relpath: &str) -> io::Result<Html> {
    let project_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let mut file_path = project_root.clone();
    file_path.push(relpath);
    let content = fs::read_to_string(file_path)?;
    Ok(Html::parse_fragment(&content))
}

#[test]
fn given_html_content_when_content_has_table_table_is_found() -> io::Result<()> {
    let body = load_test_file("tests/assets/JNOJNC_0001.html")?;
    let table = testable_find_table_in_content(&body);
    assert_ne!(table, None);
    Ok(())
}
