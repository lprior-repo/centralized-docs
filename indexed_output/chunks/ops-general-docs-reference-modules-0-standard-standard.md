---
doc_id: ops/general/docs-reference-modules
chunk_id: ops/general/docs-reference-modules#0-standard
chunk_level: standard
chunk_type: prose
heading: Introduction
token_count: 521
summary: # CUE Modules | CUE. **Source:** https://cuelang
---

# CUE Modules | CUE

**Source:** https://cuelang.org/docs/reference/modules/

Skip to content

Homepage of CUE [/]
 * Documentation [/docs/]
 * Play [/play/]
 * Community [/community/]

 * 
   GitHub [https://github.com/cue-lang/cue]
 * 
   Slack [/s/slack]
 * 
   Discord [/s/discord]
 * 
   X (Twitter) [https://twitter.com/cue_lang]
 * 
   Bluesky [https://bsky.app/profile/cuelang.org]
 * 
   YouTube [https://www.youtube.com/@cuelang/videos]

Install
[/docs/introduction/installation/]

Search [/search]

What are you looking for?

Menu

 1. References [https://cuelang.org/docs/reference/]


 2. CUE MODULES

rogpeppe [https://github.com/rogpeppe.png]
Roger Peppe
rogpeppe [https://github.com/rogpeppe.png]
Roger Peppe

Github profile

[https://github.com/rogpeppe]

Search all content by this author

[/search/?q=author:rogpeppe]
 * modules [/search?q=tag:modules]

INTRODUCTION

Modules are how CUE manages dependencies.
This document is a detailed reference manual for CUE’s module system.
CUE’s modules support has a lot in common with Go’s modules
and this document has substantial parts that have been taken
directly from the Go modules reference [https://go.dev/ref/mod].
Thanks very much to Russ Cox and the Go team for their
amazing work there.

This document largely supercedes the
prior modules documentation [/docs/concept/modules-packages-instances/]
although, as a transitionary measure, the CUE tool still supports
the import of packages present in the cue.mod/pkg, cue.mod/usr and
cue.mod/gen directories. This only applies to the main module, and if
there is any ambiguity with respect to regular module dependencies an
“ambiguous import” error will be reported.

MODULES, PACKAGES, AND VERSIONS

A module is a collection of packages that are released,
versioned, and distributed together. Modules are downloaded from
OCI-compliant [https://github.com/opencontainers/distribution-spec/blob/main/spec.md]
artifact registries. This means that if you are deploying CUE to the cloud,
you can use the same distribution mechanism that you might be using for
