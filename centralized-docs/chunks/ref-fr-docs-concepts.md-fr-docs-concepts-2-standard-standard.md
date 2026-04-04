---
doc_id: ref/fr-docs-concepts.md/fr-docs-concepts
chunk_id: ref/fr-docs-concepts.md/fr-docs-concepts#2-standard
chunk_level: standard
chunk_type: prose
heading: Vue d'ensemble
token_count: 286
summary: ## Vue d'ensemble Pour utiliser Kubernetes, vous utilisez *les objets de l'API Kubernetes* pour décrire *l'état souhaité* de votre cluster: quelles applications ou autres processus que vous souhaitez...
---

## Vue d'ensemble
Pour utiliser Kubernetes, vous utilisez *les objets de l'API Kubernetes* pour décrire *l'état souhaité* de votre cluster: quelles applications ou autres processus que vous souhaitez exécuter, quelles images de conteneur elles utilisent, le nombre de réplicas, les ressources réseau et disque que vous mettez à disposition, et plus encore.
Vous définissez l'état souhaité en créant des objets à l'aide de l'API Kubernetes, généralement via l'interface en ligne de commande, `kubectl`.
Vous pouvez également utiliser l'API Kubernetes directement pour interagir avec le cluster et définir ou modifier l'état souhaité.
Une fois que vous avez défini l'état souhaité, le *plan de contrôle Kubernetes* (control plane en anglais) permet de faire en sorte que l'état actuel du cluster corresponde à l'état souhaité.
Pour ce faire, Kubernetes effectue automatiquement diverses tâches, telles que le démarrage ou le redémarrage de conteneurs, la mise à jour du nombre de réplicas d'une application donnée, etc.
Le control plane Kubernetes comprend un ensemble de processus en cours d'exécution sur votre cluster: