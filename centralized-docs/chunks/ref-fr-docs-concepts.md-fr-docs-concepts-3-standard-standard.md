---
doc_id: ref/fr-docs-concepts.md/fr-docs-concepts
chunk_id: ref/fr-docs-concepts.md/fr-docs-concepts#3-standard
chunk_level: standard
chunk_type: prose
heading: Vue d'ensemble
token_count: 480
summary: * Le **maître Kubernetes** (Kubernetes master en anglais) qui est un ensemble de trois processus qui s'exécutent sur un seul nœud de votre cluster, désigné comme nœud maître (master node en anglais)....
---

* Le **maître Kubernetes** (Kubernetes master en anglais) qui est un ensemble de trois processus qui s'exécutent sur un seul nœud de votre cluster, désigné comme nœud maître (master node en anglais). Ces processus sont: [kube-apiserver](/docs/admin/kube-apiserver/), [kube-controller-manager](/docs/admin/kube-controller-manager/) et [kube-scheduler](/docs/admin/kube-scheduler/).
* Chaque nœud non maître de votre cluster exécute deux processus:
* **[kubelet](/docs/admin/kubelet/)**, qui communique avec le Kubernetes master.
* **[kube-proxy](/docs/admin/kube-proxy/)**, un proxy réseau reflétant les services réseau Kubernetes sur chaque nœud.## Objets Kubernetes
Kubernetes contient un certain nombre d'abstractions représentant l'état de votre système: applications et processus conteneurisés déployés, leurs ressources réseau et disque associées, ainsi que d'autres informations sur les activités de votre cluster.
Ces abstractions sont représentées par des objets de l'API Kubernetes; consultez [Vue d'ensemble des objets Kubernetes](/docs/concepts/abstractions/overview/) pour plus d'informations.
Les objets de base de Kubernetes incluent:
* [Pod](/fr/docs/concepts/workloads/pods/pod-overview/)
* [Service](/fr/docs/concepts/services-networking/service/)
* [Volume](/fr/docs/concepts/storage/volumes/)
* [Namespace](/fr/docs/concepts/overview/working-with-objects/namespaces/)
En outre, Kubernetes contient un certain nombre d'abstractions de niveau supérieur appelées Contrôleurs.
Les contrôleurs s'appuient sur les objets de base et fournissent des fonctionnalités supplémentaires.
Voici quelques exemples:
* [ReplicaSet](/fr/docs/concepts/workloads/controllers/replicaset/)
* [Deployment](/fr/docs/concepts/workloads/controllers/deployment/)
* [StatefulSet](/fr/docs/concepts/workloads/controllers/statefulset/)
* [DaemonSet](/docs/concepts/workloads/controllers/daemonset/)