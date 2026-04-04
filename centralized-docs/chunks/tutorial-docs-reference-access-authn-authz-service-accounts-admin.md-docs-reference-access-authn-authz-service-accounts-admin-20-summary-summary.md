---
doc_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin
chunk_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin#20-summary
chunk_level: summary
chunk_type: prose
heading: User accounts versus service accounts
token_count: 100
summary: 3. Copy this token into a new file named `tokenreview.yaml`: ``` `apiVersion: authentication.k8s.io/v1 kind: TokenReview spec: token: &lt;token from step 2&gt; ` ``` 4. Submit this resource to the...
---

3. Copy this token into a new file named `tokenreview.yaml`:
```
`apiVersion: authentication.k8s.io/v1
kind: TokenReview
spec:
token: &lt;token from step 2&gt;
`
```
4. Submit this resource to the apiserver for review:
```
`# use '-o yaml' to inspect the output
kubectl create -o yaml -f tokenreview.yaml
`
```
You should see an output like below: