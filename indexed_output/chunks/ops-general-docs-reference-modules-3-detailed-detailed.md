---
doc_id: ops/general/docs-reference-modules
chunk_id: ops/general/docs-reference-modules#3-detailed
chunk_level: detailed
chunk_type: table
heading: Introduction
token_count: 1041
summary: A module is defined by a cue. mod directory in its root containing
---


CUE.MOD/MODULE.CUE FILES

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

// #Source describes a source of truth for a module's content.
#Source: {
	// kind specifies the kind of source.
	//
	// The special value "self" signifies a module is stand-alone, associated
	// with no particular source. The module's file list is determined from
	// the contents of the directory (and its subdirectories) that contains
	// the cue.mod directory.
	//
	// See https://cuelang.org/docs/reference/modules/#determining-zip-file-contents
	// for details on all the possible values for kind, and how they relate
	// to determining the list of files in a module.
	kind!: "self" | "git"
}

For example:


Copy code
Copied!

language: version: "v0.4.3"

module: "foo.example/my/thing@v1"

deps: {
	"foo.example/other/thing@v1": v: "v1.0.2"
	"foo.example/new/thing@v2": v:   "v2.3.4"
}

The module.cue file is designed to be human readable and machine writable. The
cue command will provide several subcommands that manipulate cue.mod/module.cue files.
For now, the only one is cue mod tidy which will fetch dependencies
and canonicalize the module.cue file to reflect all the most recent versions.

A cue.mod/module.cue file is required for all modules.

MINIMAL VERSION SELECTION (MVS)

CUE uses an algorithm called Minimal version selection (MVS) to select
a set of module versions to use when building packages. MVS is described in
detail in Minimal Version Selection [https://research.swtch.com/vgo-mvs] by
Russ Cox.

Conceptually, MVS operates on a directed graph of modules, specified with
module.cue files [/docs/reference/modules/#glos-cue-mod-file]. Each vertex in the graph represents a
module version. Each edge represents a minimum required version of a dependency,
specified with an entry in the deps field.

MVS produces the build list [/docs/reference/modules/#glos-build-list] as output, the list of module
versions used for an evaluation.

MVS starts at the main modules (special vertices in the graph that have no
version) and traverses the graph, tracking the highest required version of each
