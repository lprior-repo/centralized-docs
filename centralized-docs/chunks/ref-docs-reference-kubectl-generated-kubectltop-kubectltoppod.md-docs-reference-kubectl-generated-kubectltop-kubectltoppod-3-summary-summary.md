---
doc_id: ref/docs-reference-kubectl-generated-kubectltop-kubectltoppod.md/docs-reference-kubectl-generated-kubectltop-kubectltoppod
chunk_id: ref/docs-reference-kubectl-generated-kubectltop-kubectltoppod.md/docs-reference-kubectl-generated-kubectltop-kubectltoppod#3-summary
chunk_level: summary
chunk_type: prose
heading: Synopsis
token_count: 79
summary: # kubectl top pod Display resource (CPU/memory) usage of pods ## Synopsis Display resource (CPU/memory) usage of pods. The 'top pod' command allows you to see the resource consumption of pods. Due to...
---

# kubectl top pod
Display resource (CPU/memory) usage of pods
## Synopsis
Display resource (CPU/memory) usage of pods.
The 'top pod' command allows you to see the resource consumption of pods.
Due to the metrics pipeline delay, they may be unavailable for a few minutes since pod creation.
```
`kubectl top pod [NAME | -l label]
`
```