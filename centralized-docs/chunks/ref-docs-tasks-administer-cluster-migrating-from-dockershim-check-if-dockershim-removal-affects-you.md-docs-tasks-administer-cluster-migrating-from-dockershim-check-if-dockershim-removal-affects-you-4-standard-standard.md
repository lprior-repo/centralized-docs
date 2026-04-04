---
doc_id: ref/docs-tasks-administer-cluster-migrating-from-dockershim-check-if-dockershim-removal-affects-you.md/docs-tasks-administer-cluster-migrating-from-dockershim-check-if-dockershim-removal-affects-you
chunk_id: ref/docs-tasks-administer-cluster-migrating-from-dockershim-check-if-dockershim-removal-affects-you.md/docs-tasks-administer-cluster-migrating-from-dockershim-check-if-dockershim-removal-affects-you#4-standard
chunk_level: standard
chunk_type: prose
heading: Finding if your app has a dependencies on Docker
token_count: 468
summary: * Make sure there are no indirect dependencies on dockershim behavior. This is an edge case and unlikely to affect your application. Some tooling may be configured to react to Docker-specific...
---

* Make sure there are no indirect dependencies on dockershim behavior.
This is an edge case and unlikely to affect your application. Some tooling may be configured
to react to Docker-specific behaviors, for example, raise alert on specific metrics or search for
a specific log message as part of troubleshooting instructions.
If you have such tooling configured, test the behavior on a test
cluster before migration.## Dependency on Docker explained
A [container runtime](/docs/concepts/containers/#container-runtimes) is software that can
execute the containers that make up a Kubernetes pod. Kubernetes is responsible for orchestration
and scheduling of Pods; on each node, the [kubelet](/docs/reference/command-line-tools-reference/kubelet)
uses the container runtime interface as an abstraction so that you can use any compatible
container runtime.
In its earliest releases, Kubernetes offered compatibility with one container runtime: Docker.
Later in the Kubernetes project's history, cluster operators wanted to adopt additional container runtimes.
The CRI was designed to allow this kind of flexibility - and the kubelet began supporting CRI. However,
because Docker existed before the CRI specification was invented, the Kubernetes project created an
adapter component, `dockershim`. The dockershim adapter allows the kubelet to interact with Docker as
if Docker were a CRI compatible runtime.
You can read about it in [Kubernetes Containerd integration goes GA](/blog/2018/05/24/kubernetes-containerd-integration-goes-ga/) blog post.
![Dockershim vs. CRI with Containerd](/images/blog/2018-05-24-kubernetes-containerd-integration-goes-ga/cri-containerd.png)
Switching to Containerd as a container runtime eliminates the middleman. All the
same containers can be run by container runtimes like Containerd as before. But
now, since containers schedule directly with the container runtime, they are not visible to Docker.
So any Docker tooling or fancy UI you might have used
before to check on these containers is no longer available.
You cannot get container information using `docker ps` or `docker inspect`
commands. As you cannot list containers, you cannot get logs, stop containers,
or execute something inside a container using `docker exec`.