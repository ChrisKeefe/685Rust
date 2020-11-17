# Some things to do

## Short-term

- Factor clone out of Config constructor for performance? (book Ch13)
- Remove .unwrap() calls, and improve error handling
- Factor data structures and parsing tools out into a separate crate?
- Basic tests. Pass a bad path, pass a non-Artifact path, pass relative or
complete filepaths

## Long-term goals

- Do we want to keep the name of our "replayed" file/files for our final output?
- Handle earlier archive versions. Right now, this is build to handle only v5.

## Initial presentation notes

This is a stand-alone, and can be run outside of a QIIME2 environment right now. Could be refactored to use QIIME 2 framework, but this got me more mileage with basic Rust tools.
