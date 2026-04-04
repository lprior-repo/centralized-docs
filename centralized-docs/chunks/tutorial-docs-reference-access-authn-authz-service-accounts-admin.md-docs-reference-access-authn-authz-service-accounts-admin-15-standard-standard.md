---
doc_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin
chunk_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin#15-standard
chunk_level: standard
chunk_type: prose
heading: Auto-generated legacy ServiceAccount token clean up
token_count: 211
summary: That manifest snippet defines a projected volume that combines information from three sources: 1. A `serviceAccountToken` source, that contains a token that the kubelet acquires from kube-apiserver....
---

That manifest snippet defines a projected volume that combines information from three sources:
1. A `serviceAccountToken` source, that contains a token that the kubelet acquires from kube-apiserver.
The kubelet fetches time-bound tokens using the TokenRequest API. A token served for a TokenRequest expires
either when the pod is deleted or after a defined lifespan (by default, that is 1 hour).
The token is bound to the specific Pod and has the kube-apiserver as its audience.
2. A `configMap` source. The ConfigMap contains a bundle of certificate authority data. Pods can use these
certificates to make sure that they are connecting to your cluster's kube-apiserver (and not to a middlebox
or an accidentally misconfigured peer).
3. A `downwardAPI` source. This `downwardAPI` volume makes the name of the namespace containing the Pod available
to application code running inside the Pod.
Any container within the Pod that mounts this volume can access the above information.