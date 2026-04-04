---
doc_id: ref/docs-reference-command-line-tools-reference-kube-scheduler.md/docs-reference-command-line-tools-reference-kube-scheduler
chunk_id: ref/docs-reference-command-line-tools-reference-kube-scheduler.md/docs-reference-command-line-tools-reference-kube-scheduler#0-detailed
chunk_level: detailed
chunk_type: prose
heading: Synopsis
token_count: 152
summary: ## Table of Contents    - [Synopsis](#synopsis)   - [Options](#options)   - [Feedback](#feedback)  ---  ## Synopsis The Kubernetes scheduler is a control plane process which assigns Pods to Nodes....
---

## Table of Contents

  - [Synopsis](#synopsis)
  - [Options](#options)
  - [Feedback](#feedback)

---

## Synopsis
The Kubernetes scheduler is a control plane process which assigns
Pods to Nodes. The scheduler determines which Nodes are valid placements for
each Pod in the scheduling queue according to constraints and available
resources. The scheduler then ranks each valid Node and binds the Pod to a
suitable Node. Multiple different schedulers may be used within a cluster;
kube-scheduler is the reference implementation.
See [scheduling](https://kubernetes.io/docs/concepts/scheduling-eviction/)
for more information about scheduling and the kube-scheduler component.
```
`kube-scheduler [flags]
`
```