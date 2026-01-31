---
doc_id: ops/general/docs-concept-how-cue-works-with-openapi
chunk_id: ops/general/docs-concept-how-cue-works-with-openapi#3-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 140
summary: use as if they were expressed in CUE.  This allows you to work with these
---

use as if they were expressed in CUE. This allows you to work with these
constraints directly, using them to validate data, and to represent them
natively in CUE’s significantly more concise form.

In this guide, we’ll see:

 * cue def [/docs/reference/command/cue-help-def/]
   generating an OpenAPI data schema from a CUE definition,
 * cue import [/docs/reference/command/cue-help-import/]
   turning the generated OpenAPI back into CUE,
 * cue vet [/docs/reference/command/cue-help-vet/]
   using an OpenAPI data schema directly, to validate some data,
