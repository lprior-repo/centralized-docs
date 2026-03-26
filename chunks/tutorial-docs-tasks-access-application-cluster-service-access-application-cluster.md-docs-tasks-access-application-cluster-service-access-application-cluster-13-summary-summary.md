---
doc_id: tutorial/docs-tasks-access-application-cluster-service-access-application-cluster.md/docs-tasks-access-application-cluster-service-access-application-cluster
chunk_id: tutorial/docs-tasks-access-application-cluster-service-access-application-cluster.md/docs-tasks-access-application-cluster-service-access-application-cluster#13-summary
chunk_level: summary
chunk_type: prose
heading: Before you begin
token_count: 94
summary: 7. Get the public IP address of one of your nodes that is running a Hello World pod. How you get this address depends on how you set up your cluster. For example, if you are using Minikube, you can...
---

7. Get the public IP address of one of your nodes that is running
a Hello World pod. How you get this address depends on how you set
up your cluster. For example, if you are using Minikube, you can
see the node address by running `kubectl cluster-info`. If you are
using Google Compute Engine instances, you can use the
`gcloud compute instances list` command to see the public addresses of your
nodes.