---
doc_id: tutorial/docs-tasks-administer-cluster-securing-a-cluster.md/docs-tasks-administer-cluster-securing-a-cluster
chunk_id: tutorial/docs-tasks-administer-cluster-securing-a-cluster.md/docs-tasks-administer-cluster-securing-a-cluster#49-summary
chunk_level: summary
chunk_type: prose
heading: Protecting cluster components from compromise
token_count: 111
summary: ### Rotate infrastructure credentials frequently The shorter the lifetime of a secret or credential the harder it is for an attacker to make use of that credential. Set short lifetimes on...
---

### Rotate infrastructure credentials frequently
The shorter the lifetime of a secret or credential the harder it is for an attacker to make
use of that credential. Set short lifetimes on certificates and automate their rotation. Use
an authentication provider that can control how long issued tokens are available and use short
lifetimes where possible. If you use service-account tokens in external integrations, plan to
rotate those tokens frequently. For example, once the bootstrap phase is complete, a bootstrap
token used for setting up nodes should be revoked or its authorization removed.