## Vision
Center QIIME 2's provenance tracking capacity as a key strength, by leveraging provenance to improve and simplify scientific practice. 
- Reduce human error through reliable automation of repeated analysis
- Improve the reliability of analysis reproduction by automating/validating computing environment creation
- Enable analysis troubleshooting with diff tools
- Support institutional academic honesty goals with same
- Support interface interoperability by allowing reproduction of an analysis across interfaces
- Improve attribution and simplify citation production by aggregating all relevant methods citations in a common, parseable format
- Improve methods reporting, by automating the production of plaintext method descriptions.

## Accomplishments
- specify behavior
- enumerate constraints
- specify a system
- build that system.. which we'll show you today


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


## 
Base class: executableMaker -> import this from our library
Child Classes for each Interface -> and subclass locally

## Provenance Data Organization: 
Top-level folder
    - data folder
    - provenance folder (Is this required? Can plugins break this part of the structure?)
    - metadata.yaml (format, uuid, type)
    - VERSION
    - checksums.md5


## Questions:

Citations must be handled as a set to reduce duplication. Are we trusting bibtex keys to do this, or parsing data from the bibtex records themselves?

Similarly, if we want to restrict "replay" to 

How do we handle parameters that require user interaction, like rarefaction depth? Assume the scripts will be run with user intervention?

## Resources: 
- q2view
- unspaghetti (Ben Kaehler and Evan)



## Inspirations: 
Justine: I’d love a way to cross reference provenance with PCoA. So, something
that - and I dont know how the witchcraft works or if its even possible - if I
pass a like --show-provenance flag, the artefact will propegate a set of UUIDs
and command names into the PCoA… or maybe it will generate me a metadata file
wtih the list of commands and UUIDs. …Or, possibly, it will let me give my
UUIDs human readable alias, although that has a possibillity for failure, too.
But, I think in this case and possibly others, the ability to map sample and
pattern back to specific artifact and step would be potentially useful. It at
least narrows things down

https://forum.qiime2.org/t/merging-libraries-same-dada2-perim/17150/15?u=chriskeefe
