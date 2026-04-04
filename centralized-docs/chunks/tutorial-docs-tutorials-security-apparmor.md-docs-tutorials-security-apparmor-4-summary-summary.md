---
doc_id: tutorial/docs-tutorials-security-apparmor.md/docs-tutorials-security-apparmor
chunk_id: tutorial/docs-tutorials-security-apparmor.md/docs-tutorials-security-apparmor#4-summary
chunk_level: summary
chunk_type: prose
heading: Objectives
token_count: 88
summary: ## Objectives * See an example of how to load a profile on a Node * Learn how to enforce the profile on a Pod * Learn how to check that the profile is loaded * See what happens when a profile is...
---

## Objectives
* See an example of how to load a profile on a Node
* Learn how to enforce the profile on a Pod
* Learn how to check that the profile is loaded
* See what happens when a profile is violated
* See what happens when a profile cannot be loaded## Before you begin
AppArmor is an optional kernel module and Kubernetes feature, so verify it is supported on your
Nodes before proceeding: