---
doc_id: ops/general/docs-reference-modules
chunk_id: ops/general/docs-reference-modules#1-standard
chunk_level: standard
chunk_type: prose
heading: Introduction
token_count: 514
summary: Docker images to deploy your CUE configuration too. A module is identified by a module path, which is declared in a
---

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
