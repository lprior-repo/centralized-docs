---
doc_id: tutorial/docs-tutorials-configuration-pod-sidecar-containers.md/docs-tutorials-configuration-pod-sidecar-containers
chunk_id: tutorial/docs-tutorials-configuration-pod-sidecar-containers.md/docs-tutorials-configuration-pod-sidecar-containers#33-summary
chunk_level: summary
chunk_type: prose
heading: Adopting built-in sidecar containers
token_count: 116
summary: * Consider a runtime feature detection design: * Define an empty dir so containers can communicate with each other * Inject an init container, let's call it `NativeSidecar` with...
---

* Consider a runtime feature detection design:
* Define an empty dir so containers can communicate with each other
* Inject an init container, let's call it `NativeSidecar` with `restartPolicy=Always`.
* `NativeSidecar` must write a file to an empty directory indicating the first run and exit
immediately with exit code `0`.
* `NativeSidecar` on restart (when native sidecars are supported) checks that file already
exists in the empty dir and changes it - indicating that the built-in sidecar containers
are supported and running.