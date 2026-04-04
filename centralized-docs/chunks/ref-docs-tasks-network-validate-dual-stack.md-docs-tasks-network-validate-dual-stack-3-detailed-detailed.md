---
doc_id: ref/docs-tasks-network-validate-dual-stack.md/docs-tasks-network-validate-dual-stack
chunk_id: ref/docs-tasks-network-validate-dual-stack.md/docs-tasks-network-validate-dual-stack#3-detailed
chunk_level: detailed
chunk_type: code
heading: Related Pages
token_count: 941
summary: #### Note: The `kubectl get svc` command will only show the primary IP in the `CLUSTER-IP` field. ``` `kubectl get svc -l app.kubernetes.io/name=MyApp ` ``` ``` `NAME TYPE CLUSTER-IP EXTERNAL-IP...
---

#### Note:
The `kubectl get svc` command will only show the primary IP in the `CLUSTER-IP` field.
```
`kubectl get svc -l app.kubernetes.io/name=MyApp
`
```
```
`NAME TYPE CLUSTER-IP EXTERNAL-IP PORT(S) AGE
my-service ClusterIP 10.0.216.242 &lt;none&gt; 80/TCP 5s
`
```
Validate that the Service gets cluster IPs from the IPv4 and IPv6 address blocks using
`kubectl describe`. You may then validate access to the service via the IPs and ports.
```
`kubectl describe svc -l app.kubernetes.io/name=MyApp
`
```
```
`Name: my-service
Namespace: default
Labels: app.kubernetes.io/name=MyApp
Annotations: &lt;none&gt;
Selector: app.kubernetes.io/name=MyApp
Type: ClusterIP
IP Family Policy: PreferDualStack
IP Families: IPv4,IPv6
IP: 10.0.216.242
IPs: 10.0.216.242,2001:db8:fd00::af55
Port: &lt;unset&gt; 80/TCP
TargetPort: 9376/TCP
Endpoints: &lt;none&gt;
Session Affinity: None
Events: &lt;none&gt;
`
```
### Create a dual-stack load balanced Service
If the cloud provider supports the provisioning of IPv6 enabled external load balancers,
create the following Service with `PreferDualStack` in `.spec.ipFamilyPolicy`, `IPv6` as
the first element of the `.spec.ipFamilies` array and the `type` field set to `LoadBalancer`.
[`service/networking/dual-stack-prefer-ipv6-lb-svc.yaml`
](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/service/networking/dual-stack-prefer-ipv6-lb-svc.yaml)![](/images/copycode.svg "Copy service/networking/dual-stack-prefer-ipv6-lb-svc.yaml to clipboard")
```
`apiVersion: v1
kind: Service
metadata:
name: my-service
labels:
app.kubernetes.io/name: MyApp
spec:
ipFamilyPolicy: PreferDualStack
ipFamilies:
- IPv6
type: LoadBalancer
selector:
app.kubernetes.io/name: MyApp
ports:
- protocol: TCP
port: 80
`
```
Check the Service:
```
`kubectl get svc -l app.kubernetes.io/name=MyApp
`
```
Validate that the Service receives a `CLUSTER-IP` address from the IPv6 address block
along with an `EXTERNAL-IP`. You may then validate access to the service via the IP and port.
```
`NAME TYPE CLUSTER-IP EXTERNAL-IP PORT(S) AGE
my-service LoadBalancer 2001:db8:fd00::7ebc 2603:1030:805::5 80:30790/TCP 35s
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
Last modified January 23, 2025 at 9:50 AM PST: [Tweak two network tasks: validate-dual-stack and extend-service-ip-ranges (00c294a18a)](https://github.com/kubernetes/website/commit/00c294a18ab12c5ce1792195c80635c2c02b98c6)
## Related Pages

- [Documentation Content Guide](docs-contribute-style-content-guide.md)
- [Binding](docs-reference-kubernetes-api-workload-resources-binding-v1.md)
- [conventions](docs-reference-kubectl-conventions.md)
- [HorizontalPodAutoscaler](docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2.md)
- [Концепции](ru-docs-concepts.md)