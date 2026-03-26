---
doc_id: tutorial/docs-tasks-administer-cluster-access-cluster-api.md/docs-tasks-administer-cluster-access-cluster-api
chunk_id: tutorial/docs-tasks-administer-cluster-access-cluster-api.md/docs-tasks-administer-cluster-access-cluster-api#44-summary
chunk_level: summary
chunk_type: prose
heading: What's next
token_count: 125
summary: ``` `exampleWithKubeConfig :: IO () exampleWithKubeConfig = do oidcCache &lt;- atomically $ newTVar $ Map.fromList [] (mgr, kcfg) &lt;- mkKubeClientConfig oidcCache $ KubeConfigFile...
---

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