---
doc_id: ref/fr-docs-concepts.md/fr-docs-concepts
chunk_id: ref/fr-docs-concepts.md/fr-docs-concepts#18-summary
chunk_level: summary
chunk_type: prose
heading: Vue d'ensemble
token_count: 123
summary: * [Job](/docs/concepts/workloads/controllers/jobs-run-to-completion/)## Kubernetes control plane Les différentes parties du control plane Kubernetes, telles que les processus Kubernetes master et...
---

* [Job](/docs/concepts/workloads/controllers/jobs-run-to-completion/)## Kubernetes control plane
Les différentes parties du control plane Kubernetes, telles que les processus Kubernetes master et kubelet, déterminent la manière dont Kubernetes communique avec votre cluster.
Le control plane conserve un enregistrement de tous les objets Kubernetes du système et exécute des boucles de contrôle continues pour gérer l'état de ces objets.
À tout moment, les boucles de contrôle du control plane répondent aux modifications du cluster et permettent de faire en sorte que l'