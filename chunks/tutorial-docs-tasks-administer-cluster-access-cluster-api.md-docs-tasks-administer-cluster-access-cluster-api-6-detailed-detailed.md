---
doc_id: tutorial/docs-tasks-administer-cluster-access-cluster-api.md/docs-tasks-administer-cluster-access-cluster-api
chunk_id: tutorial/docs-tasks-administer-cluster-access-cluster-api.md/docs-tasks-administer-cluster-access-cluster-api#6-detailed
chunk_level: detailed
chunk_type: prose
heading: Related Pages
token_count: 730
summary: #### JavaScript client To install [JavaScript client](https://github.com/kubernetes-client/javascript), run the following command: `npm install @kubernetes/client-node`. See...
---

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
#### Haskell client
See [https://github.com/kubernetes-client/haskell/releases](https://github.com/kubernetes-client/haskell/releases)
to see which versions are supported.
The [Haskell client](https://github.com/kubernetes-client/haskell) can use the same
[kubeconfig file](/docs/concepts/configuration/organize-cluster-access-kubeconfig/)
as the kubectl CLI does to locate and authenticate to the API server. See this
[example](https://github.com/kubernetes-client/haskell/blob/master/kubernetes-client/example/App.hs):
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
## Feedback
Was this page helpful?
Yes
No
Thanks for the feedback. If you have a specific, answerable question about how to use Kubernetes, ask it on
[Stack Overflow](https://stackoverflow.com/questions/tagged/kubernetes).
Open an issue in the [GitHub Repository](https://www.github.com/kubernetes/website/) if you want to
[report a problem](<https://github.com/kubernetes/website/issues/new?title=Issue with k8s.io>)
or
[suggest an improvement](<https://github.com/kubernetes/website/issues/new?title=Improvement for k8s.io>).
Last modified May 13, 2025 at 11:27 AM PST: [Fix incorrect usage of listNamespacedPod in JavaScript client example (7edfd2dfbc)](https://github.com/kubernetes/website/commit/7edfd2dfbceee3efa1750f0240c8f04cb61708d8)
## Related Pages

- [deploy intro](docs-tutorials-kubernetes-basics-deploy-app-deploy-intro.md)
- [Hello Minikube](docs-tutorials-hello-minikube.md)
- [kubectl](docs-reference-kubectl-kubectl.md)
- [Creating a cluster with kubeadm](docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm.md)
- [kubectl](docs-reference-kubectl-generated-kubectl.md)