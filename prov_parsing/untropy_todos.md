# Some things to do

## Short-term

- build a tree
- Write data to yaml file
- Handle struct API better, so everything doesn't need to be public
- Remove .clone() calls where possible (book Ch13)
- Remove .unwrap() calls, and improve error handling
- Basic tests. Pass a bad path, pass a non-Artifact path, pass relative or
complete filepaths

## Long-term goals

- Do we want to keep the name of our "replayed" file/files for our final output?
- Handle earlier archive versions. Right now, this is built to handle only v5.
