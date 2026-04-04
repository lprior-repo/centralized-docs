---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#71-summary
chunk_level: summary
chunk_type: prose
heading: Types of volumes
token_count: 110
summary: ``` ### image FEATURE STATE: `Kubernetes v1.35 [beta]`(enabled by default) An `image` volume source represents an OCI object (a container image or artifact) which is available on the kubelet's host...
---

```
### image
FEATURE STATE:
`Kubernetes v1.35 [beta]`(enabled by default)
An `image` volume source represents an OCI object (a container image or
artifact) which is available on the kubelet's host machine.
An example of using the `image` volume source is:
[`pods/image-volumes.yaml`
](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/pods/image-volumes.yaml)![](/images/copycode.svg "Copy pods/image-volumes.yaml to clipboard")
```