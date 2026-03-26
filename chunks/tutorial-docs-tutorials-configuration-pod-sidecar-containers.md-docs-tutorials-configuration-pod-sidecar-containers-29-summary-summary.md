---
doc_id: tutorial/docs-tutorials-configuration-pod-sidecar-containers.md/docs-tutorials-configuration-pod-sidecar-containers
chunk_id: tutorial/docs-tutorials-configuration-pod-sidecar-containers.md/docs-tutorials-configuration-pod-sidecar-containers#29-summary
chunk_level: summary
chunk_type: prose
heading: Adopting built-in sidecar containers
token_count: 110
summary: ### Automatic injection of sidecars If you are using software that injects sidecars automatically, there are a few possible strategies you may follow to ensure that native sidecar containers can be...
---

### Automatic injection of sidecars
If you are using software that injects sidecars automatically,
there are a few possible strategies you may follow to
ensure that native sidecar containers can be used.
All strategies are generally options you may choose to decide whether
the Pod the sidecar will be injected to will land on a Node supporting the feature or not.
As an example, you can follow [this conversation in Istio community](https://github.com/istio/istio/issues/48794).
The discussion explores the options listed below.