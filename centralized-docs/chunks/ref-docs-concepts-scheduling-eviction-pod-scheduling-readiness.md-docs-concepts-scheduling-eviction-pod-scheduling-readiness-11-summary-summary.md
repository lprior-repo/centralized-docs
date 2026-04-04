---
doc_id: ref/docs-concepts-scheduling-eviction-pod-scheduling-readiness.md/docs-concepts-scheduling-eviction-pod-scheduling-readiness
chunk_id: ref/docs-concepts-scheduling-eviction-pod-scheduling-readiness.md/docs-concepts-scheduling-eviction-pod-scheduling-readiness#11-summary
chunk_level: summary
chunk_type: prose
heading: Usage example
token_count: 83
summary: ## Usage example To mark a Pod not-ready for scheduling, you can create it with one or more scheduling gates like this: [`pods/pod-with-scheduling-gates.yaml`...
---

## Usage example
To mark a Pod not-ready for scheduling, you can create it with one or more scheduling gates like this:
[`pods/pod-with-scheduling-gates.yaml`
](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/pods/pod-with-scheduling-gates.yaml)![](/images/copycode.svg "Copy pods/pod-with-scheduling-gates.yaml to clipboard")