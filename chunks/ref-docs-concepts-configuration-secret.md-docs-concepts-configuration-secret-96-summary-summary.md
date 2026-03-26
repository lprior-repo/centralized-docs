---
doc_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret
chunk_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret#96-summary
chunk_level: summary
chunk_type: prose
heading: Types of Secret
token_count: 87
summary: reference actually points to an object of type Secret. Therefore, a Secret needs to be created before any Pods that depend on it. If the Secret cannot be fetched (perhaps because it does not exist,...
---

reference actually points to an object of type Secret. Therefore, a Secret
needs to be created before any Pods that depend on it.
If the Secret cannot be fetched (perhaps because it does not exist, or
due to a temporary lack of connection to the API server) the kubelet
periodically retries running that Pod. The kubelet also reports an Event
for that Pod, including details of the problem fetching the Secret.