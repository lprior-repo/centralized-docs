---
doc_id: ops/general/docs-reference-modules
chunk_id: ops/general/docs-reference-modules#23-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 141
summary: // version indicates the language version used by the code in this module. // - the minimum version of CUE required to evaluate the code in this
---


// version indicates the language version used by the code in this module
// - the minimum version of CUE required to evaluate the code in this
// module. When a later version of CUE is evaluating code in this module,
// this will be used to choose version-specific behavior. If an earlier
// version of CUE is used, an error will be given.
language?: version?: #Semver

// source holds information about the source of the files within the
// module. This field is mandatory at publish time.
source?: #Source

// description describes the purpose of this module.
