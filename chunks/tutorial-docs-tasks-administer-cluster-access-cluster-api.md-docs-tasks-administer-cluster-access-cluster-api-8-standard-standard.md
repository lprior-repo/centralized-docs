---
doc_id: tutorial/docs-tasks-administer-cluster-access-cluster-api.md/docs-tasks-administer-cluster-access-cluster-api
chunk_id: tutorial/docs-tasks-administer-cluster-access-cluster-api.md/docs-tasks-administer-cluster-access-cluster-api#8-standard
chunk_level: standard
chunk_type: prose
heading: Before you begin
token_count: 163
summary: #### Java client To install the [Java Client](https://github.com/kubernetes-client/java), run: ``` `# Clone java library git clone --recursive https://github.com/kubernetes-client/java # Installing...
---

#### Java client
To install the [Java Client](https://github.com/kubernetes-client/java), run:
```
`# Clone java library
git clone --recursive https://github.com/kubernetes-client/java
# Installing project artifacts, POM etc:
cd java
mvn install
`
```
See [https://github.com/kubernetes-client/java/releases](https://github.com/kubernetes-client/java/releases)
to see which versions are supported.
The Java client can use the same [kubeconfig file](/docs/concepts/configuration/organize-cluster-access-kubeconfig/)
as the kubectl CLI does to locate and authenticate to the API server. See this
[example](https://github.com/kubernetes-client/java/blob/master/examples/examples-release-15/src/main/java/io/kubernetes/client/examples/KubeConfigFileClientExample.java):