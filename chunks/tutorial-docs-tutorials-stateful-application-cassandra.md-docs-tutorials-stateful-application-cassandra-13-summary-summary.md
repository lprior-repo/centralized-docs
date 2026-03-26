---
doc_id: tutorial/docs-tutorials-stateful-application-cassandra.md/docs-tutorials-stateful-application-cassandra
chunk_id: tutorial/docs-tutorials-stateful-application-cassandra.md/docs-tutorials-stateful-application-cassandra#13-summary
chunk_level: summary
chunk_type: prose
heading: Objectives
token_count: 88
summary: #### Caution: [Minikube](https://minikube.sigs.k8s.io/docs/) defaults to 2048MB of memory and 2 CPU. Running Minikube with the default resource configuration results in insufficient resource errors...
---

#### Caution:
[Minikube](https://minikube.sigs.k8s.io/docs/) defaults to 2048MB of memory and 2 CPU.
Running Minikube with the default resource configuration results in insufficient resource
errors during this tutorial. To avoid these errors, start Minikube with the following settings:
```
`minikube start --memory 5120 --cpus=4
`
```