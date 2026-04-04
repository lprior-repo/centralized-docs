---
id: ref/fr-docs-concepts.md/fr-docs-concepts
title: Concepts
category: ref
tags: ["concepts", "contents", "d'ensemble", "ref", "table"]
---

## Table of Contents

* [Concepts](#concepts)
  * [Vue d’ensemble](#vue-densemble)
    * [Kubernetes master](#kubernetes-master)
    * [Noeuds Kubernetes](#noeuds-kubernetes)
      * [Metadonnées des objets Kubernetes](#metadonnées-des-objets-kubernetes)
        * \[[Vue d’ensemble](/fr/docs/concepts/overview/)\](#vue-densemblefrdocsconceptsoverview)
        * \[[Workloads](/fr/docs/concepts/workloads/)\](#workloadsfrdocsconceptsworkloads)
        * \[[Services, Equilibreur de charge, et Réseau](/fr/docs/concepts/services-networking/)\](#services-equilibreur-de-charge-et-réseaufrdocsconceptsservices-networking)
  * [Feedback](#feedback)

---

# Concepts



 > 
 > **Context**: Concepts Kubernetes La section Concepts vous aide à mieux comprendre les composants du système Kubernetes et les abstractions que Kubernetes utilise p



Concepts Kubernetes
La section Concepts vous aide à mieux comprendre les composants du système Kubernetes et les abstractions que Kubernetes utilise pour représenter votre cluster.
Elle vous aide également à mieux comprendre le fonctionnement de Kubernetes en général.

## Vue d’ensemble

Pour utiliser Kubernetes, vous utilisez *les objets de l’API Kubernetes* pour décrire *l’état souhaité* de votre cluster: quelles applications ou autres processus que vous souhaitez exécuter, quelles images de conteneur elles utilisent, le nombre de réplicas, les ressources réseau et disque que vous mettez à disposition, et plus encore.
Vous définissez l’état souhaité en créant des objets à l’aide de l’API Kubernetes, généralement via l’interface en ligne de commande, `kubectl`.
Vous pouvez également utiliser l’API Kubernetes directement pour interagir avec le cluster et définir ou modifier l’état souhaité.
Une fois que vous avez défini l’état souhaité, le *plan de contrôle Kubernetes* (control plane en anglais) permet de faire en sorte que l’état actuel du cluster corresponde à l’état souhaité.
Pour ce faire, Kubernetes effectue automatiquement diverses tâches, telles que le démarrage ou le redémarrage de conteneurs, la mise à jour du nombre de réplicas d’une application donnée, etc.
Le control plane Kubernetes comprend un ensemble de processus en cours d’exécution sur votre cluster:

* Le **maître Kubernetes** (Kubernetes master en anglais) qui est un ensemble de trois processus qui s’exécutent sur un seul nœud de votre cluster, désigné comme nœud maître (master node en anglais). Ces processus sont: [kube-apiserver](/docs/admin/kube-apiserver/), [kube-controller-manager](/docs/admin/kube-controller-manager/) et [kube-scheduler](/docs/admin/kube-scheduler/).
* Chaque nœud non maître de votre cluster exécute deux processus:
* **[kubelet](/docs/admin/kubelet/)**, qui communique avec le Kubernetes master.
* **[kube-proxy](/docs/admin/kube-proxy/)**, un proxy réseau reflétant les services réseau Kubernetes sur chaque nœud.## Objets Kubernetes
  Kubernetes contient un certain nombre d’abstractions représentant l’état de votre système: applications et processus conteneurisés déployés, leurs ressources réseau et disque associées, ainsi que d’autres informations sur les activités de votre cluster.
  Ces abstractions sont représentées par des objets de l’API Kubernetes; consultez [Vue d’ensemble des objets Kubernetes](/docs/concepts/abstractions/overview/) pour plus d’informations.
  Les objets de base de Kubernetes incluent:
* [Pod](/fr/docs/concepts/workloads/pods/pod-overview/)
* [Service](/fr/docs/concepts/services-networking/service/)
* [Volume](/fr/docs/concepts/storage/volumes/)
* [Namespace](/fr/docs/concepts/overview/working-with-objects/namespaces/)
  En outre, Kubernetes contient un certain nombre d’abstractions de niveau supérieur appelées Contrôleurs.
  Les contrôleurs s’appuient sur les objets de base et fournissent des fonctionnalités supplémentaires.
  Voici quelques exemples:
* [ReplicaSet](/fr/docs/concepts/workloads/controllers/replicaset/)
* [Deployment](/fr/docs/concepts/workloads/controllers/deployment/)
* [StatefulSet](/fr/docs/concepts/workloads/controllers/statefulset/)
* [DaemonSet](/docs/concepts/workloads/controllers/daemonset/)
* [Job](/docs/concepts/workloads/controllers/jobs-run-to-completion/)\## Kubernetes control plane
  Les différentes parties du control plane Kubernetes, telles que les processus Kubernetes master et kubelet, déterminent la manière dont Kubernetes communique avec votre cluster.
  Le control plane conserve un enregistrement de tous les objets Kubernetes du système et exécute des boucles de contrôle continues pour gérer l’état de ces objets.
  À tout moment, les boucles de contrôle du control plane répondent aux modifications du cluster et permettent de faire en sorte que l’état réel de tous les objets du système corresponde à l’état souhaité que vous avez fourni.
  Par exemple, lorsque vous utilisez l’API Kubernetes pour créer un objet Deployment, vous fournissez un nouvel état souhaité pour le système.
  Le control plane Kubernetes enregistre la création de cet objet et exécute vos instructions en lançant les applications requises et en les planifiant vers des nœuds de cluster, afin que l’état actuel du cluster corresponde à l’état souhaité.

### Kubernetes master

Le Kubernetes master est responsable du maintien de l’état souhaité pour votre cluster.
Lorsque vous interagissez avec Kubernetes, par exemple en utilisant l’interface en ligne de commande `kubectl`, vous communiquez avec le master Kubernetes de votre cluster.

 > 
 > Le “master” fait référence à un ensemble de processus gérant l’état du cluster.> En règle générale, tous les processus sont exécutés sur un seul nœud du cluster.> Ce nœud est également appelé master.Le master peut également être répliqué pour la disponibilité et la redondance.

### Noeuds Kubernetes

Les nœuds d’un cluster sont les machines (serveurs physiques, machines virtuelles, etc.) qui exécutent vos applications et vos workflows.
Le master node Kubernetes contrôle chaque noeud; vous interagirez rarement directement avec les nœuds.

#### Metadonnées des objets Kubernetes

* [Annotations](/fr/docs/concepts/overview/working-with-objects/annotations/)\## A suivre
  Si vous souhaitez écrire une page de concept, consultez
  [Utilisation de modèles de page](/docs/home/contribute/page-templates/)
  pour plus d’informations sur le type de page pour la documentation d’un concept.

#### [Vue d’ensemble](/fr/docs/concepts/overview/)

Kubernetes est une plateforme open source portable et extensible pour gérer les charges de travail et les services conteneurisés, qui facilite à la fois la configuration déclarative et l’automatisation. Il dispose d’un écosystème vaste et en pleine croissance. Les services, le support et les outils Kubernetes sont largement disponibles.

#### [Workloads](/fr/docs/concepts/workloads/)

Comprendre les Pods, le plus petit objet déployable sur Kubernetes, et les abstractions de haut niveaux vous permettant de les lancer.

#### [Services, Equilibreur de charge, et Réseau](/fr/docs/concepts/services-networking/)

Service Reseau Kubernetes

## Feedback

Cette page est elle utile ?
Oui
Non
Thanks for the feedback. If you have a specific, answerable question about how to use Kubernetes, ask it on
[Stack Overflow](https://stackoverflow.com/questions/tagged/kubernetes).
Open an issue in the [GitHub Repository](https://www.github.com/kubernetes/website/) if you want to
[report a problem](<https://github.com/kubernetes/website/issues/new?title=Issue with k8s.io>)
or
[suggest an improvement](<https://github.com/kubernetes/website/issues/new?title=Improvement for k8s.io>).
Dernière modification September 02, 2024 at 9:30 PM PST: [update fr docs concepts overview (db1e142f84)](https://github.com/kubernetes/website/commit/db1e142f84a4b22f6ed145fc727aa9cec3dbfd32)

## Related Pages

* [Концепции](./ref-ru-docs-concepts.md-ru-docs-concepts.md)
* [Taints and Tolerations](./ref-docs-concepts-scheduling-eviction-taint-and-toleration.md-docs-concepts-scheduling-eviction-taint-and-toleration.md)
* [Binding](./ref-docs-reference-kubernetes-api-workload-resources-binding-v1.md-docs-reference-kubernetes-api-workload-resources-binding-v1.md)
* [conventions](./ref-docs-reference-kubectl-conventions.md-docs-reference-kubectl-conventions.md)
* [HorizontalPodAutoscaler](./ref-docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2.md-docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2.md)
## See Also

- [Documentation Index](./COMPASS.md)
