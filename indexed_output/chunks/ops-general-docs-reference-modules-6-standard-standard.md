---
doc_id: ops/general/docs-reference-modules
chunk_id: ops/general/docs-reference-modules#6-standard
chunk_level: standard
chunk_type: table
heading: Introduction
token_count: 529
summary: A module is defined by a cue. mod directory in its root containing
---


A module is defined by a cue.mod directory in its root containing
a module.cue CUE file.


Copy code
Copied!

// module indicates the module's path.
module!: #Module

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
// from https://semver.org/spec/v2.0.0.html
#Semver: =~#"^v(?P<major>0|[1-9]\d*)\.(?P<minor>0|[1-9]\d*)\.(?P<patch>0|[1-9]\d*)(?:-(?P<prerelease>(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?(?:\+(?P<buildmetadata>[0-9a-zA-Z-]+(?:\.[0-9a-zA-Z-]+)*))?$"#
