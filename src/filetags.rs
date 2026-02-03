use lazy_static::lazy_static;
use regex::Regex;
// use std::ffi::{OsStr, OsString};

// static YYYY_MM_DD_PATTERN:: &str Regex = Regex::new("^(\\d{4})([01]\\d)([0123]\\d)[- _T]").unwrap();

use std::path::Path;
// use std::error::Error;
// use std::iter;
use std::fmt;
use std::collections::{HashSet,HashMap};

use time::macros::{date, format_description};
use time::Date;
// use time::error::Parse;

static FILENAME_DATE_SEPARATOR: &str = "--";
static FILENAME_TAG_SEPARATOR: &str = "__";
static BETWEEN_TAG_SEPARATOR: &str = "_";
static CONTROLLED_VOCABULARY_FILENAME: &str = ".filetags";

pub struct TaggedFile {
    head: String,
    date_id_option: Option<Date>,
    name: String,
    tags: Vec<String>,
    exts: String,
}

impl TaggedFile {
    pub fn new(filename: &str) -> Self {
        let (head, date_str, name, tags_str, exts) = split_into_components(filename);

        // %Y%m%dT%H%M%S
        let date_format = format_description!("[year][month][day]T[hour][minute][second]");
        let date_id_option = Date::parse(date_str.as_str(), &date_format).ok();

        let tags: Vec<String> = if tags_str == "" {
            Vec::new()
        } else {
            tags_str.split(BETWEEN_TAG_SEPARATOR)
            .map(|tag_str| String::from(tag_str))
            .collect()
        };

        TaggedFile {
            head,
            date_id_option,
            name,
            tags,
            exts,
        }
    }

    pub fn is_lnk_file(&self) -> bool {
        self.exts[self.exts.len() - 3..].to_ascii_uppercase() == "LNK"
    }

    pub fn contains_tag(&self, tagname_option: Option<&str>) -> bool {
        tagname_option.map(|s| self.tags.contains(&String::from(s)))
            .unwrap_or(false)
    }

    pub fn add_tag(&mut self, tag: &str) {
        let tag_str = String::from(tag);
        if !self.contains_tag(Some(&tag_str)) {
            self.tags.push(tag_str);
        }
    }

    pub fn remove_tag(&mut self, tag: &str) {
        let tag_str = String::from(tag);
        self.tags.retain(|s| *s != tag_str);
    }
}

impl fmt::Display for TaggedFile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = String::new();
        if self.head != "" {
            output.push_str(&(self.head.clone() + "/"));
        }

        let date_format = format_description!("[year][month][day]T[hour][minute][second]");
        let date_str = self.date_id_option
            .map(|di| di.format(&date_format).ok())
            .flatten()
            .unwrap_or("".to_string());

        if date_str != "" {
            output.push_str(&(date_str + FILENAME_DATE_SEPARATOR));
        }

        output.push_str(&self.name);

        if !self.tags.is_empty() {
            output.push_str(FILENAME_TAG_SEPARATOR);
            output.push_str(&self.tags.join(BETWEEN_TAG_SEPARATOR));
        }

        output.push_str(&self.exts);

        write!(
            f,
            "{}",
            output
        )
    }
}

/// Return separate strings for a given filename.
///
/// if filename is not a Windows lnk file, the "basename without the optional .lnk
/// extension" is the same as the basname.
/*
fn split_filename(filename: &str) -> Result<(String, String, String), &str> {
    let filepath = Path::new(filename);

    let canon_filepath_result = filepath.canonicalize();

    let dir_and_basename = match canon_filepath_result {
        Ok(path_buffer) => path_buffer.as_path(),
        Err(e) => return Err("In split_filename, canonicalize threw an error: file likely does not exist"),
    };

    let dirname = dir_and_basename.parent().ok_or("I hate Rust")?;
    let basename = filepath.file_name().ok_or("I hate Rust")?;

    Ok((
        dir_and_basename.to_str().ok_or("")?.to_string(),
        <&str>::try_from(dirname)?.to_string(),
        <&str>::try_from(basename)?.to_string()
    ))
}
*/

// Returns the date, name, and tags of a file name
pub fn split_into_components(filename: &str) -> (String, String, String, String, String) {
    let filepath = Path::new(filename);

    let head_str = filepath.parent()
        .map(|p| p.to_str())
        .flatten()
        .map(|s| String::from(s))
        .unwrap_or(String::from(""));

    let basename = filepath.file_name()
        .map(|s| s.to_str())
        .flatten()
        .unwrap_or("");

    if basename == "" {
        return (head_str, String::from(""), String::from(""), String::from(""), String::from(""));
    }

    let first_dot_ndx = basename.find(".")
        .unwrap_or(basename.chars().count());

    // Get the date
    let date_end_ndx_option = basename.find(FILENAME_DATE_SEPARATOR);
    let date_end_ndx = date_end_ndx_option.unwrap_or(0);

    let date_str = String::from(&basename[..date_end_ndx]);

    // Get the name
    let name_start_ndx = date_end_ndx_option.map(|ndx| ndx + 2).unwrap_or(0);

    let name_end_ndx_option = basename[name_start_ndx..]
        .find(FILENAME_TAG_SEPARATOR)
        .map(|ndx| ndx + name_start_ndx);

    let name_end_ndx = name_end_ndx_option.unwrap_or(first_dot_ndx);

    let name_str = String::from(&basename[name_start_ndx..name_end_ndx]);

    // Get the tags
    let tags_start_ndx = name_end_ndx_option.map(|ndx| ndx + 2)
        .unwrap_or(first_dot_ndx);

    let tags_str = String::from(&basename[tags_start_ndx..first_dot_ndx]);

    let exts_str = String::from(&basename[first_dot_ndx..]);

    (head_str, date_str, name_str, tags_str, exts_str)
}

/*
pub fn extract_tags_from_filename(filename: &str) -> Vec<String> {
    let (_, _, tags) = split_into_components(filename);

    tags.split(BETWEEN_TAG_SEPARATOR)
        .map(&String::from)
        .collect()
}
*/

pub fn add_tag_to_countmap(tag: &str, tagmap: &mut HashMap<String, u32>) {
    let tag_string = String::from(tag);

    tagmap.entry(tag_string).and_modify(|count| *count += 1).or_insert(1);
}
