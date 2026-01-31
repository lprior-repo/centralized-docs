---
doc_id: ops/general/docs-reference-modules
chunk_id: ops/general/docs-reference-modules#35-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 145
summary: control of a Git VCS [https://git-scm. com/] repository
---

control of a Git VCS [https://git-scm.com/] repository. The git ls-files
command is then used to determine the module file list within the module root
directory. When publishing a module that is not in the repository root
directory, if the module does not have a file named LICENSE in its root
directory, cue mod publish will include the file named LICENSE from the
repository root directory at the module root. Every entry in the module file
list must be “clean” with respect to the current commit.

The initial list of files determined by the source is then filtered according
