## Project Organization

- File IO
- Validation?
- Build tree from bottom to top
    - Read "parent" UUIDs from `action.yaml`
    - Use those UUIDs to traverse
- Look to q2View for cues here.

## Provenance Data Organization: 
Top-level folder
    - data folder
    - provenance folder (Is this required? Can plugins break this part of the structure?)
    - metadata.yaml (format, uuid, type)
    - VERSION
    - checksums.md5


## Questions:

Citations must be handled as a set to reduce duplication. Are we trusting bibtex keys to do this, or parsing data from the bibtex records themselves?
