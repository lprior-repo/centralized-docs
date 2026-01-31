---
doc_id: ops/general/docs-reference-modules
chunk_id: ops/general/docs-reference-modules#7-standard
chunk_level: standard
chunk_type: prose
heading: Introduction
token_count: 525
summary: // #Source describes a source of truth for a module's content. 	// kind specifies the kind of source
---


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
module. At the end of the traversal, the highest required versions comprise the
