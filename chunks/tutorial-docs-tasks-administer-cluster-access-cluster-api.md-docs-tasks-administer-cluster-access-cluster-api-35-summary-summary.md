---
doc_id: tutorial/docs-tasks-administer-cluster-access-cluster-api.md/docs-tasks-administer-cluster-access-cluster-api
chunk_id: tutorial/docs-tasks-administer-cluster-access-cluster-api.md/docs-tasks-administer-cluster-access-cluster-api#35-summary
chunk_level: summary
chunk_type: prose
heading: Before you begin
token_count: 104
summary: String kubeConfigPath = \"\~/.kube/config\"; // loading the out-of-cluster config, a kubeconfig from file-system ApiClient client = ClientBuilder.kubeconfig(KubeConfig.loadKubeConfig(new...
---

String kubeConfigPath = "\~/.kube/config";
// loading the out-of-cluster config, a kubeconfig from file-system
ApiClient client =
ClientBuilder.kubeconfig(KubeConfig.loadKubeConfig(new FileReader(kubeConfigPath))).build();
// set the global default api-client to the in-cluster one from above
Configuration.setDefaultApiClient(client);
// the CoreV1Api loads default api-client from global configuration.
CoreV1Api api = new CoreV1Api();
// invokes the CoreV1Api client