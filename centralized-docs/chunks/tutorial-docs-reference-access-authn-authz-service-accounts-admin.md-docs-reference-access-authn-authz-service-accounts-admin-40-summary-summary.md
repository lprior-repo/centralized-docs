---
doc_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin
chunk_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin#40-summary
chunk_level: summary
chunk_type: prose
heading: Bound service account token volume mechanism
token_count: 101
summary: 3. A `downwardAPI` source that looks up the name of the namespace containing the Pod, and makes that name information available to application code running inside the Pod. Any container within the...
---

3. A `downwardAPI` source that looks up the name of the namespace containing the Pod, and makes
that name information available to application code running inside the Pod.
Any container within the Pod that mounts this particular volume can access the above information.
#### Note:
There is no specific mechanism to invalidate a token issued via TokenRequest. If you no longer
trust a bound service account token for a Pod, you can delete that Pod. Deleting a Pod expires
its bound service account tokens.