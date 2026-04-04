---
doc_id: ref/fr-docs-concepts.md/fr-docs-concepts
chunk_id: ref/fr-docs-concepts.md/fr-docs-concepts#19-summary
chunk_level: summary
chunk_type: prose
heading: Vue d'ensemble
token_count: 128
summary: À tout moment, les boucles de contrôle du control plane répondent aux modifications du cluster et permettent de faire en sorte que l'état réel de tous les objets du système corresponde à l'état...
---

À tout moment, les boucles de contrôle du control plane répondent aux modifications du cluster et permettent de faire en sorte que l'état réel de tous les objets du système corresponde à l'état souhaité que vous avez fourni.
Par exemple, lorsque vous utilisez l'API Kubernetes pour créer un objet Deployment, vous fournissez un nouvel état souhaité pour le système.
Le control plane Kubernetes enregistre la création de cet objet et exécute vos instructions en lançant les applications requises et en les planifiant vers des nœuds de cluster, afin que l'