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
        fn find_tags_similar_to_Simpson() {
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
}
