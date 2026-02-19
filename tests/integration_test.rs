use std::fs;
use std::path::PathBuf;
use assert_fs::prelude::*;
use std::io::Write;

fn create_file(name: &PathBuf, content: &str) -> std::io::Result<()> {
    let mut buffer = fs::File::create(name)?;

    buffer.write_all(content.as_bytes())?;
    Ok(())
}

const TEMPDIR_CV: &str = "tag_from_tempdir_CV";
const SUBDIR1_CV: &str = "tag_from_subdir1_CV";
const SUBDIR2_CV: &str = "tag_from_subdir2_CV";
const SUBDIR2B_CV: &str = "tag_from_subdir2b_CV";

#[test]
fn test_locate_parse_controlled_vocabulary() -> Result<(), Box<dyn std::error::Error>> {
    let tmp_dir = assert_fs::TempDir::new().unwrap();

    let subdir1 = tmp_dir.path().join("subdir1");
    fs::create_dir(&subdir1)?;

    let subdir2 = tmp_dir.path().join("subdir2");
    fs::create_dir(&subdir2)?;

    let subdir3 = tmp_dir.path().join("subdir3");
    fs::create_dir(&subdir3)?;

    assert!(subdir1.is_dir());
    assert!(subdir2.is_dir());
    assert!(subdir3.is_dir());

    // Create controlled vocabulary files
    let tmp_dir_file = tmp_dir.path().join(".filetags");
    let subdir1_file = subdir1.join(".filetags");
    let subdir2_file = subdir2.join("filetags_subdir2");
    let subdir2b_file = subdir2.join("filetags_subdir2b");

    create_file(&tmp_dir_file, TEMPDIR_CV)?;
    create_file(&subdir1_file, SUBDIR1_CV)?;
    create_file(&subdir2_file, SUBDIR2_CV)?;
    create_file(&subdir2b_file, SUBDIR2B_CV)?;

    assert!(tmp_dir_file.exists());
    assert!(subdir1_file.exists());
    assert!(subdir2_file.exists());
    assert!(subdir2b_file.exists());

    let subdir1_test_file = subdir1.join("test-file-for-tagging.txt");
    create_file(&subdir1_test_file, "this is a test file")?;

    assert!(subdir1_test_file.is_file());

    Ok(())
}
