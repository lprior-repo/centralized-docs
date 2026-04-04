---
doc_id: ref/docs-tasks-administer-cluster-migrating-from-dockershim-check-if-dockershim-removal-affects-you.md/docs-tasks-administer-cluster-migrating-from-dockershim-check-if-dockershim-removal-affects-you
chunk_id: ref/docs-tasks-administer-cluster-migrating-from-dockershim-check-if-dockershim-removal-affects-you.md/docs-tasks-administer-cluster-migrating-from-dockershim-check-if-dockershim-removal-affects-you#12-summary
chunk_level: summary
chunk_type: prose
heading: Finding if your app has a dependencies on Docker
token_count: 91
summary: * The metrics format on the Docker node is `k8s\_&lt;container-name&gt;\_&lt;pod-name&gt;\_&lt;namespace&gt;\_&lt;pod-uid&gt;\_&lt;restart-count&gt;` but the format on other runtime is different. For...
---

* The metrics format on the Docker node is `k8s\_&lt;container-name&gt;\_&lt;pod-name&gt;\_&lt;namespace&gt;\_&lt;pod-uid&gt;\_&lt;restart-count&gt;`
but the format on other runtime is different. For example, on containerd node it is `&lt;container-id&gt;`.
* Some filesystem metrics are missing, as follows: