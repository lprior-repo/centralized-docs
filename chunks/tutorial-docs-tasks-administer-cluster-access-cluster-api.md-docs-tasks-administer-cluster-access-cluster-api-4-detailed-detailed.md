---
doc_id: tutorial/docs-tasks-administer-cluster-access-cluster-api.md/docs-tasks-administer-cluster-access-cluster-api
chunk_id: tutorial/docs-tasks-administer-cluster-access-cluster-api.md/docs-tasks-administer-cluster-access-cluster-api#4-detailed
chunk_level: detailed
chunk_type: prose
heading: Before you begin
token_count: 705
summary: ### Programmatic access to the API Kubernetes officially supports client libraries for [Go](#go-client), [Python](#python-client), [Java](#java-client), [dotnet](#dotnet-client),...
---

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