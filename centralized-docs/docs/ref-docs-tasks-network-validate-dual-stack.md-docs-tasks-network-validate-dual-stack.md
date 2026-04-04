---
id: ref/docs-tasks-network-validate-dual-stack.md/docs-tasks-network-validate-dual-stack
title: Validate IPv4/IPv6 dual-stack
category: ref
tags: ["before", "begin", "contents", "dual-stack", "ipv4/ipv6"]
---

## Table of Contents

* [Validate IPv4/IPv6 dual-stack](#validate-ipv4ipv6-dual-stack)
  * [Before you begin](#before-you-begin)
    * [Note:](#note)
    * [Validate node addressing](#validate-node-addressing)
    * [Validate Pod addressing](#validate-pod-addressing)
  * [Validate Services](#validate-services)
    * [Note:](#note)
    * [Create a dual-stack load balanced Service](#create-a-dual-stack-load-balanced-service)
  * [Feedback](#feedback)

---

# Validate IPv4/IPv6 dual-stack



 > 
 > **Context**: This document shares how to validate IPv4/IPv6 dual-stack enabled Kubernetes clusters.



This document shares how to validate IPv4/IPv6 dual-stack enabled Kubernetes clusters.

## Before you begin

* Provider support for dual-stack networking (Cloud provider or otherwise must be able to
  provide Kubernetes nodes with routable IPv4/IPv6 network interfaces)
* A [network plugin](/docs/concepts/extend-kubernetes/compute-storage-net/network-plugins/)
  that supports dual-stack networking.
* [Dual-stack enabled](/docs/concepts/services-networking/dual-stack/) clusterYour Kubernetes server must be at or later than version v1.23.
  To check the version, enter `kubectl version`.

### Note:

While you can validate with an earlier version, the feature is only GA and officially supported since v1.23.

### Validate node addressing

Each dual-stack Node should have a single IPv4 block and a single IPv6 block allocated.
Validate that IPv4/IPv6 Pod address ranges are configured by running the following command.
Replace the sample node name with a valid dual-stack Node from your cluster. In this example,
the Node’s name is `k8s-linuxpool1-34450317-0`:

````
`kubectl get nodes k8s-linuxpool1-34450317-0 -o go-template --template='{{range .spec.podCIDRs}}{{printf "%s\\n" .}}{{end}}'
`
````

````
`10.244.1.0/24
2001:db8::/64
`
````

There should be one IPv4 block and one IPv6 block allocated.
Validate that the node has an IPv4 and IPv6 interface detected.
Replace node name with a valid node from the cluster.
In this example the node name is `k8s-linuxpool1-34450317-0`:

````
`kubectl get nodes k8s-linuxpool1-34450317-0 -o go-template --template='{{range .status.addresses}}{{printf "%s: %s\\n" .type .address}}{{end}}'
`
````

````
`Hostname: k8s-linuxpool1-34450317-0
InternalIP: 10.0.0.5
InternalIP: 2001:db8:10::5
`
````

### Validate Pod addressing

Validate that a Pod has an IPv4 and IPv6 address assigned. Replace the Pod name with
a valid Pod in your cluster. In this example the Pod name is `pod01`:

````
`kubectl get pods pod01 -o go-template --template='{{range .status.podIPs}}{{printf "%s\\n" .ip}}{{end}}'
`
````

````
`10.244.1.4
2001:db8::4
`
````

You can also validate Pod IPs using the Downward API via the `status.podIPs` fieldPath.
The following snippet demonstrates how you can expose the Pod IPs via an environment variable
called `MY\_POD\_IPS` within a container.

````
` env:
- name: MY\_POD\_IPS
valueFrom:
fieldRef:
fieldPath: status.podIPs
`
````

The following command prints the value of the `MY\_POD\_IPS` environment variable from
within a container. The value is a comma separated list that corresponds to the
Pod’s IPv4 and IPv6 addresses.

````
`kubectl exec -it pod01 -- set | grep MY\_POD\_IPS
`
````

````
`MY\_POD\_IPS=10.244.1.4,2001:db8::4
`
````

The Pod’s IP addresses will also be written to `/etc/hosts` within a container.
The following command executes a cat on `/etc/hosts` on a dual stack Pod.
From the output you can verify both the IPv4 and IPv6 IP address for the Pod.

````
`kubectl exec -it pod01 -- cat /etc/hosts
`
````

````
`# Kubernetes-managed hosts file.
127.0.0.1 localhost
::1 localhost ip6-localhost ip6-loopback
fe00::0 ip6-localnet
fe00::0 ip6-mcastprefix
fe00::1 ip6-allnodes
fe00::2 ip6-allrouters
10.244.1.4 pod01
2001:db8::4 pod01
`
````

## Validate Services

Create the following Service that does not explicitly define `.spec.ipFamilyPolicy`.
Kubernetes will assign a cluster IP for the Service from the first configured
`service-cluster-ip-range` and set the `.spec.ipFamilyPolicy` to `SingleStack`.
[`service/networking/dual-stack-default-svc.yaml`
](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/service/networking/dual-stack-default-svc.yaml)![](/images/copycode.svg "Copy service/networking/dual-stack-default-svc.yaml to clipboard")

````
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
````

Use `kubectl` to view the YAML for the Service.

````
`kubectl get svc my-service -o yaml
`
````

The Service has `.spec.ipFamilyPolicy` set to `SingleStack` and `.spec.clusterIP` set
to an IPv4 address from the first configured range set via `--service-cluster-ip-range`
flag on kube-controller-manager.

````
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
````

Create the following Service that explicitly defines `IPv6` as the first array element in
`.spec.ipFamilies`. Kubernetes will assign a cluster IP for the Service from the IPv6 range
configured `service-cluster-ip-range` and set the `.spec.ipFamilyPolicy` to `SingleStack`.
[`service/networking/dual-stack-ipfamilies-ipv6.yaml`
](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/service/networking/dual-stack-ipfamilies-ipv6.yaml)![](/images/copycode.svg "Copy service/networking/dual-stack-ipfamilies-ipv6.yaml to clipboard")

````
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
````

Use `kubectl` to view the YAML for the Service.

````
`kubectl get svc my-service -o yaml
`
````

The Service has `.spec.ipFamilyPolicy` set to `SingleStack` and `.spec.clusterIP` set to
an IPv6 address from the IPv6 range set via `--service-cluster-ip-range` flag on kube-controller-manager.

````
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
````

Create the following Service that explicitly defines `PreferDualStack` in `.spec.ipFamilyPolicy`.
Kubernetes will assign both IPv4 and IPv6 addresses (as this cluster has dual-stack enabled) and
select the `.spec.ClusterIP` from the list of `.spec.ClusterIPs` based on the address family of
the first element in the `.spec.ipFamilies` array.
[`service/networking/dual-stack-preferred-svc.yaml`
](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/service/networking/dual-stack-preferred-svc.yaml)![](/images/copycode.svg "Copy service/networking/dual-stack-preferred-svc.yaml to clipboard")

````
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
````

### Note:

The `kubectl get svc` command will only show the primary IP in the `CLUSTER-IP` field.

````
`kubectl get svc -l app.kubernetes.io/name=MyApp
`
````

````
`NAME TYPE CLUSTER-IP EXTERNAL-IP PORT(S) AGE
my-service ClusterIP 10.0.216.242 &lt;none&gt; 80/TCP 5s
`
````

Validate that the Service gets cluster IPs from the IPv4 and IPv6 address blocks using
`kubectl describe`. You may then validate access to the service via the IPs and ports.

````
`kubectl describe svc -l app.kubernetes.io/name=MyApp
`
````

````
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
````

### Create a dual-stack load balanced Service

If the cloud provider supports the provisioning of IPv6 enabled external load balancers,
create the following Service with `PreferDualStack` in `.spec.ipFamilyPolicy`, `IPv6` as
the first element of the `.spec.ipFamilies` array and the `type` field set to `LoadBalancer`.
[`service/networking/dual-stack-prefer-ipv6-lb-svc.yaml`
](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/service/networking/dual-stack-prefer-ipv6-lb-svc.yaml)![](/images/copycode.svg "Copy service/networking/dual-stack-prefer-ipv6-lb-svc.yaml to clipboard")

````
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
````

Check the Service:

````
`kubectl get svc -l app.kubernetes.io/name=MyApp
`
````

Validate that the Service receives a `CLUSTER-IP` address from the IPv6 address block
along with an `EXTERNAL-IP`. You may then validate access to the service via the IP and port.

````
`NAME TYPE CLUSTER-IP EXTERNAL-IP PORT(S) AGE
my-service LoadBalancer 2001:db8:fd00::7ebc 2603:1030:805::5 80:30790/TCP 35s
`
````

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

* [Documentation Content Guide](./ref-docs-contribute-style-content-guide.md-docs-contribute-style-content-guide.md)
* [Binding](./ref-docs-reference-kubernetes-api-workload-resources-binding-v1.md-docs-reference-kubernetes-api-workload-resources-binding-v1.md)
* [conventions](./ref-docs-reference-kubectl-conventions.md-docs-reference-kubectl-conventions.md)
* [HorizontalPodAutoscaler](./ref-docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2.md-docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2.md)
* [Концепции](./ref-ru-docs-concepts.md-ru-docs-concepts.md)
## See Also

- [Documentation Index](./COMPASS.md)
