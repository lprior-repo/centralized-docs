---
doc_id: ops/general/docs-reference-modules
chunk_id: ops/general/docs-reference-modules#25-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 130
summary: 	// than one major version is specified for the same module path. 	// Imports must specify the exact major version for a module path if
---

	// than one major version is specified for the same module path.
	// Imports must specify the exact major version for a module path if
	// there is more than one major version for that path and default is
	// not set for exactly one of them.
	default?: bool
}

// #Module constrains a module path. The major version indicator is
// optional, but should always be present in a normalized module.cue
// file.
#Module: =~#"^[^@]+(@v(0|[1-9]\d*))$"#

// #Semver constrains a semantic version. This regular expression is taken
