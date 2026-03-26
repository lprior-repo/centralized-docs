---
doc_id: ref/docs-concepts-containers-container-lifecycle-hooks.md/docs-concepts-containers-container-lifecycle-hooks
chunk_id: ref/docs-concepts-containers-container-lifecycle-hooks.md/docs-concepts-containers-container-lifecycle-hooks#16-summary
chunk_level: summary
chunk_type: prose
heading: Container hooks
token_count: 123
summary: The logs for a Hook handler are not exposed in Pod events. If a handler fails for some reason, it broadcasts an event. For `PostStart`, this is the `FailedPostStartHook` event, and for `PreStop`,...
---

The logs for a Hook handler are not exposed in Pod events.
If a handler fails for some reason, it broadcasts an event.
For `PostStart`, this is the `FailedPostStartHook` event,
and for `PreStop`, this is the `FailedPreStopHook` event.
To generate a failed `FailedPostStartHook` event yourself, modify the
[lifecycle-events.yaml](https://k8s.io/examples/pods/lifecycle-events.yaml)
file to change the postStart command to "badcommand" and apply it.
Here is some example output of the resulting events you see from running