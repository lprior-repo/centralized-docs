---
doc_id: tutorial/docs-reference-encodings-kyaml.md/docs-reference-encodings-kyaml
chunk_id: tutorial/docs-reference-encodings-kyaml.md/docs-reference-encodings-kyaml#3-summary
chunk_level: summary
chunk_type: prose
heading: Getting started with KYAML
token_count: 97
summary: ### Basic Structure KYAML uses *flow style* syntax with `{}` for objects and `[]` for arrays. All string values must be **double-quoted**. ``` `--- { apiVersion: \"v1\", kind: \"Pod\", metadata: { name:...
---

### Basic Structure
KYAML uses *flow style* syntax with `{}` for objects and `[]` for arrays. All string values must be **double-quoted**.
```
`---
{
apiVersion: "v1",
kind: "Pod",
metadata: {
name: "my-pod",
labels: {
app: "demo"
},
},
spec: {
containers: [{
name: "nginx",
image: "nginx:1.20"
}]
}
}
`
```