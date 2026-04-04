---
doc_id: tutorial/docs-tutorials-security-apparmor.md/docs-tutorials-security-apparmor
chunk_id: tutorial/docs-tutorials-security-apparmor.md/docs-tutorials-security-apparmor#6-summary
chunk_level: summary
chunk_type: prose
heading: Objectives
token_count: 119
summary: The kubelet verifies that AppArmor is enabled on the host before admitting a pod with AppArmor explicitly configured. 2. Container runtime supports AppArmor -- All common Kubernetes-supported...
---

The kubelet verifies that AppArmor is enabled on the host before admitting a pod with AppArmor
explicitly configured.
2. Container runtime supports AppArmor -- All common Kubernetes-supported container
runtimes should support AppArmor, including [containerd](https://containerd.io/docs/) and
[CRI-O](https://cri-o.io/#what-is-cri-o). Please refer to the corresponding runtime
documentation and verify that the cluster fulfills the requirements to use AppArmor.
3. Profile is loaded -- AppArmor is applied to a Pod by specifying an AppArmor profile that each