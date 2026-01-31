---
doc_id: ops/general/docs-reference-modules
chunk_id: ops/general/docs-reference-modules#37-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 143
summary:    denial of service attacks on users, proxies, and other parts of the module.  Repositories that contain more than 500 MiB of files in a module
---

   denial of service attacks on users, proxies, and other parts of the module
   ecosystem. Repositories that contain more than 500 MiB of files in a module
   directory tree should tag module versions at commits that only include files
   needed to build the module’s packages; videos, models, and other large assets
   are usually not needed for builds.
 * File modes, timestamps, and other metadata are ignored.
 * Empty directories (entries with paths ending with a slash) may be included
   in module zip files but are not extracted. The cue command does not include
