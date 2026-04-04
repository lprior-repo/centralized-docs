---
doc_id: tutorial/docs-tasks-extend-kubernetes-setup-konnectivity.md/docs-tasks-extend-kubernetes-setup-konnectivity
chunk_id: tutorial/docs-tasks-extend-kubernetes-setup-konnectivity.md/docs-tasks-extend-kubernetes-setup-konnectivity#3-detailed
chunk_level: detailed
chunk_type: prose
heading: Configure the Konnectivity service
token_count: 797
summary: Next, you need to deploy the Konnectivity server and agents. [kubernetes-sigs/apiserver-network-proxy](https://github.com/kubernetes-sigs/apiserver-network-proxy) is a reference implementation....
---

Next, you need to deploy the Konnectivity server and agents.
[kubernetes-sigs/apiserver-network-proxy](https://github.com/kubernetes-sigs/apiserver-network-proxy)
is a reference implementation.
Deploy the Konnectivity server on your control plane node. The provided
`konnectivity-server.yaml` manifest assumes
that the Kubernetes components are deployed as a [static Pod](/docs/tasks/configure-pod-container/static-pod/) in your cluster. If not, you can deploy the Konnectivity
server as a DaemonSet.
[`admin/konnectivity/konnectivity-server.yaml`
](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/admin/konnectivity/konnectivity-server.yaml)![](/images/copycode.svg "Copy admin/konnectivity/konnectivity-server.yaml to clipboard")
```
`apiVersion: v1
kind: Pod
metadata:
name: konnectivity-server
namespace: kube-system
spec:
priorityClassName: system-cluster-critical
hostNetwork: true
containers:
- name: konnectivity-server-container
image: registry.k8s.io/kas-network-proxy/proxy-server:v0.0.37
command: ["/proxy-server"]
args: [
"--logtostderr=true",
# This needs to be consistent with the value set in egressSelectorConfiguration.
"--uds-name=/etc/kubernetes/konnectivity-server/konnectivity-server.socket",
"--delete-existing-uds-file",
# The following two lines assume the Konnectivity server is
# deployed on the same machine as the apiserver, and the certs and
# key of the API Server are at the specified location.
"--cluster-cert=/etc/kubernetes/pki/apiserver.crt",
"--cluster-key=/etc/kubernetes/pki/apiserver.key",
# This needs to be consistent with the value set in egressSelectorConfiguration.
"--mode=grpc",
"--server-port=0",
"--agent-port=8132",
"--admin-port=8133",
"--health-port=8134",
"--agent-namespace=kube-system",
"--agent-service-account=konnectivity-agent",
"--kubeconfig=/etc/kubernetes/konnectivity-server.conf",
"--authentication-audience=system:konnectivity-server"
]
livenessProbe:
httpGet:
scheme: HTTP
host: 127.0.0.1
port: 8134
path: /healthz
initialDelaySeconds: 30
timeoutSeconds: 60
ports:
- name: agentport
containerPort: 8132
hostPort: 8132
- name: adminport
containerPort: 8133
hostPort: 8133
- name: healthport
containerPort: 8134
hostPort: 8134
volumeMounts:
- name: k8s-certs
mountPath: /etc/kubernetes/pki
readOnly: true
- name: kubeconfig
mountPath: /etc/kubernetes/konnectivity-server.conf
readOnly: true
- name: konnectivity-uds
mountPath: /etc/kubernetes/konnectivity-server
readOnly: false
volumes:
- name: k8s-certs
hostPath:
path: /etc/kubernetes/pki
- name: kubeconfig
hostPath:
path: /etc/kubernetes/konnectivity-server.conf
type: FileOrCreate
- name: konnectivity-uds
hostPath:
path: /etc/kubernetes/konnectivity-server
type: DirectoryOrCreate
`
```
Then deploy the Konnectivity agents in your cluster:
[`admin/konnectivity/konnectivity-agent.yaml`
](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/admin/konnectivity/konnectivity-agent.yaml)![](/images/copycode.svg "Copy admin/konnectivity/konnectivity-agent.yaml to clipboard")