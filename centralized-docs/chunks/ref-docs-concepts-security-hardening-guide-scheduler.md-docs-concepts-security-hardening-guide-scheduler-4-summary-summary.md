---
doc_id: ref/docs-concepts-security-hardening-guide-scheduler.md/docs-concepts-security-hardening-guide-scheduler
chunk_id: ref/docs-concepts-security-hardening-guide-scheduler.md/docs-concepts-security-hardening-guide-scheduler#4-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 109
summary: Information about how to make the Kubernetes scheduler more secure. The Kubernetes [scheduler](/docs/reference/command-line-tools-reference/kube-scheduler/) is one of the critical components of the...
---

Information about how to make the Kubernetes scheduler more secure.
The Kubernetes [scheduler](/docs/reference/command-line-tools-reference/kube-scheduler/) is
one of the critical components of the
[control plane](/docs/reference/glossary/?all=true#term-control-plane).
This document covers how to improve the security posture of the Scheduler.
A misconfigured scheduler can have security implications.
Such a scheduler can target specific nodes and evict the workloads or applications that are sharing the node and its resources.
This can aid an attacker with a