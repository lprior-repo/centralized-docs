---
doc_id: tutorial/docs-tasks-administer-cluster-access-cluster-api.md/docs-tasks-administer-cluster-access-cluster-api
chunk_id: tutorial/docs-tasks-administer-cluster-access-cluster-api.md/docs-tasks-administer-cluster-access-cluster-api#34-summary
chunk_level: summary
chunk_type: prose
heading: Before you begin
token_count: 127
summary: /\*\* \* A simple example of how to use the Java API from an application outside a kubernetes cluster \* \* &lt;p&gt;Easiest way to run this: mvn exec:java \*...
---

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