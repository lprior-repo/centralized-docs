---
doc_id: tutorial/docs-tasks-extend-kubernetes-setup-konnectivity.md/docs-tasks-extend-kubernetes-setup-konnectivity
chunk_id: tutorial/docs-tasks-extend-kubernetes-setup-konnectivity.md/docs-tasks-extend-kubernetes-setup-konnectivity#10-standard
chunk_level: standard
chunk_type: prose
heading: Configure the Konnectivity service
token_count: 457
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