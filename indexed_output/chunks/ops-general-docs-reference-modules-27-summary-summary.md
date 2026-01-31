---
doc_id: ops/general/docs-reference-modules
chunk_id: ops/general/docs-reference-modules#27-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 131
summary: 	// the contents of the directory (and its subdirectories) that contains. mod directory
---

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
