---
doc_id: ref/docs-concepts-containers-container-lifecycle-hooks.md/docs-concepts-containers-container-lifecycle-hooks
chunk_id: ref/docs-concepts-containers-container-lifecycle-hooks.md/docs-concepts-containers-container-lifecycle-hooks#20-summary
chunk_level: summary
chunk_type: prose
heading: Container hooks
token_count: 125
summary: Warning FailedPostStartHook 4s (x2 over 5s) kubelet Exec lifecycle hook ([badcommand]) for Container \"lifecycle-demo-container\" in Pod \"lifecycle-demo\_default(30229739-9651-4e5a-9a32-a8f1688862db)\"...
---

Warning FailedPostStartHook 4s (x2 over 5s) kubelet Exec lifecycle hook ([badcommand]) for Container "lifecycle-demo-container" in Pod "lifecycle-demo\_default(30229739-9651-4e5a-9a32-a8f1688862db)" failed - error: command 'badcommand' exited with 126: , message: "OCI runtime exec failed: exec failed: container\_linux.go:380: starting container process caused: exec: \\"badcommand\\": executable file not found in $PATH: unknown\\r\\n"