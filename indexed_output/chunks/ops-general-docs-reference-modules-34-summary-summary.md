---
doc_id: ops/general/docs-reference-modules
chunk_id: ops/general/docs-reference-modules#34-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 145
summary: from the zip file.  The latter enables fast access to the dependency information
---

from the zip file. The latter enables fast access to the dependency information
without the need to download the entire module archive.

DETERMINING ZIP FILE CONTENTS

The source field in module.cue is used by cue mod publish to determine
which files to include in a module zip. It is required when publishing a module.
The source.kind field specifies the kind of source. The supported kinds are
listed below.

source: kind: "self" determines the module file list from the module root
directory tree on disk.

source: kind: "git" requires that the module root directory be under the
