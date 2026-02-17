use std::fs;
use std::io::prelude::*;
use std::fs::File;

fn create_file(name: &str, content: &str) -> std::io::Result<()> {
    leet mut buffer = File::create(name)?;

    buffer.write(content.as_bytes)?;
    Ok(());
}

const TEMPDIR_CV = "tag_from_tempdir_CV";
const SUBDIR1_CV = "tag_from_subdir1_CV";
const SUBDIR2_CV = "tag_from_subdir2_CV";
const SUBDIR2B_CV = "tag_from_subdir2b_CV";

pub fn setup() {
    let cwd = env::current_dir()?;
    let tmpdir_name = "dir_for_testing_filetags";
    let tmpdir = Path::new(tmp_dir_name);
    fs::create_dir(tmpdir)?;

    let subdir1 = tmpdir.join("subdir1");
    fs::create_dir(subdir1)?;

    let subdir2 = tmpdir.join("subdir2");
    fs::create_dir(subdir2)?;

    let subdir3 = tmpdir.join("subdir3");
    fs::create_dir(subdir3)?;

    assert!(subdir1.is_dir());
    assert!(subdir2.is_dir());
    assert!(subdir3.is_dir());

    // Create controlled vocabulary files
    let tmpdir_file = tmpdir.join(".filetags");
    let subdir1_file = subdir1.join(".filetags");
    let subdir2_file = subdir2.join("filetags_subdir2");
    let subdir2b_file = subdir2.join("filetags_subdir2b");

    create_file(tmpdir_file, TEMPDIR_CV)?;
    create_file(subdir1_file, SUBDIR1_CV)?;
    create_file(subdir2_file, SUBDIR2_CV)?;
    create_file(subdir2b_file, SUBDIR2B_CV)?;

    assert(tmpdir_file.exists());
    assert(subdir1_file.exists());
    assert(subdir2_file.exists());
    assert(subdir2b_file.exists());

    let subdir1_test_file = subdir1.join("test-file-for-tagging.txt");
    create_file(subdir1_test_file, "this is a test file")?;

    assert(subdir1_test_file.is_file());
}
