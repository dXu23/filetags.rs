pub mod filetags;

#[cfg(test)]
mod tests {

    mod test_contains_tag {
        use crate::filetags::TaggedFile;
        #[test]
        fn detects_tag_foo() {
            // contains_tag should detect a tag when the file name
            // contains that singluar tag.
            let tagged_file = TaggedFile::new("Some-file-name__foo.jpeg");
            assert!(tagged_file.contains_tag(Some("foo")));
        }

        #[test]
        fn detects_tag_foo_among_multiple() {
            // contains_tag should detect a tag, especially when
            // is the first tag among multiple tags.
            let tagged_file = TaggedFile::new("Some-file-name__foo_bar.jpeg");
            assert!(tagged_file.contains_tag(Some("foo")));
        }

        #[test]
        fn detects_single_tag_foo_even_if_last() {
            // contains_tag should detect tag even if it is the last
            // tag in the tags of a file name.
            let tagged_file = TaggedFile::new("Some-file-name__bar_foo.jpeg");
            assert!(tagged_file.contains_tag(Some("foo")));
        }

        #[test]
        fn does_not_detect_tag_substring_foo() {
            // contains_tag should not detect the tag just because it is
            // the substring of a tag in a file name.
            let tagged_file = TaggedFile::new("Some-file-name__foobar.jpeg");
            assert!(!tagged_file.contains_tag(Some("foo")));
        }

        #[test]
        fn does_not_detect_tag_bar() {
            // contains_tag should not detect bar if it is not present
            // inside a tags list in a file name.
            let tagged_file = TaggedFile::new("Some-file-name__foo.jpeg");
            assert!(!tagged_file.contains_tag(Some("bar")));
        }

        #[test]
        fn does_not_detect_tag_foo() {
            // contains_tag should not detect foo if it is not present
            // inside a tags list in a file name.
            let tagged_file = TaggedFile::new("Some-foo-file-name__bar.jpeg");
            assert!(!tagged_file.contains_tag(Some("foo")));
        }
    }

        // without tagname -> check if any tags are found:;
        /*
        assert!(contains_tag("Some-file-name__foo.jpeg", None));
        assert!(contains_tag("Some-file-name__foo_bar.jpeg", None));
        assert!(!contains_tag("Some-file-name.jpeg", None));
        */

        // ignoring Windows .lnk extension as extension:;
        /*
        assert!(contains_tag("Some file name--foo.jpeg.lnk", "foo"));
        assert!(contains_tag("Some file name--foo bar.jpeg.lnk", "foo"));
        assert!(contains_tag("Some file name--bar foo.jpeg.lnk", "foo"));
        assert!(contains_tag("Some file name--foobar.jpeg.lnk", "foo"));
        assert!(contains_tag("Some file name--foo.jpeg.lnk", "bar"));
        assert!(contains_tag("Some foo file name--bar.jpeg.lnk", "foo"));
        // without tagname -> check if any tags are found:;
        assert!(contains_tag("Some file name--foo.jpeg.lnk"));
        assert!(contains_tag("Some file name--foo bar.jpeg.lnk"));
        assert!(contains_tag("Some file name.jpeg.lnk"));
        */

    mod test_add_tag {
        use crate::filetags::TaggedFile;
        use std::fmt::Write;

        #[test]
        fn adds_tag_bar() {
            let mut output = String::new();
            let mut tagged_file = TaggedFile::new("Some-file-name.jpeg");
            tagged_file.add_tag("bar");
            write!(&mut output, "{}", tagged_file).expect("Failed to write to output");
            assert_eq!("Some-file-name__bar.jpeg", output);
        }

        #[test]
        fn adds_second_tag_bar() {
            let mut output = String::new();
            let mut tagged_file = TaggedFile::new("Some-file-name__foo.jpeg");
            tagged_file.add_tag("bar");
            write!(&mut output, "{}", tagged_file).expect("Failed to write to output");
            assert_eq!("Some-file-name__foo_bar.jpeg", output);
        }

        #[test]
        fn adds_tag_foo_once() {
            let mut output = String::new();
            let mut tagged_file = TaggedFile::new("Some-file-name__foo.jpeg");
            tagged_file.add_tag("foo");
            write!(&mut output, "{}", tagged_file).expect("Failed to write to output");
            assert_eq!("Some-file-name__foo.jpeg", output);
        }
    }

    mod test_remove_tag {
        use crate::filetags::TaggedFile;
        use std::fmt::Write;

        #[test]
        fn removes_tag_bar() {
            let mut output = String::new();
            let mut tagged_file = TaggedFile::new("Some-file-name__bar.jpeg");
            tagged_file.remove_tag("bar");
            write!(&mut output, "{}", tagged_file).expect("Failed to write to output");
            assert_eq!("Some-file-name.jpeg", output);
        }

