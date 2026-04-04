---
doc_id: tutorial/docs-tasks-extend-kubernetes-setup-konnectivity.md/docs-tasks-extend-kubernetes-setup-konnectivity
chunk_id: tutorial/docs-tasks-extend-kubernetes-setup-konnectivity.md/docs-tasks-extend-kubernetes-setup-konnectivity#4-detailed
chunk_level: detailed
chunk_type: prose
heading: Related Pages
token_count: 989
summary: Then deploy the Konnectivity agents in your cluster: [`admin/konnectivity/konnectivity-agent.yaml`...
---

Then deploy the Konnectivity agents in your cluster:
[`admin/konnectivity/konnectivity-agent.yaml`
](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/admin/konnectivity/konnectivity-agent.yaml)![](/images/copycode.svg "Copy admin/konnectivity/konnectivity-agent.yaml to clipboard")
```
`apiVersion: apps/v1
# Alternatively, you can deploy the agents as Deployments. It is not necessary
# to have an agent on each node.
kind: DaemonSet
metadata:
labels:
addonmanager.kubernetes.io/mode: Reconcile
k8s-app: konnectivity-agent
namespace: kube-system
name: konnectivity-agent
spec:
selector:
matchLabels:
k8s-app: konnectivity-agent
template:
metadata:
labels:
k8s-app: konnectivity-agent
spec:
priorityClassName: system-cluster-critical
tolerations:
- key: "CriticalAddonsOnly"
operator: "Exists"
containers:
- image: us.gcr.io/k8s-artifacts-prod/kas-network-proxy/proxy-agent:v0.0.37
name: konnectivity-agent
command: ["/proxy-agent"]
args: [
"--logtostderr=true",
"--ca-cert=/var/run/secrets/kubernetes.io/serviceaccount/ca.crt",
# this is the IP address of the master machine.
"--proxy-server-host=35.225.206.7",
"--proxy-server-port=8132",
"--admin-server-port=8133",
"--health-server-port=8134",
"--service-account-token-path=/var/run/secrets/tokens/konnectivity-agent-token"
]
volumeMounts:
- mountPath: /var/run/secrets/tokens
name: konnectivity-agent-token
livenessProbe:
httpGet:
port: 8134
path: /healthz
initialDelaySeconds: 15
timeoutSeconds: 15
serviceAccountName: konnectivity-agent
volumes:
- name: konnectivity-agent-token
projected:
sources:
- serviceAccountToken:
path: konnectivity-agent-token
audience: system:konnectivity-server
`
```
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
## Related Pages

- [Volumes](docs-concepts-storage-volumes.md)
- [Binding](docs-reference-kubernetes-api-workload-resources-binding-v1.md)
- [conventions](docs-reference-kubectl-conventions.md)
- [HorizontalPodAutoscaler](docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2.md)
- [Концепции](ru-docs-concepts.md)