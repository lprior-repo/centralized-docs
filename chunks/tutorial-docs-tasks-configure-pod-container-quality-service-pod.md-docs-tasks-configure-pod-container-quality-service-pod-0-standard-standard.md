---
doc_id: tutorial/docs-tasks-configure-pod-container-quality-service-pod.md/docs-tasks-configure-pod-container-quality-service-pod
chunk_id: tutorial/docs-tasks-configure-pod-container-quality-service-pod.md/docs-tasks-configure-pod-container-quality-service-pod#0-standard
chunk_level: standard
chunk_type: prose
heading: Table of Contents
token_count: 296
summary: ## Table of Contents  - [Configure Quality of Service for Pods](#configure-quality-of-service-for-pods)       - [Note:](#note)   - [Before you begin](#before-you-begin)   - [Create a...
---

## Table of Contents

- [Configure Quality of Service for Pods](#configure-quality-of-service-for-pods)
      - [Note:](#note)
  - [Before you begin](#before-you-begin)
  - [Create a namespace](#create-a-namespace)
  - [Create a Pod that gets assigned a QoS class of Guaranteed](#create-a-pod-that-gets-assigned-a-qos-class-of-guaranteed)
      - [Note:](#note)
      - [Clean up](#clean-up)
  - [Create a Pod that gets assigned a QoS class of Burstable](#create-a-pod-that-gets-assigned-a-qos-class-of-burstable)
      - [Clean up](#clean-up)
  - [Create a Pod that gets assigned a QoS class of BestEffort](#create-a-pod-that-gets-assigned-a-qos-class-of-besteffort)
      - [Clean up](#clean-up)
  - [Create a Pod that has two Containers](#create-a-pod-that-has-two-containers)
  - [Retrieve the QoS class for a Pod](#retrieve-the-qos-class-for-a-pod)
  - [Clean up](#clean-up)
    - [For app developers](#for-app-developers)
    - [For cluster administrators](#for-cluster-administrators)
  - [Feedback](#feedback)

---