        #[test]
        fn removes_tag_bar_among_multiple_tags() {
            let mut output = String::new();
            let mut tagged_file = TaggedFile::new("Some-file-name__foo_bar.jpeg");
            tagged_file.remove_tag("bar");
            write!(&mut output, "{}", tagged_file).expect("Failed to write to output");
            assert_eq!("Some-file-name__foo.jpeg", output);
        }

        #[test]
        fn does_not_remove_nonexistent_tag_foo() {
            let mut output = String::new();
            let mut tagged_file = TaggedFile::new("Some-file-name__bar.jpeg");
            tagged_file.remove_tag("foo");
            write!(&mut output, "{}", tagged_file).expect("Failed to write to output");
            assert_eq!("Some-file-name__bar.jpeg", output);
        }
    }

    mod test_add_tag_to_countmap {
        use crate::filetags::add_tag_to_countmap;
        use std::collections::HashMap;

        #[test]
        fn adds_entry_tag() {
            let mut tags_expected: HashMap<String, u32> = HashMap::new();
            tags_expected.insert("tag".to_string(), 1);

            let mut tags_actual = HashMap::new();

            add_tag_to_countmap("tag", &mut tags_actual);
            assert_eq!(tags_expected, tags_actual);
        }

        #[test]
        fn increments_entry_tag_if_already_exists() {
            let mut tags_expected: HashMap<String, u32> = HashMap::new();
            tags_expected.insert("tag".to_string(), 1);

            let mut tags_actual = HashMap::new();
            tags_actual.insert("tag".to_string(), 0);

            add_tag_to_countmap("tag", &mut tags_actual);
            assert_eq!(tags_expected, tags_actual);
        }

        #[test]
        fn increments_entry_tag_if_one() {
            let mut tags_expected: HashMap<String, u32> = HashMap::new();
            tags_expected.insert("tag".to_string(), 2);

            let mut tags_actual = HashMap::new();
            tags_actual.insert("tag".to_string(), 1);

            add_tag_to_countmap("tag", &mut tags_actual);
            assert_eq!(tags_expected, tags_actual);
        }

        #[test]
        fn adds_new_tag() {
            let mut tags_expected: HashMap<String, u32> = HashMap::new();
            tags_expected.insert("oldtag".to_string(), 1);
            tags_expected.insert("newtag".to_string(), 1);

            let mut tags_actual = HashMap::new();
            tags_actual.insert("oldtag".to_string(), 1);

            add_tag_to_countmap("newtag", &mut tags_actual);
            assert_eq!(tags_expected, tags_actual);
        }

        #[test]
        fn adds_new_tag_when_old_tag_is_two() {
            let mut tags_expected: HashMap<String, u32> = HashMap::new();
            tags_expected.insert("oldtag".to_string(), 2);
            tags_expected.insert("newtag".to_string(), 1);

            let mut tags_actual = HashMap::new();
            tags_actual.insert("oldtag".to_string(), 2);

            add_tag_to_countmap("newtag", &mut tags_actual);
            assert_eq!(tags_expected, tags_actual);
        }
    }

    mod test_find_similar_tags {
        use crate::filetags::find_similar_tags;

        #[test]
        fn finds_no_similar_tags_when_different() {
            let tag_list = [
                "foobar", "bar", "baz", "Frankenstein", "parabol",
                "Bah", "paR", "por", "Schneewittchen"
            ];

            let tags_list_expected: Vec<String> = Vec::new();
            let tags_list_actual = find_similar_tags("xxx", &tag_list);
            assert_eq!(tags_list_expected, tags_list_actual);
        }

        // This test might need to be changed since get_close_matches doesn't
        // seem to preserve order.
        #[test]
        fn find_tags_similar_to_simpson() {
            let tag_list = [
                "foobar", "Simson", "simpson", "Frankenstein", "sumpson",
                "Simpso", "impson", "mpson", "Schneewittchen"
            ];

            let tags_list_expected: Vec<String> = vec![
                "Simpso".to_string(), "Simson".to_string(), "impson".to_string(),
                "simpson".to_string(), "mpson".to_string(), "sumpson".to_string()
            ];

            let tags_list_actual = find_similar_tags("Simpson", &tag_list);
            assert_eq!(tags_list_expected, tags_list_actual);
        }
    }

    mod test_possible_shortcuts {
        use crate::filetags::possible_shortcuts;
        use std::collections::HashSet;

        #[test]
        fn adds_bar() {
            let user_tags = ["bar"];
            let shortcut_tags = ["Frankenstein", "Schneewittchen"];

            let mut expected_tags: HashSet<String> = HashSet::new();
            expected_tags.insert("bar".to_string());

            let actual_tags = possible_shortcuts(&user_tags, &shortcut_tags);

            assert_eq!(expected_tags, actual_tags);
        }

