---
doc_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin
chunk_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin#4-detailed
chunk_level: detailed
chunk_type: prose
heading: Bound service account token volume mechanism
token_count: 829
summary: #### Note: The `aud` and `iss` fields in this JWT may differ between different Kubernetes clusters depending on your configuration. The presence of both the `pod` and `node` claim implies that this...
---

#### Note:
The `aud` and `iss` fields in this JWT may differ between different Kubernetes clusters depending
on your configuration.
The presence of both the `pod` and `node` claim implies that this token is bound
to a *Pod* object. When verifying Pod bound ServiceAccount tokens, the API server **does not**
verify the existence of the referenced Node object.
Services that run outside of Kubernetes and want to perform offline validation of JWTs may
use this schema, along with a compliant JWT validator configured with OpenID Discovery information
from the API server, to verify presented JWTs without requiring use of the TokenReview API.
Services that verify JWTs in this way **do not verify** the claims embedded in the JWT token to be
current and still valid.
This means if the token is bound to an object, and that object no longer exists, the token will still
be considered valid (until the configured token expires).
Clients that require assurance that a token's bound claims are still valid **MUST** use the TokenReview
API to present the token to the `kube-apiserver` for it to verify and expand the embedded claims, using
similar steps to the [Verifying and inspecting private claims](#verifying-and-inspecting-private-claims)
section above, but with a [supported client library](/docs/reference/using-api/client-libraries/).
For more information on JWTs and their structure, see the [JSON Web Token RFC](https://datatracker.ietf.org/doc/html/rfc7519).
## Bound service account token volume mechanism
FEATURE STATE:
`Kubernetes v1.22 [stable]`(enabled by default)
By default, the Kubernetes control plane (specifically, the
[ServiceAccount admission controller](#serviceaccount-admission-controller))
adds a [projected volume](/docs/concepts/storage/projected-volumes/) to Pods,
and this volume includes a token for Kubernetes API access.
Here's an example of how that looks for a launched Pod:
```
`...
- name: kube-api-access-&lt;random-suffix&gt;
projected:
sources:
- serviceAccountToken:
path: token # must match the path the app expects
- configMap:
items:
- key: ca.crt
path: ca.crt
name: kube-root-ca.crt
- downwardAPI:
items:
- fieldRef:
apiVersion: v1
fieldPath: metadata.namespace
path: namespace
`
```
That manifest snippet defines a projected volume that consists of three sources. In this case,
each source also represents a single path within that volume. The three sources are:
1. A `serviceAccountToken` source, that contains a token that the kubelet acquires from kube-apiserver.
The kubelet fetches time-bound tokens using the TokenRequest API. A token served for a TokenRequest expires
either when the pod is deleted or after a defined lifespan (by default, that is 1 hour).
The kubelet also refreshes that token before the token expires.
The token is bound to the specific Pod and has the kube-apiserver as its audience.
This mechanism superseded an earlier mechanism that added a volume based on a Secret,
where the Secret represented the ServiceAccount for the Pod, but did not expire.
2. A `configMap` source. The ConfigMap contains a bundle of certificate authority data. Pods can use these
certificates to make sure that they are connecting to your cluster's kube-apiserver (and not to middlebox
or an accidentally misconfigured peer).
3. A `downwardAPI` source that looks up the name of the namespace containing the Pod, and makes
that name information available to application code running inside the Pod.
Any container within the Pod that mounts this particular volume can access the above information.
#### Note:
There is no specific mechanism to invalidate a token issued via TokenRequest. If you no longer
trust a bound service account token for a Pod, you can delete that Pod. Deleting a Pod expires
its bound service account tokens.