---
doc_id: ref/docs-reference-instrumentation-zpages.md/docs-reference-instrumentation-zpages
chunk_id: ref/docs-reference-instrumentation-zpages.md/docs-reference-instrumentation-zpages#3-standard
chunk_level: standard
chunk_type: prose
heading: z-pages
token_count: 192
summary: #### Note: If you request `application/json` without specifying all required parameters (`g`, `v`, and `as`), the server will respond with `406 Not Acceptable`. Example structured response: ``` `{...
---

#### Note:
If you request `application/json` without specifying all required parameters (`g`, `v`, and `as`),
the server will respond with `406 Not Acceptable`.
Example structured response:
```
`{
"kind": "Statusz",
"apiVersion": "config.k8s.io/v1alpha1",
"metadata": {
"name": "kube-apiserver"
},
"startTime": "2025-10-29T00:30:01Z",
"uptimeSeconds": 856,
"goVersion": "go1.23.2",
"binaryVersion": "1.35.0",
"emulationVersion": "1.35",
"paths": [
"/healthz",
"/livez",
"/metrics",
"/readyz",
"/statusz",
"/version"
]
}
`
```
The `config.k8s.io/v1alpha1` schema for the structured `/statusz` response is as follows: