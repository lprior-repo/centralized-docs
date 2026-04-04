---
doc_id: ref/docs-tasks-network-validate-dual-stack.md/docs-tasks-network-validate-dual-stack
chunk_id: ref/docs-tasks-network-validate-dual-stack.md/docs-tasks-network-validate-dual-stack#2-detailed
chunk_level: detailed
chunk_type: code
heading: Validate Services
token_count: 960
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
```
`apiVersion: v1
kind: Service
metadata:
name: my-service
labels:
app.kubernetes.io/name: MyApp
spec:
ipFamilies:
- IPv6
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
The Service has `.spec.ipFamilyPolicy` set to `SingleStack` and `.spec.clusterIP` set to
an IPv6 address from the IPv6 range set via `--service-cluster-ip-range` flag on kube-controller-manager.
```
`apiVersion: v1
kind: Service
metadata:
labels:
app.kubernetes.io/name: MyApp
name: my-service
spec:
clusterIP: 2001:db8:fd00::5118
clusterIPs:
- 2001:db8:fd00::5118
ipFamilies:
- IPv6
ipFamilyPolicy: SingleStack
ports:
- port: 80
protocol: TCP
targetPort: 80
selector:
app.kubernetes.io/name: MyApp
sessionAffinity: None
type: ClusterIP
status:
loadBalancer: {}
`
```
Create the following Service that explicitly defines `PreferDualStack` in `.spec.ipFamilyPolicy`.
Kubernetes will assign both IPv4 and IPv6 addresses (as this cluster has dual-stack enabled) and
select the `.spec.ClusterIP` from the list of `.spec.ClusterIPs` based on the address family of
the first element in the `.spec.ipFamilies` array.
[`service/networking/dual-stack-preferred-svc.yaml`
](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/service/networking/dual-stack-preferred-svc.yaml)![](/images/copycode.svg "Copy service/networking/dual-stack-preferred-svc.yaml to clipboard")
```
`apiVersion: v1
kind: Service
metadata:
name: my-service
labels:
app.kubernetes.io/name: MyApp
spec:
ipFamilyPolicy: PreferDualStack
selector:
app.kubernetes.io/name: MyApp
ports:
- protocol: TCP
port: 80
`
```