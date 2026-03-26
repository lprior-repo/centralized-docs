---
doc_id: concept/docs-tasks-administer-cluster-kubelet-credential-provider.md/docs-tasks-administer-cluster-kubelet-credential-provider
chunk_id: concept/docs-tasks-administer-cluster-kubelet-credential-provider.md/docs-tasks-administer-cluster-kubelet-credential-provider#28-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 96
summary: short-lived credentials for an image that is being pulled by the kubelet. You may be interested in using this capability if any of the below are true: * API calls to a cloud provider service are...
---

short-lived credentials for an image that is being pulled by the kubelet.
You may be interested in using this capability if any of the below are true:
* API calls to a cloud provider service are required to retrieve authentication information for a registry.
* Credentials have short expiration times and requesting new credentials frequently is required.
* Storing registry credentials on disk or in imagePullSecrets is not acceptable.
This guide demonstrates how to configure the kubelet's image credential provider plugin mechanism.