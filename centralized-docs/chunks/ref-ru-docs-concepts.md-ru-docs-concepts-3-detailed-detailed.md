---
doc_id: ref/ru-docs-concepts.md/ru-docs-concepts
chunk_id: ref/ru-docs-concepts.md/ru-docs-concepts#3-detailed
chunk_level: detailed
chunk_type: prose
heading: Краткий обзор
token_count: 960
summary: * **Мастер Kubernetes** — это коллекция из трех процессов, которые выполняются на одном узле в вашем кластере, который обозначен как главный узел. Это процессы:...
---

* **Мастер Kubernetes** — это коллекция из трех процессов, которые выполняются на одном узле в вашем кластере, который обозначен как главный узел. Это процессы: [kube-apiserver](/docs/admin/kube-apiserver/), [kube-controller-manager](/docs/admin/kube-controller-manager/) и [kube-scheduler](/docs/admin/kube-scheduler/).
* Каждый отдельный неосновной узел в вашем кластере выполняет два процесса:
* **[kubelet](/docs/admin/kubelet/)**, который взаимодействует с мастером Kubernetes.
* **[kube-proxy](/docs/admin/kube-proxy/)**, сетевой прокси, который обрабатывает сетевые сервисы Kubernetes на каждом узле.## Объекты Kubernetes
Kubernetes содержит ряд абстракций, которые представляют состояние вашей системы: развернутые контейнеризованные приложения и рабочие нагрузки, связанные с ними сетевые и дисковые ресурсы и другую информацию о том, что делает ваш кластер. Эти абстракции представлены объектами в API Kubernetes. См. [Понимание объектов Kubernetes](/ru/docs/concepts/overview/working-with-objects/kubernetes-objects/) для получения более подробной информации.
Основные объекты Kubernetes включают в себя:
* [Pod](/docs/concepts/workloads/pods/pod-overview/)
* [Service](/docs/concepts/services-networking/service/)
* [Том](/docs/concepts/storage/volumes/)
* [Namespace](/ru/docs/concepts/overview/working-with-objects/namespaces/)
Kubernetes также содержит абстракции более высокого уровня, которые опираются на [Контроллеры](/ru/docs/concepts/architecture/controller/) для создания базовых объектов и предоставляют дополнительные функциональные и удобные функции. Они включают:
* [Deployment](/docs/concepts/workloads/controllers/deployment/)
* [DaemonSet](/docs/concepts/workloads/controllers/daemonset/)
* [StatefulSet](/docs/concepts/workloads/controllers/statefulset/)
* [ReplicaSet](/docs/concepts/workloads/controllers/replicaset/)
* [Job](/docs/concepts/workloads/controllers/jobs-run-to-completion/)## Управляющий слой Kubernetes
Различные части управляющего слоя Kubernetes (control plane), такие как мастер Kubernetes и процессы kubelet, определяют, как Kubernetes взаимодействует с кластером. Управляющий слой поддерживает запись всех объектов Kubernetes в системе и запускает непрерывные циклы управления для обработки состояния этих объектов. В любое время циклы управления управляющего слоя будут реагировать на изменения в кластере и работать, чтобы фактическое состояние всех объектов в системе соответствовало желаемому состоянию, которое вы указали.
Например, когда вы используете API Kubernetes для создания развертывания, вы предоставляете новое желаемое состояние для системы. Управляющий слой Kubernetes записывает создание этого объекта и выполняет ваши инструкции, запуская необходимые приложения и планируя их на узлы кластера, чтобы фактическое состояние кластера соответствовало желаемому состоянию.