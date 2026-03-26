---
doc_id: tutorial/docs-tasks-administer-cluster-access-cluster-api.md/docs-tasks-administer-cluster-access-cluster-api
chunk_id: tutorial/docs-tasks-administer-cluster-access-cluster-api.md/docs-tasks-administer-cluster-access-cluster-api#10-standard
chunk_level: standard
chunk_type: prose
heading: Before you begin
token_count: 460
summary: #### dotnet client To use [dotnet client](https://github.com/kubernetes-client/csharp), run the following command: `dotnet add package KubernetesClient --version 1.6.1`. See [dotnet Client Library...
---

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