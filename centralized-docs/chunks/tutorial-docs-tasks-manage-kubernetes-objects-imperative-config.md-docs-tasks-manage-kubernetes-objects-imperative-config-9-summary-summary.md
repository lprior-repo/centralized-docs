---
doc_id: tutorial/docs-tasks-manage-kubernetes-objects-imperative-config.md/docs-tasks-manage-kubernetes-objects-imperative-config
chunk_id: tutorial/docs-tasks-manage-kubernetes-objects-imperative-config.md/docs-tasks-manage-kubernetes-objects-imperative-config#9-summary
chunk_level: summary
chunk_type: prose
heading: How to create objects
token_count: 112
summary: #### Warning: Updating objects with the `replace` command drops all parts of the spec not specified in the configuration file. This should not be used with objects whose specs are partially managed...
---

#### Warning:
Updating objects with the `replace` command drops all
parts of the spec not specified in the configuration file. This
should not be used with objects whose specs are partially managed
by the cluster, such as Services of type `LoadBalancer`, where
the `externalIPs` field is managed independently from the configuration
file. Independently managed fields must be copied to the configuration
file to prevent `replace` from dropping them.
You can use `kubectl replace -f` to update a live object according to a
configuration file.