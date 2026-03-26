---
doc_id: ref/docs-concepts-containers-container-lifecycle-hooks.md/docs-concepts-containers-container-lifecycle-hooks
chunk_id: ref/docs-concepts-containers-container-lifecycle-hooks.md/docs-concepts-containers-container-lifecycle-hooks#10-summary
chunk_level: summary
chunk_type: prose
heading: Container hooks
token_count: 128
summary: meaning the container ENTRYPOINT and the `PostStart` hook are triggered simultaneously. (This means it generally doesn't make sense to use an HTTP hook for `PostStart`, since there is no guarantee...
---

meaning the container ENTRYPOINT and the `PostStart` hook are triggered simultaneously.
(This means it generally doesn't make sense to use an HTTP hook for `PostStart`, since
there is no guarantee that the container's process will have fully started up when the
hook runs.)
If the `PostStart` hook takes too long to execute or if it hangs,
it can prevent the container from transitioning to a `running` state.
`PreStop` hooks are not executed asynchronously from the signal to stop the Container; the hook must
complete its execution before the TERM signal can be sent. If a `PreStop` hook hangs during