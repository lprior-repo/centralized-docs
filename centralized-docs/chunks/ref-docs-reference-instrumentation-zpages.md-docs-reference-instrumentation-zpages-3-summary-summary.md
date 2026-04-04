---
doc_id: ref/docs-reference-instrumentation-zpages.md/docs-reference-instrumentation-zpages
chunk_id: ref/docs-reference-instrumentation-zpages.md/docs-reference-instrumentation-zpages#3-summary
chunk_level: summary
chunk_type: prose
heading: z-pages
token_count: 128
summary: * [z-pages](#z-pages) * [statusz](#statusz) * [statusz (structured)](#statusz-structured) * [flagz](#flagz) * [flagz (structured)](#flagz-structured)### statusz Enabled using the `ComponentStatusz`...
---

* [z-pages](#z-pages)
* [statusz](#statusz)
* [statusz (structured)](#statusz-structured)
* [flagz](#flagz)
* [flagz (structured)](#flagz-structured)### statusz
Enabled using the `ComponentStatusz` [feature gate](/docs/reference/command-line-tools-reference/feature-gates/#ComponentStatusz),
the `/statusz` endpoint displays high level information about the component such as its Kubernetes version, emulation version, start time and more.
The `/statusz` plain text response from the API server is similar to: