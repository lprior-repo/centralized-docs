---
doc_id: ref/docs-concepts-scheduling-eviction-pod-priority-preemption.md/docs-concepts-scheduling-eviction-pod-priority-preemption
chunk_id: ref/docs-concepts-scheduling-eviction-pod-priority-preemption.md/docs-concepts-scheduling-eviction-pod-priority-preemption#0-detailed
chunk_level: detailed
chunk_type: prose
heading: Table of Contents
token_count: 453
summary: ## Table of Contents  - [Pod Priority and Preemption](#pod-priority-and-preemption)       - [Warning:](#warning)   - [How to use priority and preemption](#how-to-use-priority-and-preemption)       -...
---

## Table of Contents

- [Pod Priority and Preemption](#pod-priority-and-preemption)
      - [Warning:](#warning)
  - [How to use priority and preemption](#how-to-use-priority-and-preemption)
      - [Note:](#note)
  - [PriorityClass](#priorityclass)
    - [Notes about PodPriority and existing clusters](#notes-about-podpriority-and-existing-clusters)
    - [Example PriorityClass](#example-priorityclass)
  - [Non-preempting PriorityClass](#non-preempting-priorityclass)
    - [Example Non-preempting PriorityClass](#example-non-preempting-priorityclass)
  - [Pod priority](#pod-priority)
    - [Effect of Pod priority on scheduling order](#effect-of-pod-priority-on-scheduling-order)
  - [Preemption](#preemption)
    - [User exposed information](#user-exposed-information)
      - [Graceful termination of preemption victims](#graceful-termination-of-preemption-victims)
      - [PodDisruptionBudget is supported, but not guaranteed](#poddisruptionbudget-is-supported-but-not-guaranteed)
      - [Inter-Pod affinity on lower-priority Pods](#inter-pod-affinity-on-lower-priority-pods)
      - [Note:](#note)
      - [Cross node preemption](#cross-node-preemption)
  - [Troubleshooting](#troubleshooting)
    - [Pods are preempted unnecessarily](#pods-are-preempted-unnecessarily)
    - [Pods are preempted, but the preemptor is not scheduled](#pods-are-preempted-but-the-preemptor-is-not-scheduled)
    - [Higher priority Pods are preempted before lower priority pods](#higher-priority-pods-are-preempted-before-lower-priority-pods)
  - [Interactions between Pod priority and quality of service](#interactions-between-pod-priority-and-quality-of-service)
  - [What's next](#whats-next)
  - [Feedback](#feedback)

---