---
doc_id: tutorial/docs-tutorials-configuration-pod-sidecar-containers.md/docs-tutorials-configuration-pod-sidecar-containers
chunk_id: tutorial/docs-tutorials-configuration-pod-sidecar-containers.md/docs-tutorials-configuration-pod-sidecar-containers#32-summary
chunk_level: summary
chunk_type: prose
heading: Adopting built-in sidecar containers
token_count: 125
summary: * there may be other custom ways to detect nodes compatibility. * Develop a universal sidecar injector. The idea of a universal sidecar injector is to inject a sidecar container as a regular...
---

* there may be other custom ways to detect nodes compatibility.
* Develop a universal sidecar injector. The idea of a universal sidecar injector is to
inject a sidecar container as a regular container as well as a native sidecar container.
And have a runtime logic to decide which one will work. The universal sidecar injector
is wasteful, as it will account for requests twice, but may be considered as a workable
solution for special cases.
* One way would be on start of a native sidecar container
detect the node version and exit immediately if the version does not support the sidecar feature.