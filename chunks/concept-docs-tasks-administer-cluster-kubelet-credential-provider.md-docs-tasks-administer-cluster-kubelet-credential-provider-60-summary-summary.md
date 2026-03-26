---
doc_id: concept/docs-tasks-administer-cluster-kubelet-credential-provider.md/docs-tasks-administer-cluster-kubelet-credential-provider
chunk_id: concept/docs-tasks-administer-cluster-kubelet-credential-provider.md/docs-tasks-administer-cluster-kubelet-credential-provider#60-summary
chunk_level: summary
chunk_type: prose
heading: Configuring the Kubelet
token_count: 81
summary: , and top-level domains such as `k8s.\*`. Matching partial subdomains like `app\*.k8s.io` is also supported. Each glob can only match a single subdomain segment, so `\*.io` does NOT match...
---

,
and top-level domains such as `k8s.\*`. Matching partial subdomains like `app\*.k8s.io` is also supported. Each glob can only match
a single subdomain segment, so `\*.io` does NOT match `\*.k8s.io`.
A match exists between an image name and a `matchImage` entry when all of the below are true: