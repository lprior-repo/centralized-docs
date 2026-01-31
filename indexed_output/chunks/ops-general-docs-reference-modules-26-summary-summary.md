---
doc_id: ops/general/docs-reference-modules
chunk_id: ops/general/docs-reference-modules#26-summary
chunk_level: summary
chunk_type: table
heading: Introduction
token_count: 141
summary: // from https://semver. org/spec/v2
---

// from https://semver.org/spec/v2.0.0.html
#Semver: =~#"^v(?P<major>0|[1-9]\d*)\.(?P<minor>0|[1-9]\d*)\.(?P<patch>0|[1-9]\d*)(?:-(?P<prerelease>(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?(?:\+(?P<buildmetadata>[0-9a-zA-Z-]+(?:\.[0-9a-zA-Z-]+)*))?$"#

// #Source describes a source of truth for a module's content.
#Source: {
	// kind specifies the kind of source.
	//
	// The special value "self" signifies a module is stand-alone, associated
	// with no particular source. The module's file list is determined from
