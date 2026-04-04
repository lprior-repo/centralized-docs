---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#8-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 103
summary: # Implementation details FEATURE STATE: `Kubernetes v1.10 [stable]` `kubeadm init` and `kubeadm join` together provide a nice user experience for creating a bare Kubernetes cluster from scratch, that...
---

# Implementation details
FEATURE STATE:
`Kubernetes v1.10 [stable]`
`kubeadm init` and `kubeadm join` together provide a nice user experience for creating a
bare Kubernetes cluster from scratch, that aligns with the best-practices.
However, it might not be obvious *how* kubeadm does that.
This document provides additional details on what happens under the hood, with the aim of sharing
knowledge on the best practices for a Kubernetes cluster.