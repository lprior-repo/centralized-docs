---
doc_id: tutorial/docs-concepts-containers.md/docs-concepts-containers
chunk_id: tutorial/docs-concepts-containers.md/docs-concepts-containers#8-summary
chunk_level: summary
chunk_type: prose
heading: Container runtimes
token_count: 118
summary: A fundamental component that empowers Kubernetes to run containers effectively. It is responsible for managing the execution and lifecycle of containers within the Kubernetes environment. Kubernetes...
---

A fundamental component that empowers Kubernetes to run containers effectively.
It is responsible for managing the execution and lifecycle of containers within the Kubernetes environment.
Kubernetes supports container runtimes such as
[containerd](https://containerd.io/docs/), [CRI-O](https://cri-o.io/#what-is-cri-o),
and any other implementation of the [Kubernetes CRI (Container Runtime
Interface)](https://github.com/kubernetes/community/blob/master/contributors/devel/sig-node/container-runtime-interface.md).
Usually, you can allow your cluster to pick the default container runtime