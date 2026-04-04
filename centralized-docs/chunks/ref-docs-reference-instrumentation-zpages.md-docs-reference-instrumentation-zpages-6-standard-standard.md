---
doc_id: ref/docs-reference-instrumentation-zpages.md/docs-reference-instrumentation-zpages
chunk_id: ref/docs-reference-instrumentation-zpages.md/docs-reference-instrumentation-zpages#6-standard
chunk_level: standard
chunk_type: prose
heading: z-pages
token_count: 379
summary: #### Note: If you request `application/json` without specifying all required parameters (`g`, `v`, and `as`), the server will respond with `406 Not Acceptable`. Example structured response: ``` `{...
---

#### Note:
If you request `application/json` without specifying all required parameters (`g`, `v`, and `as`),
the server will respond with `406 Not Acceptable`.
Example structured response:
```
`{
"kind": "Flagz",
"apiVersion": "config.k8s.io/v1alpha1",
"metadata": {
"name": "kube-apiserver"
},
"flags": {
"advertise-address": "192.168.8.4",
"allow-privileged": "true",
"anonymous-auth": "true",
"authorization-mode": "[Node,RBAC]",
"enable-priority-and-fairness": "true",
"profiling": "true",
"default-watch-cache-size": "100"
}
}
`
```
The `config.k8s.io/v1alpha1` schema for the structured `/flagz` response is as follows:
```
`// Flagz is the config.k8s.io/v1alpha1 schema for the /flagz endpoint.
type Flagz struct {
// Kind is "Flagz".
Kind string `json:"kind"`
// APIVersion is the version of the object, e.g., "config.k8s.io/v1alpha1".
APIVersion string `json:"apiVersion"`
// Standard object's metadata.
// +optional
Metadata metav1.ObjectMeta `json:"metadata,omitempty"`
// Flags contains the command-line flags and their values.
// The keys are the flag names and the values are the flag values,
// possibly with confidential values redacted.
// +optional
Flags map[string]string `json:"flags,omitempty"`
}
`
```
#### Note:
The structured responses for both `/statusz` and `/flagz` are alpha features in v1.35
and are subject to change in future releases.
They are intended to provide machine-parseable output for debugging and introspection tools.