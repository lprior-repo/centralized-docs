---
doc_id: tutorial/docs-tutorials-stateful-application-cassandra.md/docs-tutorials-stateful-application-cassandra
chunk_id: tutorial/docs-tutorials-stateful-application-cassandra.md/docs-tutorials-stateful-application-cassandra#12-standard
chunk_level: standard
chunk_type: table
heading: Cassandra container environment variables
token_count: 430
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