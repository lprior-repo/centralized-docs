---
doc_id: ref/docs-tasks-administer-cluster-guaranteed-scheduling-critical-addon-pods.md/docs-tasks-administer-cluster-guaranteed-scheduling-critical-addon-pods
chunk_id: ref/docs-tasks-administer-cluster-guaranteed-scheduling-critical-addon-pods.md/docs-tasks-administer-cluster-guaranteed-scheduling-critical-addon-pods#2-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 109
summary: Kubernetes core components such as the API server, scheduler, and controller-manager run on a control plane node. However, add-ons must run on a regular cluster node. Some of these add-ons are...
---

Kubernetes core components such as the API server, scheduler, and controller-manager run on a control plane node. However, add-ons must run on a regular cluster node.
Some of these add-ons are critical to a fully functional cluster, such as metrics-server, DNS, and UI.
A cluster may stop working properly if a critical add-on is evicted (either manually or as a side effect of another operation like upgrade)
and becomes pending (for example when the cluster is highly utilized and either there are other pending pods that schedule into the space