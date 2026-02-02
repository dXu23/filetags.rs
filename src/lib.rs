
pub mod filetags;

#[cfg(test)]
mod tests {

    mod test_contains_tag {
        use crate::filetags::contains_tag;
        #[test]
        fn detects_single_tag() {
            // contains_tag should detect a tag when the file name
            // contains that singluar tag.
            assert!(contains_tag("Some-file-name__foo.jpeg", Some("foo")));
        }

        #[test]
        fn detects_single_tag_among_multiple() {
            // contains_tag should detect a tag, especially when
            // is the first tag among multiple tags.
            assert!(contains_tag("Some-file-name__foo_bar.jpeg", Some("foo")));
        }

        #[test]
        fn detects_single_tag_even_if_last() {
            // contains_tag should detect tag even if it is the last
            // tag in the tags of a file name.
            assert!(contains_tag("Some-file-name__bar_foo.jpeg", Some("foo")));
        }

        #[test]
        fn does_not_detect_tag_substring() {
            // contains_tag should not detect the tag just because it is
            // the substring of a tag in a file name.
            assert!(!contains_tag("Some-file-name__foobar.jpeg", Some("foo")));
        }

        #[test]
        fn does_not_detect_tag() {
            // contains_tag should not detect bar if it is not present
            // inside a tags list in a file name.
            assert!(!contains_tag("Some-file-name__foo.jpeg", Some("bar")));
        }

        #[test]
        fn does_not_detect_tag_2() {
            // contains_tag should not detect foo if it is not present
            // inside a tags list in a file name.
            assert!(!contains_tag("Some-foo-file-name__bar.jpeg", Some("foo")));
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
}
