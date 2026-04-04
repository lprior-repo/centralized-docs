---
doc_id: tutorial/docs-tasks-extend-kubernetes-setup-konnectivity.md/docs-tasks-extend-kubernetes-setup-konnectivity
chunk_id: tutorial/docs-tasks-extend-kubernetes-setup-konnectivity.md/docs-tasks-extend-kubernetes-setup-konnectivity#2-detailed
chunk_level: detailed
chunk_type: prose
heading: Configure the Konnectivity service
token_count: 871
summary: ## Configure the Konnectivity service The following steps require an egress configuration, for example: [`admin/konnectivity/egress-selector-configuration.yaml`...
---

## Configure the Konnectivity service
The following steps require an egress configuration, for example:
[`admin/konnectivity/egress-selector-configuration.yaml`
](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/admin/konnectivity/egress-selector-configuration.yaml)![](/images/copycode.svg "Copy admin/konnectivity/egress-selector-configuration.yaml to clipboard")
```
`apiVersion: apiserver.k8s.io/v1beta1
kind: EgressSelectorConfiguration
egressSelections:
# Since we want to control the egress traffic to the cluster, we use the
# "cluster" as the name. Other supported values are "etcd", and "controlplane".
- name: cluster
connection:
# server. Supported values are "GRPC" and "HTTPConnect". There is no
# end user visible difference between the two modes. You need to set the
# Konnectivity server to work in the same mode.
proxyProtocol: GRPC
transport:
# This controls what transport the API Server uses to communicate with the
# Konnectivity server. UDS is recommended if the Konnectivity server
# locates on the same machine as the API Server. You need to configure the
# Konnectivity server to listen on the same UDS socket.
# The other supported transport is "tcp". You will need to set up TLS
# config to secure the TCP transport.
uds:
udsName: /etc/kubernetes/konnectivity-server/konnectivity-server.socket
`
```
You need to configure the API Server to use the Konnectivity service
and direct the network traffic to the cluster nodes:
1. Make sure that
[Service Account Token Volume Projection](/docs/tasks/configure-pod-container/configure-service-account/#serviceaccount-token-volume-projection)
feature enabled in your cluster. It is enabled by default since Kubernetes v1.20.
2. Create an egress configuration file such as `admin/konnectivity/egress-selector-configuration.yaml`.
3. Set the `--egress-selector-config-file` flag of the API Server to the path of
your API Server egress configuration file.
4. If you use UDS connection, add volumes config to the kube-apiserver:
```
`spec:
containers:
volumeMounts:
- name: konnectivity-uds
mountPath: /etc/kubernetes/konnectivity-server
readOnly: false
volumes:
- name: konnectivity-uds
hostPath:
path: /etc/kubernetes/konnectivity-server
type: DirectoryOrCreate
`
```
Generate or obtain a certificate and kubeconfig for konnectivity-server.
For example, you can use the OpenSSL command line tool to issue a X.509 certificate,
using the cluster CA certificate `/etc/kubernetes/pki/ca.crt` from a control-plane host.
```
`openssl req -subj "/CN=system:konnectivity-server" -new -newkey rsa:2048 -noenc -out konnectivity.csr -keyout konnectivity.key
openssl x509 -req -in konnectivity.csr -CA /etc/kubernetes/pki/ca.crt -CAkey /etc/kubernetes/pki/ca.key -CAcreateserial -out konnectivity.crt -days 375 -sha256
SERVER=$(kubectl config view -o jsonpath='{.clusters..server}')
kubectl --kubeconfig /etc/kubernetes/konnectivity-server.conf config set-credentials system:konnectivity-server --client-certificate konnectivity.crt --client-key konnectivity.key --embed-certs=true
kubectl --kubeconfig /etc/kubernetes/konnectivity-server.conf config set-cluster kubernetes --server "$SERVER" --certificate-authority /etc/kubernetes/pki/ca.crt --embed-certs=true
kubectl --kubeconfig /etc/kubernetes/konnectivity-server.conf config set-context system:konnectivity-server@kubernetes --cluster kubernetes --user system:konnectivity-server
kubectl --kubeconfig /etc/kubernetes/konnectivity-server.conf config use-context system:konnectivity-server@kubernetes
rm -f konnectivity.crt konnectivity.key konnectivity.csr
`
```