        #[test]
        fn adds_third_and_fourth_tags() {
            let user_tags = ["34"];
            let shortcut_tags = ["Frankenstein", "Schneewittchen", "baz", "bar"];

            let mut expected_tags: HashSet<String> = HashSet::new();
            expected_tags.insert("baz".to_string());
            expected_tags.insert("bar".to_string());

            let actual_tags = possible_shortcuts(&user_tags, &shortcut_tags);

            assert_eq!(expected_tags, actual_tags);
        }

        #[test]
        fn adds_first_and_second_tags() {
            let user_tags = ["12"];
            let shortcut_tags = ["Frankenstein", "Schneewittchen", "baz", "bar"];

            let mut expected_tags: HashSet<String> = HashSet::new();
            expected_tags.insert("Frankenstein".to_string());
            expected_tags.insert("Schneewittchen".to_string());

            let actual_tags = possible_shortcuts(&user_tags, &shortcut_tags);

            assert_eq!(expected_tags, actual_tags);
        }

        #[test]
        fn adds_numbers_as_tag_when_out_of_range() {
            let user_tags = ["59"];
            let shortcut_tags = ["Frankenstein", "Schneewittchen", "baz", "bar"];

            let mut expected_tags: HashSet<String> = HashSet::new();
            expected_tags.insert("59".to_string());

            let actual_tags = possible_shortcuts(&user_tags, &shortcut_tags);

            assert_eq!(expected_tags, actual_tags);
        }

        #[test]
        fn adds_both_tags_and_numbers() {
            let user_tags = ["baz", "12", "88"];
            let shortcut_tags = ["Frankenstein", "Schneewittchen", "baz", "bar"];

            let mut expected_tags: HashSet<String> = HashSet::new();
            expected_tags.insert("baz".to_string());
            expected_tags.insert("Frankenstein".to_string());
            expected_tags.insert("Schneewittchen".to_string());
            expected_tags.insert("88".to_string());

            let actual_tags = possible_shortcuts(&user_tags, &shortcut_tags);

            assert_eq!(expected_tags, actual_tags);
        }

        #[test]
        fn adds_both_tags_and_numbers_2() {
            let user_tags = ["19", "88", "baz"];
            let shortcut_tags = ["Frankenstein", "Schneewittchen", "baz", "bar"];

            let mut expected_tags: HashSet<String> = HashSet::new();
            expected_tags.insert("19".to_string());
            expected_tags.insert("88".to_string());
            expected_tags.insert("baz".to_string());

            let actual_tags = possible_shortcuts(&user_tags, &shortcut_tags);

            assert_eq!(expected_tags, actual_tags);
        }
    }

    mod test_most_common_keys {
        use std::collections::{HashSet, HashMap};
        use crate::filetags::most_common_keys;

        #[test]
        pub fn sorts_keys_by_value() {
            let mut tag_counts: HashMap<String, usize> = HashMap::new();
            tag_counts.insert("key2".to_string(), 45);
            tag_counts.insert("key1".to_string(), 33);

            let expected_keys = vec![
                "key2".to_string(),
                "key1".to_string()
            ];

            let actual_keys = most_common_keys(
                tag_counts,
                9,
                HashSet::new(),
                HashSet::new()
            );

            assert_eq!(expected_keys, actual_keys);
        }

        #[test]
        fn yields_top_nine_keys_sorted() {
            let tag_counts: HashMap<String, usize> = HashMap::from([
                ("key1".to_string(), 45usize),
                ("key2".to_string(), 33usize),
                ("key3".to_string(), 3usize),
                ("key4".to_string(), 1usize),
                ("key5".to_string(), 5usize),
                ("key6".to_string(), 159usize),
                ("key7".to_string(), 0usize),
                ("key8".to_string(), 999usize),
                ("key9".to_string(), 42usize),
                ("key10".to_string(), 4242usize),
            ]);

            let expected_keys = vec![
                "key10".to_string(), "key8".to_string(), "key6".to_string(),
                "key1".to_string(), "key9".to_string(), "key2".to_string(),
                "key5".to_string(), "key3".to_string(), "key4".to_string(), 
            ];

            let actual_keys = most_common_keys(
                tag_counts,
                9,
                HashSet::new(),
                HashSet::new()
            );

            assert_eq!(expected_keys, actual_keys);
        }

