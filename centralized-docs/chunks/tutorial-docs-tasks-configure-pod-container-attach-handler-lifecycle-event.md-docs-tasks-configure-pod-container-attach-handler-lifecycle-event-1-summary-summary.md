---
doc_id: tutorial/docs-tasks-configure-pod-container-attach-handler-lifecycle-event.md/docs-tasks-configure-pod-container-attach-handler-lifecycle-event
chunk_id: tutorial/docs-tasks-configure-pod-container-attach-handler-lifecycle-event.md/docs-tasks-configure-pod-container-attach-handler-lifecycle-event#1-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 69
summary: # Attach Handlers to Container Lifecycle Events This page shows how to attach handlers to Container lifecycle events. Kubernetes supports the postStart and preStop events. Kubernetes sends the...
---

# Attach Handlers to Container Lifecycle Events
This page shows how to attach handlers to Container lifecycle events. Kubernetes supports
the postStart and preStop events. Kubernetes sends the postStart event immediately
after a Container is started, and it sends the preStop event immediately before the
Container is terminated. A Container may specify one handler per event.