---
doc_id: ops/general/docs-reference-modules
chunk_id: ops/general/docs-reference-modules#0-detailed
chunk_level: detailed
chunk_type: prose
heading: Introduction
token_count: 1035
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
Docker images to deploy your CUE configuration too.

A module is identified by a module path, which is declared in a
cue.mod/module.cue file, together with information about the module’s
dependencies. The module root directory is the directory that contains
the cue.mod directory. The main module is the module containing the
directory where the cue command is invoked.

Each package within a module is a collection of source files that are
unified together, usually all in the same directory. A package path
is the module path joined with the subdirectory containing the package,
relative to the module root.

MODULE PATHS

A module path is the canonical name for a module, declared with the
module field in the module’s cue.mod/module.cue file. A module’s
path is the prefix for package paths within the module.

A module path consists of a root path and a major version suffix,
for example in the module path myhost.example/foo@v0, the root path is
myhost.example/foo and the major version suffix is @v0.

Module paths are domain-name qualified: a module path always begins
with a host name, although that host is only a guide to the origin of
the module and is not used directly to fetch the module’s contents (see
here [/docs/reference/modules/#cue-registry-env] for details about that)). The expectation is
that any modules you create should have names that are inside domains
or namespaces that you have control of, enabling modules from different
creators to live together without conflicts in the same registry.

 * The root path is the
   portion of the module path that identifies the OCI repository within
   a registry. All versions of a module are located in that same OCI
   repository.
 * The major version suffix declares the major
   version of the module and is of the form @v1 where the version
   v1 here must match the major version of the full version it’s been
   published as.

There are also several lexical restrictions on characters allowed in
module paths. As modules are stored in OCI repositories, these correspond
