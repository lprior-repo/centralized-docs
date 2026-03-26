---
doc_id: tutorial/docs-tutorials-stateful-application-cassandra.md/docs-tutorials-stateful-application-cassandra
chunk_id: tutorial/docs-tutorials-stateful-application-cassandra.md/docs-tutorials-stateful-application-cassandra#4-detailed
chunk_level: detailed
chunk_type: table
heading: Related Pages
token_count: 819
summary: ## Cleaning up Deleting or scaling a StatefulSet down does not delete the volumes associated with the StatefulSet. This setting is for your safety because your data is more valuable than...
---

## Cleaning up
Deleting or scaling a StatefulSet down does not delete the volumes associated with the StatefulSet.
This setting is for your safety because your data is more valuable than automatically purging all related StatefulSet resources.
#### Warning:
Depending on the storage class and reclaim policy, deleting the *PersistentVolumeClaims* may cause the associated volumes
to also be deleted. Never assume you'll be able to access data if its volume claims are deleted.
1. Run the following commands (chained together into a single command) to delete everything in the Cassandra StatefulSet:
```
`grace=$(kubectl get pod cassandra-0 -o=jsonpath='{.spec.terminationGracePeriodSeconds}') \\
&amp;&amp; kubectl delete statefulset -l app=cassandra \\
&amp;&amp; echo "Sleeping ${grace} seconds" 1&gt;&amp;2 \\
&amp;&amp; sleep $grace \\
&amp;&amp; kubectl delete persistentvolumeclaim -l app=cassandra
`
```
2. Run the following command to delete the Service you set up for Cassandra:
```
`kubectl delete service -l app=cassandra
`
```
## Cassandra container environment variables
The Pods in this tutorial use the [`gcr.io/google-samples/cassandra:v13`](https://github.com/kubernetes/examples/blob/master/cassandra/image/Dockerfile)
image from Google's [container registry](https://cloud.google.com/container-registry/docs/).
The Docker image above is based on [debian-base](https://github.com/kubernetes/release/tree/master/images/build/debian-base)
and includes OpenJDK 8.
This image includes a standard Cassandra installation from the Apache Debian repo.
By using environment variables you can change values that are inserted into `cassandra.yaml`.
|Environment variable|Default value|
|`CASSANDRA\_CLUSTER\_NAME`|`'Test Cluster'`|
|`CASSANDRA\_NUM\_TOKENS`|`32`|
|`CASSANDRA\_RPC\_ADDRESS`|`0.0.0.0`|
## What's next
* Learn how to [Scale a StatefulSet](/docs/tasks/run-application/scale-stateful-set/).
* Learn more about the [*KubernetesSeedProvider*](https://github.com/kubernetes/examples/blob/master/cassandra/java/src/main/java/io/k8s/cassandra/KubernetesSeedProvider.java)
* See more custom [Seed Provider Configurations](https://git.k8s.io/examples/cassandra/java/README.md)
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
Last modified August 24, 2023 at 6:38 PM PST: [Use code\_sample shortcode instead of code shortcode (e8b136c3b3)](https://github.com/kubernetes/website/commit/e8b136c3b3e6fb96580f889ed3260a0918e99896)
## Related Pages

- [Process ID Limits And Reservations](docs-concepts-policy-pid-limiting.md)
- [Debugging DNS Resolution](docs-tasks-administer-cluster-dns-debugging-resolution.md)
- [Containers](docs-concepts-containers.md)
- [Tools for Monitoring Resources](docs-tasks-debug-debug-cluster-resource-usage-monitoring.md)
- [Use a Service to Access an Application in a Cluster](docs-tasks-access-application-cluster-service-access-application-cluster.md)