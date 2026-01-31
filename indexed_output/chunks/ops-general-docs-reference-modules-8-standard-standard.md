---
doc_id: ops/general/docs-reference-modules
chunk_id: ops/general/docs-reference-modules#8-standard
chunk_level: standard
chunk_type: prose
heading: Introduction
token_count: 519
summary: build list: they are the minimum versions that satisfy all requirements. Unlike other dependency management systems, the build list is
---

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
