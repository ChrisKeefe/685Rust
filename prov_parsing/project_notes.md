# Provenance Parsing

## Basic Project Objectives

- Read provenance from one archive into json or similar
- Write CLI/Artifact API executable script from json

## PossibleFeatures

- Write executable for an arbitrary interface / API for interfaces to enable this?
- Provenance Diff tool - identify differences between two analyses
- Provenance Rerun tool
  - identify base inputs from the parent analysis (Data and its format, req'd metadata columns, classifiers, etc.)
  - Prompt user for all inputs upfront
  - Generate artifact using identical parameters
  - Possibly, prompt for select different params
- Take in multiple terminal artifacts (a directory), and script an entire analysis for rerun
  - This would make the Rerun tool more useful
  - This would require some kind of diff tool, to identify artifacts not from the same analysis
- Can we visualize all parts of an analysis at once, and export a PDF of the prov. diagram
- Visualize provenance diff as pdf
- Methods section generator -> Parse a .json and generate a sample bioinformatics methods section.

## Project Organization

- File IO
- Validation?
- Build tree from bottom to top
  - Read "parent" UUIDs from `action.yaml`
  - Use those UUIDs to traverse
- Look to q2View for cues here.

## How do different interfaces handle making executables?

Base class: executableMaker -> import this from our library
Child Classes for each Interface -> and subclass locally

## Provenance Data Organization

Top-level folder
    - data folder
    - provenance folder (Is this required? Can plugins break this part of the structure?)
    - metadata.yaml (format, uuid, type)
    - VERSION
    - checksums.md5

## Questions

Is there ever any difference between the top-level metadata.yaml and VERSION files, and the metadata.yaml and VERSION files stored in the top level of provenance?

Citations must be handled as a set to reduce duplication. Are we trusting bibtex keys to do this, or parsing data from the bibtex records themselves?

Similarly, if we want to restrict "replay" to 

How do we handle parameters that require user interaction, like rarefaction depth? Assume the scripts will be run with user intervention?

## Resources

- q2view
- unspaghetti (Ben Kaehler and Evan)
