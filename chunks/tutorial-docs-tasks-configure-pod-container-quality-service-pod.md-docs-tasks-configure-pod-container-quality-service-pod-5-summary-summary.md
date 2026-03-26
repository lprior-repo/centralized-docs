---
doc_id: tutorial/docs-tasks-configure-pod-container-quality-service-pod.md/docs-tasks-configure-pod-container-quality-service-pod
chunk_id: tutorial/docs-tasks-configure-pod-container-quality-service-pod.md/docs-tasks-configure-pod-container-quality-service-pod#5-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 84
summary: # Configure Quality of Service for Pods This page shows how to configure Pods so that they will be assigned particular [Quality of Service (QoS) classes](/docs/concepts/workloads/pods/pod-qos/)....
---

# Configure Quality of Service for Pods
This page shows how to configure Pods so that they will be assigned particular
[Quality of Service (QoS) classes](/docs/concepts/workloads/pods/pod-qos/).
Kubernetes uses QoS classes to make decisions about evicting Pods when Node resources are exceeded.
When Kubernetes creates a Pod it assigns one of these QoS classes to the Pod: