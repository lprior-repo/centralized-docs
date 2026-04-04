---
doc_id: ref/docs-tasks-network-validate-dual-stack.md/docs-tasks-network-validate-dual-stack
chunk_id: ref/docs-tasks-network-validate-dual-stack.md/docs-tasks-network-validate-dual-stack#4-standard
chunk_level: standard
chunk_type: prose
heading: Validate Services
token_count: 487
summary: ## Validate Services Create the following Service that does not explicitly define `.spec.ipFamilyPolicy`. Kubernetes will assign a cluster IP for the Service from the first configured...
---

## Validate Services
Create the following Service that does not explicitly define `.spec.ipFamilyPolicy`.
Kubernetes will assign a cluster IP for the Service from the first configured
`service-cluster-ip-range` and set the `.spec.ipFamilyPolicy` to `SingleStack`.
[`service/networking/dual-stack-default-svc.yaml`
](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/service/networking/dual-stack-default-svc.yaml)![](/images/copycode.svg "Copy service/networking/dual-stack-default-svc.yaml to clipboard")
```
`apiVersion: v1
kind: Service
metadata:
name: my-service
labels:
app.kubernetes.io/name: MyApp
spec:
selector:
app.kubernetes.io/name: MyApp
ports:
- protocol: TCP
port: 80
`
```
Use `kubectl` to view the YAML for the Service.
```
`kubectl get svc my-service -o yaml
`
```
The Service has `.spec.ipFamilyPolicy` set to `SingleStack` and `.spec.clusterIP` set
to an IPv4 address from the first configured range set via `--service-cluster-ip-range`
flag on kube-controller-manager.
```
`apiVersion: v1
kind: Service
metadata:
name: my-service
namespace: default
spec:
clusterIP: 10.0.217.164
clusterIPs:
- 10.0.217.164
ipFamilies:
- IPv4
ipFamilyPolicy: SingleStack
ports:
- port: 80
protocol: TCP
targetPort: 9376
selector:
app.kubernetes.io/name: MyApp
sessionAffinity: None
type: ClusterIP
status:
loadBalancer: {}
`
```
Create the following Service that explicitly defines `IPv6` as the first array element in
`.spec.ipFamilies`. Kubernetes will assign a cluster IP for the Service from the IPv6 range
configured `service-cluster-ip-range` and set the `.spec.ipFamilyPolicy` to `SingleStack`.
[`service/networking/dual-stack-ipfamilies-ipv6.yaml`
](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/service/networking/dual-stack-ipfamilies-ipv6.yaml)![](/images/copycode.svg "Copy service/networking/dual-stack-ipfamilies-ipv6.yaml to clipboard")