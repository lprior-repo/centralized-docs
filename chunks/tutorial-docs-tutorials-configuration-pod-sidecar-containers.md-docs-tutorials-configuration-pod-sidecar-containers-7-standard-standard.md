---
doc_id: tutorial/docs-tutorials-configuration-pod-sidecar-containers.md/docs-tutorials-configuration-pod-sidecar-containers
chunk_id: tutorial/docs-tutorials-configuration-pod-sidecar-containers.md/docs-tutorials-configuration-pod-sidecar-containers#7-standard
chunk_level: standard
chunk_type: prose
heading: Adopting built-in sidecar containers
token_count: 487
summary: 2. Check Nodes compatibility on injection. During sidecar injection, you may use the following strategies to check node compatibility: * query node version and assume the feature gate is enabled on...
---

2. Check Nodes compatibility on injection. During sidecar injection, you may use
the following strategies to check node compatibility:
* query node version and assume the feature gate is enabled on the version 1.29+
* query node prometheus metrics and check feature enablement status
* assume the nodes are running with a [supported version skew](/releases/version-skew-policy/#supported-version-skew)
from the API server
* there may be other custom ways to detect nodes compatibility.
* Develop a universal sidecar injector. The idea of a universal sidecar injector is to
inject a sidecar container as a regular container as well as a native sidecar container.
And have a runtime logic to decide which one will work. The universal sidecar injector
is wasteful, as it will account for requests twice, but may be considered as a workable
solution for special cases.
* One way would be on start of a native sidecar container
detect the node version and exit immediately if the version does not support the sidecar feature.
* Consider a runtime feature detection design:
* Define an empty dir so containers can communicate with each other
* Inject an init container, let's call it `NativeSidecar` with `restartPolicy=Always`.
* `NativeSidecar` must write a file to an empty directory indicating the first run and exit
immediately with exit code `0`.
* `NativeSidecar` on restart (when native sidecars are supported) checks that file already
exists in the empty dir and changes it - indicating that the built-in sidecar containers
are supported and running.
* Inject regular container, let's call it `OldWaySidecar`.
* `OldWaySidecar` on start checks the presence of a file in an empty dir.
* If the file indicates that the `NativeSidecar` is NOT running, it assumes that the sidecar
feature is not supported and works assuming it is the sidecar.
* If the file indicates that the `NativeSidecar` is running, it either does nothing and sleeps
forever (in the case when Pod’s `restartPolicy=Always`) or exits immediately with exit code `0`
(in the case when Pod’s `restartPolicy!=Always`).## What's next
* Learn more about [sidecar containers](/docs/concepts/workloads/pods/sidecar-containers/).