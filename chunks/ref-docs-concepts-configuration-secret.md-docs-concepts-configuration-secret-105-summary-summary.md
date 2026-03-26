---
doc_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret
chunk_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret#105-summary
chunk_level: summary
chunk_type: prose
heading: Types of Secret
token_count: 116
summary: controls which strategy the kubelet uses. The default strategy is `Watch`. Updates to Secrets can be either propagated by an API watch mechanism (the default), based on a cache with a defined...
---

 controls
which strategy the kubelet uses. The default strategy is `Watch`.
Updates to Secrets can be either propagated by an API watch mechanism (the default), based on
a cache with a defined time-to-live, or polled from the cluster API server on each kubelet
synchronisation loop.
As a result, the total delay from the moment when the Secret is updated to the moment
when new keys are projected to the Pod can be as long as the kubelet sync period + cache
propagation delay, where the cache propagation delay depends on the chosen cache type