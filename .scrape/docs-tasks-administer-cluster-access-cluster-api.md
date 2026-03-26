---
url: https://kubernetes.io/docs/tasks/administer-cluster/access-cluster-api/
title: Access Clusters Using the Kubernetes API
word_count: 1582
filtered: true
elements_removed: 0
density_score: 0.94
---

## Table of Contents

- [Access Clusters Using the Kubernetes API](#access-clusters-using-the-kubernetes-api)
  - [Before you begin](#before-you-begin)
    - [Accessing for the first time with kubectl](#accessing-for-the-first-time-with-kubectl)
    - [Directly accessing the REST API](#directly-accessing-the-rest-api)
      - [Using kubectl proxy](#using-kubectl-proxy)
      - [Without kubectl proxy](#without-kubectl-proxy)
- [Select name of cluster you want to interact with from above output:](#select-name-of-cluster-you-want-to-interact-with-from-above-output)
- [Point to the API server referring the cluster name](#point-to-the-api-server-referring-the-cluster-name)
- [Create a secret to hold a token for the default service account](#create-a-secret-to-hold-a-token-for-the-default-service-account)
- [Wait for the token controller to populate the secret with a token:](#wait-for-the-token-controller-to-populate-the-secret-with-a-token)
- [Get the token value](#get-the-token-value)
- [Explore the API with TOKEN](#explore-the-api-with-token)
    - [Programmatic access to the API](#programmatic-access-to-the-api)
      - [Go client](#go-client)
      - [Note:](#note)
      - [Python client](#python-client)
      - [Java client](#java-client)
- [Installing project artifacts, POM etc:](#installing-project-artifacts-pom-etc)
      - [dotnet client](#dotnet-client)
      - [JavaScript client](#javascript-client)
      - [Haskell client](#haskell-client)
  - [What's next](#whats-next)
  - [Feedback](#feedback)

---

# Access Clusters Using the Kubernetes API
This page shows how to access clusters using the Kubernetes API.
## Before you begin
You need to have a Kubernetes cluster, and the kubectl command-line tool must
be configured to communicate with your cluster. It is recommended to run this tutorial on a cluster with at least two nodes that are not acting as control plane hosts. If you do not already have a
cluster, you can create one by using
[minikube](https://minikube.sigs.k8s.io/docs/tutorials/multi_node/)
or you can use one of these Kubernetes playgrounds:
* [iximiuz Labs](https://labs.iximiuz.com/playgrounds?category=kubernetes&amp;filter=all)
* [Killercoda](https://killercoda.com/playgrounds/scenario/kubernetes)
* [KodeKloud](https://kodekloud.com/public-playgrounds)
To check the version, enter `kubectl version`.
### Accessing for the first time with kubectl
When accessing the Kubernetes API for the first time, use the
Kubernetes command-line tool, `kubectl`.
To access a cluster, you need to know the location of the cluster and have credentials
to access it. Typically, this is automatically set-up when you work through
a [Getting started guide](/docs/setup/),
or someone else set up the cluster and provided you with credentials and a location.
Check the location and credentials that kubectl knows about with this command:
```
`kubectl config view
`
```
Many of the [examples](https://github.com/kubernetes/examples/tree/master/) provide an introduction to using
kubectl. Complete documentation is found in the [kubectl manual](/docs/reference/kubectl/).
### Directly accessing the REST API
kubectl handles locating and authenticating to the API server. If you want to directly access the REST API with an http client like
`curl` or `wget`, or a browser, there are multiple ways you can locate and authenticate against the API server:
1. Run kubectl in proxy mode (recommended). This method is recommended, since it uses
the stored API server location and verifies the identity of the API server using a
self-signed certificate. No man-in-the-middle (MITM) attack is possible using this method.
2. Alternatively, you can provide the location and credentials directly to the http client.
This works with client code that is confused by proxies. To protect against man in the
middle attacks, you'll need to import a root cert into your browser.
Using the Go or Python client libraries provides accessing kubectl in proxy mode.
#### Using kubectl proxy
The following command runs kubectl in a mode where it acts as a reverse proxy. It handles
locating the API server and authenticating.
Run it like this:
```
`kubectl proxy --port=8080 &amp;
`
```
See [kubectl proxy](/docs/reference/generated/kubectl/kubectl-commands/#proxy) for more details.
Then you can explore the API with curl, wget, or a browser, like so:
```
`curl http://localhost:8080/api/
`
```
The output is similar to this:
```
`{
"versions": [
"v1"
],
"serverAddressByClientCIDRs": [
{
"clientCIDR": "0.0.0.0/0",
"serverAddress": "10.0.1.149:443"
}
]
}
`
```
#### Without kubectl proxy
It is possible to avoid using kubectl proxy by passing an authentication token
directly to the API server, like this:
Using `grep/cut` approach:
```
`# Check all possible clusters, as your .KUBECONFIG may have multiple contexts:
kubectl config view -o jsonpath='{"Cluster name\\tServer\\n"}{range .clusters[\*]}{.name}{"\\t"}{.cluster.server}{"\\n"}{end}'
# Select name of cluster you want to interact with from above output:
export CLUSTER\_NAME="some\_server\_name"
# Point to the API server referring the cluster name
APISERVER=$(kubectl config view -o jsonpath="{.clusters[?(@.name==\\"$CLUSTER\_NAME\\")].cluster.server}")
# Create a secret to hold a token for the default service account
kubectl apply -f - &lt;&lt;EOF
apiVersion: v1
kind: Secret
metadata:
name: default-token
annotations:
kubernetes.io/service-account.name: default
type: kubernetes.io/service-account-token
EOF
# Wait for the token controller to populate the secret with a token:
while ! kubectl describe secret default-token | grep -E '^token' &gt;/dev/null; do
echo "waiting for token..." &gt;&amp;2
sleep 1
done
# Get the token value
TOKEN=$(kubectl get secret default-token -o jsonpath='{.data.token}' | base64 --decode)
# Explore the API with TOKEN
curl -X GET $APISERVER/api --header "Authorization: Bearer $TOKEN" --insecure
`
```
The output is similar to this:
```
`{
"kind": "APIVersions",
"versions": [
"v1"
],
"serverAddressByClientCIDRs": [
{
"clientCIDR": "0.0.0.0/0",
"serverAddress": "10.0.1.149:443"
}
]
}
`
```
The above example uses the `--insecure` flag. This leaves it subject to MITM
attacks. When kubectl accesses the cluster it uses a stored root certificate
and client certificates to access the server. (These are installed in the
`\~/.kube` directory). Since cluster certificates are typically self-signed, it
may take special configuration to get your http client to use root
certificate.
On some clusters, the API server does not require authentication; it may serve
on localhost, or be protected by a firewall. There is not a standard
for this. [Controlling Access to the Kubernetes API](/docs/concepts/security/controlling-access/)
describes how you can configure this as a cluster administrator.
### Programmatic access to the API
Kubernetes officially supports client libraries for [Go](#go-client), [Python](#python-client),
[Java](#java-client), [dotnet](#dotnet-client), [JavaScript](#javascript-client), and
[Haskell](#haskell-client). There are other client libraries that are provided and maintained by
their authors, not the Kubernetes team. See [client libraries](/docs/reference/using-api/client-libraries/)
for accessing the API from other languages and how they authenticate.
#### Go client
* To get the library, run the following command: `go get k8s.io/client-go@kubernetes-&lt;kubernetes-version-number&gt;`
See [https://github.com/kubernetes/client-go/releases](https://github.com/kubernetes/client-go/releases)
to see which versions are supported.
* Write an application atop of the client-go clients.
#### Note:
`client-go` defines its own API objects, so if needed, import API definitions from client-go rather than
from the main repository. For example, `import "k8s.io/client-go/kubernetes"` is correct.
The Go client can use the same [kubeconfig file](/docs/concepts/configuration/organize-cluster-access-kubeconfig/)
as the kubectl CLI does to locate and authenticate to the API server. See this [example](https://git.k8s.io/client-go/examples/out-of-cluster-client-configuration/main.go):
```
`package main
import (
"context"
"fmt"
"k8s.io/apimachinery/pkg/apis/meta/v1"
"k8s.io/client-go/kubernetes"
"k8s.io/client-go/tools/clientcmd"
)
func main() {
// uses the current context in kubeconfig
// path-to-kubeconfig -- for example, /root/.kube/config
config, \_ := clientcmd.BuildConfigFromFlags("", "&lt;path-to-kubeconfig&gt;")
// creates the clientset
clientset, \_ := kubernetes.NewForConfig(config)
// access the API to list pods
pods, \_ := clientset.CoreV1().Pods("").List(context.TODO(), v1.ListOptions{})
fmt.Printf("There are %d pods in the cluster\\n", len(pods.Items))
}
`
```
If the application is deployed as a Pod in the cluster, see
[Accessing the API from within a Pod](/docs/tasks/access-application-cluster/access-cluster/#accessing-the-api-from-a-pod).
#### Python client
To use [Python client](https://github.com/kubernetes-client/python), run the following command:
`pip install kubernetes`. See [Python Client Library page](https://github.com/kubernetes-client/python)
for more installation options.
The Python client can use the same [kubeconfig file](/docs/concepts/configuration/organize-cluster-access-kubeconfig/)
as the kubectl CLI does to locate and authenticate to the API server. See this
[example](https://github.com/kubernetes-client/python/blob/master/examples/out_of_cluster_config.py):
```
`from kubernetes import client, config
config.load\_kube\_config()
v1=client.CoreV1Api()
print("Listing pods with their IPs:")
ret = v1.list\_pod\_for\_all\_namespaces(watch=False)
for i in ret.items:
print("%s\\t%s\\t%s" % (i.status.pod\_ip, i.metadata.namespace, i.metadata.name))
`
```
#### Java client
To install the [Java Client](https://github.com/kubernetes-client/java), run:
```
`# Clone java library
git clone --recursive https://github.com/kubernetes-client/java
# Installing project artifacts, POM etc:
cd java
mvn install
`
```
See [https://github.com/kubernetes-client/java/releases](https://github.com/kubernetes-client/java/releases)
to see which versions are supported.
The Java client can use the same [kubeconfig file](/docs/concepts/configuration/organize-cluster-access-kubeconfig/)
as the kubectl CLI does to locate and authenticate to the API server. See this
[example](https://github.com/kubernetes-client/java/blob/master/examples/examples-release-15/src/main/java/io/kubernetes/client/examples/KubeConfigFileClientExample.java):
```
`package io.kubernetes.client.examples;
import io.kubernetes.client.ApiClient;
import io.kubernetes.client.ApiException;
import io.kubernetes.client.Configuration;
import io.kubernetes.client.apis.CoreV1Api;
import io.kubernetes.client.models.V1Pod;
import io.kubernetes.client.models.V1PodList;
import io.kubernetes.client.util.ClientBuilder;
import io.kubernetes.client.util.KubeConfig;
import java.io.FileReader;
import java.io.IOException;
/\*\*
\* A simple example of how to use the Java API from an application outside a kubernetes cluster
\*
\* &lt;p&gt;Easiest way to run this: mvn exec:java
\* -Dexec.mainClass="io.kubernetes.client.examples.KubeConfigFileClientExample"
\*
\*/
public class KubeConfigFileClientExample {
public static void main(String[] args) throws IOException, ApiException {
// file path to your KubeConfig
String kubeConfigPath = "\~/.kube/config";
// loading the out-of-cluster config, a kubeconfig from file-system
ApiClient client =
ClientBuilder.kubeconfig(KubeConfig.loadKubeConfig(new FileReader(kubeConfigPath))).build();
// set the global default api-client to the in-cluster one from above
Configuration.setDefaultApiClient(client);
// the CoreV1Api loads default api-client from global configuration.
CoreV1Api api = new CoreV1Api();
// invokes the CoreV1Api client
V1PodList list = api.listPodForAllNamespaces(null, null, null, null, null, null, null, null, null);
System.out.println("Listing all pods: ");
for (V1Pod item : list.getItems()) {
System.out.println(item.getMetadata().getName());
}
}
}
`
```
#### dotnet client
To use [dotnet client](https://github.com/kubernetes-client/csharp),
run the following command: `dotnet add package KubernetesClient --version 1.6.1`.
See [dotnet Client Library page](https://github.com/kubernetes-client/csharp)
for more installation options. See
[https://github.com/kubernetes-client/csharp/releases](https://github.com/kubernetes-client/csharp/releases)
to see which versions are supported.
The dotnet client can use the same [kubeconfig file](/docs/concepts/configuration/organize-cluster-access-kubeconfig/)
as the kubectl CLI does to locate and authenticate to the API server. See this
[example](https://github.com/kubernetes-client/csharp/blob/master/examples/simple/PodList.cs):
```
`using System;
using k8s;
namespace simple
{
internal class PodList
{
private static void Main(string[] args)
{
var config = KubernetesClientConfiguration.BuildDefaultConfig();
IKubernetes client = new Kubernetes(config);
Console.WriteLine("Starting Request!");
var list = client.ListNamespacedPod("default");
foreach (var item in list.Items)
{
Console.WriteLine(item.Metadata.Name);
}
if (list.Items.Count == 0)
{
Console.WriteLine("Empty!");
}
}
}
}
`
```
#### JavaScript client
To install [JavaScript client](https://github.com/kubernetes-client/javascript),
run the following command: `npm install @kubernetes/client-node`. See
[https://github.com/kubernetes-client/javascript/releases](https://github.com/kubernetes-client/javascript/releases)
to see which versions are supported.
The JavaScript client can use the same [kubeconfig file](/docs/concepts/configuration/organize-cluster-access-kubeconfig/)
as the kubectl CLI does to locate and authenticate to the API server. See this
[example](https://github.com/kubernetes-client/javascript/blob/master/examples/example.js):
```
`const k8s = require('@kubernetes/client-node');
const kc = new k8s.KubeConfig();
kc.loadFromDefault();
const k8sApi = kc.makeApiClient(k8s.CoreV1Api);
k8sApi.listNamespacedPod({ namespace: 'default' }).then((res) =&gt; {
console.log(res);
});
`
```
#### Haskell client
See [https://github.com/kubernetes-client/haskell/releases](https://github.com/kubernetes-client/haskell/releases)
to see which versions are supported.
The [Haskell client](https://github.com/kubernetes-client/haskell) can use the same
[kubeconfig file](/docs/concepts/configuration/organize-cluster-access-kubeconfig/)
as the kubectl CLI does to locate and authenticate to the API server. See this
[example](https://github.com/kubernetes-client/haskell/blob/master/kubernetes-client/example/App.hs):
```
`exampleWithKubeConfig :: IO ()
exampleWithKubeConfig = do
oidcCache &lt;- atomically $ newTVar $ Map.fromList []
(mgr, kcfg) &lt;- mkKubeClientConfig oidcCache $ KubeConfigFile "/path/to/kubeconfig"
dispatchMime
mgr
kcfg
(CoreV1.listPodForAllNamespaces (Accept MimeJSON))
&gt;&gt;= print
`
```
## What's next
* [Accessing the Kubernetes API from a Pod](/docs/tasks/run-application/access-api-from-pod/)
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
Last modified May 13, 2025 at 11:27 AM PST: [Fix incorrect usage of listNamespacedPod in JavaScript client example (7edfd2dfbc)](https://github.com/kubernetes/website/commit/7edfd2dfbceee3efa1750f0240c8f04cb61708d8)
## Related Pages

- [deploy intro](docs-tutorials-kubernetes-basics-deploy-app-deploy-intro.md)
- [Hello Minikube](docs-tutorials-hello-minikube.md)
- [kubectl](docs-reference-kubectl-kubectl.md)
- [Creating a cluster with kubeadm](docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm.md)
- [kubectl](docs-reference-kubectl-generated-kubectl.md)
