---
doc_id: ref/docs-concepts-overview-working-with-objects.md/docs-concepts-overview-working-with-objects
chunk_id: ref/docs-concepts-overview-working-with-objects.md/docs-concepts-overview-working-with-objects#12-summary
chunk_level: summary
chunk_type: prose
heading: Understanding Kubernetes objects
token_count: 87
summary: Tools such as `kubectl` convert the information from a manifest into JSON or another supported serialization format when making the API request over HTTP. Here's an example manifest that shows the...
---

Tools such as `kubectl` convert the information from a manifest into JSON or another supported
serialization format when making the API request over HTTP.
Here's an example manifest that shows the required fields and object spec for a Kubernetes
Deployment:
[`application/deployment.yaml`
](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/application/deployment.yaml)![](/images/copycode.svg "Copy application/deployment.yaml to clipboard")