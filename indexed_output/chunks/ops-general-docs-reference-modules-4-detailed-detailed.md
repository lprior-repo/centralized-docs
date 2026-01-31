---
doc_id: ops/general/docs-reference-modules
chunk_id: ops/general/docs-reference-modules#4-detailed
chunk_level: detailed
chunk_type: prose
heading: Introduction
token_count: 1037
summary:  At the end of the traversal, the highest required versions comprise the. build list: they are the minimum versions that satisfy all requirements
---

module. At the end of the traversal, the highest required versions comprise the
build list: they are the minimum versions that satisfy all requirements.

Unlike other dependency management systems, the build list is
not saved in a “lock” file. MVS is deterministic, and the build list doesn’t
change when new versions of dependencies are released, so MVS is used to compute
it at the beginning of every module-aware command.

Module version graph with visited versions highlighted [buildlist.svg]
buildlist.svgModule version graph with visited versions highlightedConsider the example in this diagram. The main module requires module A
at version 1.2 or higher and module B at version 1.2 or higher. A 1.2 and B 1.2
require C 1.3 and C 1.4, respectively. C 1.3 and C 1.4 both require D 1.2.

MVS visits and loads the cue.mod/module.cue file for each of the module versions
highlighted in blue. At the end of the graph traversal, MVS returns a build list
containing the bolded versions: A 1.2, B 1.2, C 1.4, and D 1.2. Note that higher
versions of B and D are available but MVS does not select them, since nothing
requires them.

MODULE STORAGE FORMAT

Modules are stored in a registry using a standard manifest + blob
format. There is rarely any need to
interact directly with these artifacts, since the cue command creates, downloads,
and extracts them automatically from registries. However, it’s still useful to know about these
files to understand cross-platform compatibility constraints.

A module is stored in a registry with a top level manifest with media type
application/vnd.oci.image.manifest.v1+json and artifact type
application/vnd.cue.module.v1+json, that points to two blobs.
The first blob (also known as a “layer 0” although there’s actually
no layering going on here) has media type application/zip and holds the full contents
of the module. The second blob, layer 1, has media type application/vnd.cue.modulefile.v1
and stores an exact copy of the contents of the cue.mod/module.cue file
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
