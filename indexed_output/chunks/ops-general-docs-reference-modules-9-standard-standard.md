---
doc_id: ops/general/docs-reference-modules
chunk_id: ops/general/docs-reference-modules#9-standard
chunk_level: standard
chunk_type: prose
heading: Introduction
token_count: 517
summary: DETERMINING ZIP FILE CONTENTS. The source field in module
---


DETERMINING ZIP FILE CONTENTS

The source field in module.cue is used by cue mod publish to determine
which files to include in a module zip. It is required when publishing a module.
The source.kind field specifies the kind of source. The supported kinds are
listed below.

source: kind: "self" determines the module file list from the module root
directory tree on disk.

source: kind: "git" requires that the module root directory be under the
control of a Git VCS [https://git-scm.com/] repository. The git ls-files
command is then used to determine the module file list within the module root
directory. When publishing a module that is not in the repository root
directory, if the module does not have a file named LICENSE in its root
directory, cue mod publish will include the file named LICENSE from the
repository root directory at the module root. Every entry in the module file
list must be “clean” with respect to the current commit.

The initial list of files determined by the source is then filtered according
to file path and size constraints [/docs/reference/modules/#zip-path-size-constraints].

FILE PATH AND SIZE CONSTRAINTS

There are a number of restrictions on the content of module zip files. These
constraints ensure that zip files can be extracted safely and consistently on
a wide range of platforms.

 * A module zip file may be at most 500 MiB in size. The total uncompressed size
   of its files is also limited to 500 MiB. module.cue files are limited to 16 MiB.
   LICENSE files are also limited to 16 MiB. These limits exist to mitigate
   denial of service attacks on users, proxies, and other parts of the module
   ecosystem. Repositories that contain more than 500 MiB of files in a module
   directory tree should tag module versions at commits that only include files
   needed to build the module’s packages; videos, models, and other large assets
   are usually not needed for builds.
 * File modes, timestamps, and other metadata are ignored.
 * Empty directories (entries with paths ending with a slash) may be included
