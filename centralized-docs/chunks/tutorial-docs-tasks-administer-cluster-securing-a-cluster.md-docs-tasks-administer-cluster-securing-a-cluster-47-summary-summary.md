---
doc_id: tutorial/docs-tasks-administer-cluster-securing-a-cluster.md/docs-tasks-administer-cluster-securing-a-cluster
chunk_id: tutorial/docs-tasks-administer-cluster-securing-a-cluster.md/docs-tasks-administer-cluster-securing-a-cluster#47-summary
chunk_level: summary
chunk_type: prose
heading: Protecting cluster components from compromise
token_count: 72
summary: #### Caution: Allowing other components within the cluster to access the master etcd instance with read or write access to the full keyspace is equivalent to granting cluster-admin access. Using...
---

#### Caution:
Allowing other components within the cluster to access the master etcd instance with
read or write access to the full keyspace is equivalent to granting cluster-admin access. Using
separate etcd instances for non-master components or using etcd ACLs to restrict read and write
access to a subset of the keyspace is strongly recommended.