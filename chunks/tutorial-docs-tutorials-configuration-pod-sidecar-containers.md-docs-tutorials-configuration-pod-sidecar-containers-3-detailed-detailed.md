---
doc_id: tutorial/docs-tutorials-configuration-pod-sidecar-containers.md/docs-tutorials-configuration-pod-sidecar-containers
chunk_id: tutorial/docs-tutorials-configuration-pod-sidecar-containers.md/docs-tutorials-configuration-pod-sidecar-containers#3-detailed
chunk_level: detailed
chunk_type: prose
heading: Related Pages
token_count: 931
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
1. Mark Pods that land to nodes supporting sidecars. You can use node labels
and node affinity to mark nodes supporting sidecar containers and Pods landing on those nodes.
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
## Feedback
Was this page helpful?
Yes
No
Thanks for the feedback. If you have a specific, answerable question about how to use Kubernetes, ask it on
[Stack Overflow](https://stackoverflow.com/questions/tagged/kubernetes).
Open an issue in the [GitHub Repository](https://www.github.com/kubernetes/website/) if you want to
[report a problem](<https://github.com/kubernetes/website/issues/new?title=Issue with k8s.io>)
or
[suggest an improvement](<https://github.com/kubernetes/website/issues/new?title=Improvement for k8s.io>).
Last modified November 06, 2024 at 10:05 AM PST: [Clean up a tutorial: pod-sidecar-containers.md (96d69d62fe)](https://github.com/kubernetes/website/commit/96d69d62fef11a7c7298fd5efaa11403b233bb84)
## Related Pages

- [Hello Minikube](docs-tutorials-hello-minikube.md)
- [Using RBAC Authorization](docs-reference-access-authn-authz-rbac.md)
- [Example: Deploying Cassandra with a StatefulSet](docs-tutorials-stateful-application-cassandra.md)
- [Process ID Limits And Reservations](docs-concepts-policy-pid-limiting.md)
- [Debugging DNS Resolution](docs-tasks-administer-cluster-dns-debugging-resolution.md)