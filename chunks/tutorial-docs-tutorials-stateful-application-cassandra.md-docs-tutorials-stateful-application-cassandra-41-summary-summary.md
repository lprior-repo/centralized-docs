---
doc_id: tutorial/docs-tutorials-stateful-application-cassandra.md/docs-tutorials-stateful-application-cassandra
chunk_id: tutorial/docs-tutorials-stateful-application-cassandra.md/docs-tutorials-stateful-application-cassandra#41-summary
chunk_level: summary
chunk_type: prose
heading: Cleaning up
token_count: 117
summary: 1. Run the following commands (chained together into a single command) to delete everything in the Cassandra StatefulSet: ``` `grace=$(kubectl get pod cassandra-0...
---

1. Run the following commands (chained together into a single command) to delete everything in the Cassandra StatefulSet:
```
`grace=$(kubectl get pod cassandra-0 -o=jsonpath='{.spec.terminationGracePeriodSeconds}') \\
&amp;&amp; kubectl delete statefulset -l app=cassandra \\
&amp;&amp; echo "Sleeping ${grace} seconds" 1&gt;&amp;2 \\
&amp;&amp; sleep $grace \\
&amp;&amp; kubectl delete persistentvolumeclaim -l app=cassandra
`
```