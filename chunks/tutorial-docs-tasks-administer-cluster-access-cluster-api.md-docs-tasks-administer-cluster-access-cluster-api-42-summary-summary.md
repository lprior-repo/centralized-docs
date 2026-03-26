---
doc_id: tutorial/docs-tasks-administer-cluster-access-cluster-api.md/docs-tasks-administer-cluster-access-cluster-api
chunk_id: tutorial/docs-tasks-administer-cluster-access-cluster-api.md/docs-tasks-administer-cluster-access-cluster-api#42-summary
chunk_level: summary
chunk_type: prose
heading: Before you begin
token_count: 78
summary: ``` `const k8s = require('@kubernetes/client-node'); const kc = new k8s.KubeConfig(); kc.loadFromDefault(); const k8sApi = kc.makeApiClient(k8s.CoreV1Api); k8sApi.listNamespacedPod({ namespace:...
---

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