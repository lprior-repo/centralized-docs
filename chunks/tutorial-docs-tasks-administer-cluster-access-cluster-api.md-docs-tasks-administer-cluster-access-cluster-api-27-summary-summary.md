---
doc_id: tutorial/docs-tasks-administer-cluster-access-cluster-api.md/docs-tasks-administer-cluster-access-cluster-api
chunk_id: tutorial/docs-tasks-administer-cluster-access-cluster-api.md/docs-tasks-administer-cluster-access-cluster-api#27-summary
chunk_level: summary
chunk_type: prose
heading: Before you begin
token_count: 116
summary: // creates the clientset clientset, \_ := kubernetes.NewForConfig(config) // access the API to list pods pods, \_ := clientset.CoreV1().Pods(\"\").List(context.TODO(), v1.ListOptions{})...
---

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