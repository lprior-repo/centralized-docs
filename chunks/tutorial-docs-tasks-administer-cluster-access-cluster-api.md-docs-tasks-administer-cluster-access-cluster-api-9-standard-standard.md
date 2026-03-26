---
doc_id: tutorial/docs-tasks-administer-cluster-access-cluster-api.md/docs-tasks-administer-cluster-access-cluster-api
chunk_id: tutorial/docs-tasks-administer-cluster-access-cluster-api.md/docs-tasks-administer-cluster-access-cluster-api#9-standard
chunk_level: standard
chunk_type: prose
heading: Before you begin
token_count: 367
summary: ``` `package io.kubernetes.client.examples; import io.kubernetes.client.ApiClient; import io.kubernetes.client.ApiException; import io.kubernetes.client.Configuration; import...
---

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