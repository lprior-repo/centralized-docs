---
doc_id: ops/general/docs-reference-modules
chunk_id: ops/general/docs-reference-modules#33-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 130
summary: A module is stored in a registry with a top level manifest with media type. application/vnd
---


A module is stored in a registry with a top level manifest with media type
application/vnd.oci.image.manifest.v1+json and artifact type
application/vnd.cue.module.v1+json, that points to two blobs.
The first blob (also known as a “layer 0” although there’s actually
no layering going on here) has media type application/zip and holds the full contents
of the module. The second blob, layer 1, has media type application/vnd.cue.modulefile.v1
and stores an exact copy of the contents of the cue.mod/module.cue file
