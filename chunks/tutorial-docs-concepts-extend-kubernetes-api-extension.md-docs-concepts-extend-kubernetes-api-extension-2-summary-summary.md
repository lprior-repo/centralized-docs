---
doc_id: tutorial/docs-concepts-extend-kubernetes-api-extension.md/docs-concepts-extend-kubernetes-api-extension
chunk_id: tutorial/docs-concepts-extend-kubernetes-api-extension.md/docs-concepts-extend-kubernetes-api-extension#2-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 85
summary: * The [CustomResourceDefinition](/docs/concepts/extend-kubernetes/api-extension/custom-resources/) (CRD) mechanism allows you to declaratively define a new custom API with an API group, kind, and...
---

* The [CustomResourceDefinition](/docs/concepts/extend-kubernetes/api-extension/custom-resources/)
(CRD) mechanism allows you to declaratively define a new custom API with an API group, kind, and
schema that you specify.
The Kubernetes control plane serves and handles the storage of your custom resource. CRDs allow you to
create new types of resources for your cluster without writing and running a custom API server.