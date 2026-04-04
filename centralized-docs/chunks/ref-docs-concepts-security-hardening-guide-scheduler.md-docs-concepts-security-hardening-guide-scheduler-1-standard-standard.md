---
doc_id: ref/docs-concepts-security-hardening-guide-scheduler.md/docs-concepts-security-hardening-guide-scheduler
chunk_id: ref/docs-concepts-security-hardening-guide-scheduler.md/docs-concepts-security-hardening-guide-scheduler#1-standard
chunk_level: standard
chunk_type: prose
heading: Table of Contents
token_count: 145
summary: # Hardening Guide - Scheduler Configuration Information about how to make the Kubernetes scheduler more secure. The Kubernetes...
---

# Hardening Guide - Scheduler Configuration
Information about how to make the Kubernetes scheduler more secure.
The Kubernetes [scheduler](/docs/reference/command-line-tools-reference/kube-scheduler/) is
one of the critical components of the
[control plane](/docs/reference/glossary/?all=true#term-control-plane).
This document covers how to improve the security posture of the Scheduler.
A misconfigured scheduler can have security implications.
Such a scheduler can target specific nodes and evict the workloads or applications that are sharing the node and its resources.
This can aid an attacker with a [Yo-Yo attack](https://arxiv.org/abs/2105.00542): an attack on a vulnerable autoscaler.