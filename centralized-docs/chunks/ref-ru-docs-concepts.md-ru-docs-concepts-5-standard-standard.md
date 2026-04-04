---
doc_id: ref/ru-docs-concepts.md/ru-docs-concepts
chunk_id: ref/ru-docs-concepts.md/ru-docs-concepts#5-standard
chunk_level: standard
chunk_type: prose
heading: Краткий обзор
token_count: 413
summary: * [Deployment](/docs/concepts/workloads/controllers/deployment/) * [DaemonSet](/docs/concepts/workloads/controllers/daemonset/) * [StatefulSet](/docs/concepts/workloads/controllers/statefulset/) *...
---

* [Deployment](/docs/concepts/workloads/controllers/deployment/)
* [DaemonSet](/docs/concepts/workloads/controllers/daemonset/)
* [StatefulSet](/docs/concepts/workloads/controllers/statefulset/)
* [ReplicaSet](/docs/concepts/workloads/controllers/replicaset/)
* [Job](/docs/concepts/workloads/controllers/jobs-run-to-completion/)## Управляющий слой Kubernetes
Различные части управляющего слоя Kubernetes (control plane), такие как мастер Kubernetes и процессы kubelet, определяют, как Kubernetes взаимодействует с кластером. Управляющий слой поддерживает запись всех объектов Kubernetes в системе и запускает непрерывные циклы управления для обработки состояния этих объектов. В любое время циклы управления управляющего слоя будут реагировать на изменения в кластере и работать, чтобы фактическое состояние всех объектов в системе соответствовало желаемому состоянию, которое вы указали.
Например, когда вы используете API Kubernetes для создания развертывания, вы предоставляете новое желаемое состояние для системы. Управляющий слой Kubernetes записывает создание этого объекта и выполняет ваши инструкции, запуская необходимые приложения и планируя их на узлы кластера, чтобы фактическое состояние кластера соответствовало желаемому состоянию.