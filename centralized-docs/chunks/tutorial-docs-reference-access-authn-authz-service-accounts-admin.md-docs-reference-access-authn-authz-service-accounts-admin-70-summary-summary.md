---
doc_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin
chunk_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin#70-summary
chunk_level: summary
chunk_type: prose
heading: Auto-generated legacy ServiceAccount token clean up
token_count: 108
summary: 2. A `configMap` source. The ConfigMap contains a bundle of certificate authority data. Pods can use these certificates to make sure that they are connecting to your cluster's kube-apiserver (and not...
---

2. A `configMap` source. The ConfigMap contains a bundle of certificate authority data. Pods can use these
certificates to make sure that they are connecting to your cluster's kube-apiserver (and not to a middlebox
or an accidentally misconfigured peer).
3. A `downwardAPI` source. This `downwardAPI` volume makes the name of the namespace containing the Pod available
to application code running inside the Pod.
Any container within the Pod that mounts this volume can access the above information.