---
doc_id: tutorial/docs-tasks-extend-kubernetes-setup-konnectivity.md/docs-tasks-extend-kubernetes-setup-konnectivity
chunk_id: tutorial/docs-tasks-extend-kubernetes-setup-konnectivity.md/docs-tasks-extend-kubernetes-setup-konnectivity#11-standard
chunk_level: standard
chunk_type: prose
heading: Feedback
token_count: 443
summary: Last, if RBAC is enabled in your cluster, create the relevant RBAC rules: [`admin/konnectivity/konnectivity-rbac.yaml`...
---

Last, if RBAC is enabled in your cluster, create the relevant RBAC rules:
[`admin/konnectivity/konnectivity-rbac.yaml`
](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/admin/konnectivity/konnectivity-rbac.yaml)![](/images/copycode.svg "Copy admin/konnectivity/konnectivity-rbac.yaml to clipboard")
```
`apiVersion: rbac.authorization.k8s.io/v1
kind: ClusterRoleBinding
metadata:
name: system:konnectivity-server
labels:
kubernetes.io/cluster-service: "true"
addonmanager.kubernetes.io/mode: Reconcile
roleRef:
apiGroup: rbac.authorization.k8s.io
kind: ClusterRole
name: system:auth-delegator
subjects:
- apiGroup: rbac.authorization.k8s.io
kind: User
name: system:konnectivity-server
---
apiVersion: v1
kind: ServiceAccount
metadata:
name: konnectivity-agent
namespace: kube-system
labels:
kubernetes.io/cluster-service: "true"
addonmanager.kubernetes.io/mode: Reconcile
`
```
## Feedback
Was this page helpful?
Yes
No
Thanks for the feedback. If you have a specific, answerable question about how to use Kubernetes, ask it on
[Stack Overflow](https://stackoverflow.com/questions/tagged/kubernetes).
Open an issue in the [GitHub Repository](https://www.github.com/kubernetes/website/) if you want to
[report a problem](<https://github.com/kubernetes/website/issues/new?title=Issue with k8s.io>)
or
[suggest an improvement](<https://github.com/kubernetes/website/issues/new?title=Improvement for k8s.io>).
Last modified February 03, 2026 at 9:36 PM PST: [Modify OpenSSL command for konnectivity server setup (cf26b27b7a)](https://github.com/kubernetes/website/commit/cf26b27b7a64501000e4163afbc2ba998c619b07)