# Some things to do

## Short-term

- build a tree
- Write data to yaml file
- Handle struct API better, so everything doesn't need to be public
- Factor clone out of Config constructor for performance? (book Ch13)
- Remove .unwrap() calls, and improve error handling
- Basic tests. Pass a bad path, pass a non-Artifact path, pass relative or
complete filepaths

## Long-term goals

- Do we want to keep the name of our "replayed" file/files for our final output?
- Handle earlier archive versions. Right now, this is build to handle only v5.

## Initial presentation notes

Big-picture:
Goals

What it does:
    - modular structure
    - basic error reporting

What it doesn't do
    - handle different archive formats
    - testing
    - "style" - lots of clone and unwrap

Lots of hackery.

zip:
produces a ZipArchive full of ZipFiles which implement the Read trait.
This means all we have to do is iterate over the ZipArchive and read in files

Serde:

- You build the structs, SerDe fills em up.
- Serde brings its own useful types (Value can represent any serial structure)
- Serde recognizes your types, including Type Aliases (`type UUID = String;`)
- Keyword Aliasing - saves your butt if your serial data structure uses Rust keywords for keys

What we've achieved here

What remains for this MVP

Remaining questions

How this might be useful:

- This is a stand-alone, and can be run outside of a QIIME2 environment right now. Could be refactored to use QIIME 2 framework, but this got me more mileage with basic Rust tools.
- After _much_ refactoring, compiling to WASM could let us do cool things in-browser.
  - Diff provenance trees
  - Display "nested" provenance - other objects produced inside of pipelines
- Union citations and output a single .bibtex or other format
- Produce trees from complete analyses, which can be used to reproduces complete analyses.