---
doc_id: tutorial/docs-tasks-extend-kubernetes-setup-konnectivity.md/docs-tasks-extend-kubernetes-setup-konnectivity
chunk_id: tutorial/docs-tasks-extend-kubernetes-setup-konnectivity.md/docs-tasks-extend-kubernetes-setup-konnectivity#32-summary
chunk_level: summary
chunk_type: prose
heading: Configure the Konnectivity service
token_count: 125
summary: template: metadata: labels: k8s-app: konnectivity-agent spec: priorityClassName: system-cluster-critical tolerations: - key: \"CriticalAddonsOnly\" operator: \"Exists\" containers: - image:...
---

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