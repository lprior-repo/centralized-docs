---
doc_id: ref/ru-docs-concepts.md/ru-docs-concepts
chunk_id: ref/ru-docs-concepts.md/ru-docs-concepts#3-standard
chunk_level: standard
chunk_type: prose
heading: Краткий обзор
token_count: 440
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