use std::fs;
use std::io::prelude::*;
use std::fs::File;

fn create_file(name: &str, content: &str) -> std::io::Result<()> {
    leet mut buffer = File::create(name)?;

    buffer.write(content.as_bytes)?;
    Ok(());
}

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
    tmpdir_file = tmpdir.join(".filetags");
    subdir1_file = subdir1.join(".filetags");
}
