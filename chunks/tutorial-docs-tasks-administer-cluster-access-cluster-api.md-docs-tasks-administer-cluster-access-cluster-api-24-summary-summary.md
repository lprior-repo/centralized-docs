---
doc_id: tutorial/docs-tasks-administer-cluster-access-cluster-api.md/docs-tasks-administer-cluster-access-cluster-api
chunk_id: tutorial/docs-tasks-administer-cluster-access-cluster-api.md/docs-tasks-administer-cluster-access-cluster-api#24-summary
chunk_level: summary
chunk_type: prose
heading: Before you begin
token_count: 113
summary: #### Note: `client-go` defines its own API objects, so if needed, import API definitions from client-go rather than from the main repository. For example, `import \"k8s.io/client-go/kubernetes\"` is...
---

#### Note:
`client-go` defines its own API objects, so if needed, import API definitions from client-go rather than
from the main repository. For example, `import "k8s.io/client-go/kubernetes"` is correct.
The Go client can use the same [kubeconfig file](/docs/concepts/configuration/organize-cluster-access-kubeconfig/)
as the kubectl CLI does to locate and authenticate to the API server. See this [example](https://git.k8s.io/client-go/examples/out-of-cluster-client-configuration/main.go):