        #[test]
        fn omits_tags_from_list() {
            let tag_counts: HashMap<String, usize> = HashMap::from([
                ("key1".to_string(), 45usize),
                ("key2".to_string(), 33usize),
                ("key3".to_string(), 3usize),
                ("key4".to_string(), 1usize),
                ("key5".to_string(), 5usize),
                ("key6".to_string(), 159usize),
                ("key7".to_string(), 0usize),
                ("key8".to_string(), 999usize),
                ("key9".to_string(), 42usize),
                ("key10".to_string(), 4242usize),
                ("key11".to_string(), 1234usize),
                ("key12".to_string(), 1234usize),
                ("key13".to_string(), 1234usize),
                ("key14".to_string(), 1234usize),
            ]);

            let tags_to_omit = HashSet::from([
                "key11", "key3",
                "key7", "key14"
            ]);

            let expected_keys = vec![
                "key10".to_string(), "key12".to_string(), "key13".to_string(),
                "key8".to_string(), "key6".to_string(), "key1".to_string(),
                "key9".to_string(), "key2".to_string(), "key5".to_string(), 
            ];

            let actual_keys = most_common_keys(
                tag_counts,
                9,
                tags_to_omit,
                HashSet::new()
            );

            assert_eq!(expected_keys, actual_keys);
        }
    }

    mod test_common_tags {
        use crate::filetags::common_tags;
        use std::collections::HashSet;

        #[test]
        fn returns_no_tags_when_none_present() {
            let file_names = ["file1.txt"];
            let expected_tags: HashSet<String> = HashSet::new();
            let actual_tags = common_tags(&file_names);

            assert_eq!(expected_tags, actual_tags);
        }

        #[test]
        fn returns_foo_when_foo_present() {
            let file_names = ["file1__foo.txt"];
            let expected_tags: HashSet<String> = HashSet::from(["foo".to_string()]);
            let actual_tags = common_tags(&file_names);

            assert_eq!(expected_tags, actual_tags);
        }

        #[test]
        fn returns_all_tags_of_single_file() {
            let file_names = ["file1__foo_bar.txt"];
            let expected_tags: HashSet<String> = HashSet::from([
                "foo".to_string(),
                "bar".to_string()
            ]);

            let actual_tags = common_tags(&file_names);

            assert_eq!(expected_tags, actual_tags);
        }

        #[test]
        fn returns_tags_of_one_file_if_other_contains_none() {
            let file_names = ["file1__foo.txt", "file2.txt"];
            let expected_tags: HashSet<String> = HashSet::new();

            let actual_tags = common_tags(&file_names);

            assert_eq!(expected_tags, actual_tags);
        }

        #[test]
        fn returns_foo_when_foo_only_common_tag_between_two_files() {
            let file_names = ["file1__foo.txt", "file2__foo_bar.txt"];
            let expected_tags: HashSet<String> = HashSet::from(["foo".to_string()]);

            let actual_tags = common_tags(&file_names);

            assert_eq!(expected_tags, actual_tags);
        }

        #[test]
        fn returns_foo_when_foo_only_common_tag() {
            let file_names = [
                "file1__baz_foo.txt",
                "file2__foo_bar.txt",
                "file3__foo_bar_baz.txt",
                "file4__foo_bar_jodel.txt"
            ];

            let expected_tags: HashSet<String> = HashSet::from(["foo".to_string()]);

            let actual_tags = common_tags(&file_names);

            assert_eq!(expected_tags, actual_tags);
        }

        #[test]
        fn returns_foo_and_common_when_both_common_tags() {
            let file_names = [
                "file1__common_baz_foo.txt",
                "file2__common_foo_bar.txt",
                "file3__common_foo_bar_baz.txt",
                "file4__common_foo_bar_jodel.txt"
            ];

            let expected_tags: HashSet<String> = HashSet::from([
                "foo".to_string(),
                "common".to_string()
            ]);

            let actual_tags = common_tags(&file_names);

            assert_eq!(expected_tags, actual_tags);
        }
    }

    mod test_extract_tags_from_path {
        use std::path::Path;
        use std::collections::HashSet;
        use crate::filetags::extract_tags_from_path;

        #[test]
        fn extracts_no_tags_from_path_without_tags() {
            let expected_tags: HashSet<String> = HashSet::new();

            let path = Path::new("/a/path/without/tags");
            let actual_tags = extract_tags_from_path(&path);

            assert_eq!(expected_tags, actual_tags);
        }

        #[test]
        fn extracts_tags_from_path_where_tail_has_no_tag() {
            let expected_tags: HashSet<String> = HashSet::from([
                "ptag1".to_string(),
                "ptag2".to_string()
            ]);

            let path = Path::new("/path__ptag1/with__ptag1_ptag2/tags");
            let actual_tags = extract_tags_from_path(&path);

            assert_eq!(expected_tags, actual_tags);
        }

        #[test]
        fn extracts_tag_from_path_where_every_component_has_tags() {
            let expected_tags: HashSet<String> = HashSet::from([
                "ptag1".to_string(),
                "ptag2".to_string(),
                "ftag1".to_string()
            ]);

            let path = Path::new("/path__ptag1/with__ptag1_ptag2/tags__ftag1");
            let actual_tags = extract_tags_from_path(&path);

            assert_eq!(expected_tags, actual_tags);
        }
    }
}
