---
doc_id: tutorial/docs-tasks-administer-cluster-access-cluster-api.md/docs-tasks-administer-cluster-access-cluster-api
chunk_id: tutorial/docs-tasks-administer-cluster-access-cluster-api.md/docs-tasks-administer-cluster-access-cluster-api#5-detailed
chunk_level: detailed
chunk_type: prose
heading: Before you begin
token_count: 993
summary: #### Python client To use [Python client](https://github.com/kubernetes-client/python), run the following command: `pip install kubernetes`. See [Python Client Library...
---

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