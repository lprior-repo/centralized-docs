---
doc_id: tutorial/docs-tasks-administer-cluster-access-cluster-api.md/docs-tasks-administer-cluster-access-cluster-api
chunk_id: tutorial/docs-tasks-administer-cluster-access-cluster-api.md/docs-tasks-administer-cluster-access-cluster-api#36-summary
chunk_level: summary
chunk_type: prose
heading: Before you begin
token_count: 86
summary: CoreV1Api api = new CoreV1Api(); // invokes the CoreV1Api client V1PodList list = api.listPodForAllNamespaces(null, null, null, null, null, null, null, null, null); System.out.println(\"Listing all...
---

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