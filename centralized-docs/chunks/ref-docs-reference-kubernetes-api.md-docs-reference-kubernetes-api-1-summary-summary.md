---
doc_id: ref/docs-reference-kubernetes-api.md/docs-reference-kubernetes-api
chunk_id: ref/docs-reference-kubernetes-api.md/docs-reference-kubernetes-api#1-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 99
summary: # Kubernetes API Kubernetes' API is the application that serves Kubernetes functionality through a RESTful interface and stores the state of the cluster. Kubernetes resources and \"records of intent\"...
---

# Kubernetes API
Kubernetes' API is the application that serves Kubernetes functionality through a RESTful interface and stores the state of the cluster.
Kubernetes resources and "records of intent" are all stored as API objects, and modified via RESTful calls to the API. The API allows configuration to be managed in a declarative way. Users can interact with the Kubernetes API directly, or via tools like `kubectl`. The core Kubernetes API is flexible and can also be extended to support custom resources.