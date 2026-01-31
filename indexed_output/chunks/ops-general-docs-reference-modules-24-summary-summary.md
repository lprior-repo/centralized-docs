---
doc_id: ops/general/docs-reference-modules
chunk_id: ops/general/docs-reference-modules#24-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 141
summary: description?: string. // deps holds dependency information for modules, keyed by module path
---

description?: string

// deps holds dependency information for modules, keyed by module path.
deps?: [#Module]: #Dep

// custom holds arbitrary data intended for use by third-party tools.
// Each field at the top level represents a tooling namespace,
// conventionally a module or domain name. Data migrated from legacy
// module.cue files is placed in the "legacy" namespace.
custom?: [#Module | "legacy"]: [_]: _

#Dep: {
	// v indicates the minimum required version of the module.
	v!: #Semver

	// default indicates this module is used as a default in case more
