---
doc_id: ref/fr-docs-concepts.md/fr-docs-concepts
chunk_id: ref/fr-docs-concepts.md/fr-docs-concepts#4-standard
chunk_level: standard
chunk_type: prose
heading: Vue d'ensemble
token_count: 443
summary: * [ReplicaSet](/fr/docs/concepts/workloads/controllers/replicaset/) * [Deployment](/fr/docs/concepts/workloads/controllers/deployment/) *...
---

* [ReplicaSet](/fr/docs/concepts/workloads/controllers/replicaset/)
* [Deployment](/fr/docs/concepts/workloads/controllers/deployment/)
* [StatefulSet](/fr/docs/concepts/workloads/controllers/statefulset/)
* [DaemonSet](/docs/concepts/workloads/controllers/daemonset/)
* [Job](/docs/concepts/workloads/controllers/jobs-run-to-completion/)## Kubernetes control plane
Les différentes parties du control plane Kubernetes, telles que les processus Kubernetes master et kubelet, déterminent la manière dont Kubernetes communique avec votre cluster.
Le control plane conserve un enregistrement de tous les objets Kubernetes du système et exécute des boucles de contrôle continues pour gérer l'état de ces objets.
À tout moment, les boucles de contrôle du control plane répondent aux modifications du cluster et permettent de faire en sorte que l'état réel de tous les objets du système corresponde à l'état souhaité que vous avez fourni.
Par exemple, lorsque vous utilisez l'API Kubernetes pour créer un objet Deployment, vous fournissez un nouvel état souhaité pour le système.
Le control plane Kubernetes enregistre la création de cet objet et exécute vos instructions en lançant les applications requises et en les planifiant vers des nœuds de cluster, afin que l'état actuel du cluster corresponde à l'état souhaité.
### Kubernetes master
Le Kubernetes master est responsable du maintien de l'état souhaité pour votre cluster.
Lorsque vous interagissez avec Kubernetes, par exemple en utilisant l'interface en ligne de commande `kubectl`, vous communiquez avec le master Kubernetes de votre cluster.
> Le "master" fait référence à un ensemble de processus gérant l'état du cluster.> En règle générale, tous les processus sont exécutés sur un seul nœud du cluster.> Ce nœud est également appelé master.Le master peut également être répliqué pour la disponibilité et la redondance.