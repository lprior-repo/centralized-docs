---
doc_id: ref/docs-concepts-containers-container-lifecycle-hooks.md/docs-concepts-containers-container-lifecycle-hooks
chunk_id: ref/docs-concepts-containers-container-lifecycle-hooks.md/docs-concepts-containers-container-lifecycle-hooks#4-standard
chunk_level: standard
chunk_type: prose
heading: Container hooks
token_count: 512
summary: Containers can access a hook by implementing and registering a handler for that hook. There are three types of hook handlers that can be implemented for Containers: * Exec - Executes a specific...
---

Containers can access a hook by implementing and registering a handler for that hook.
There are three types of hook handlers that can be implemented for Containers:
* Exec - Executes a specific command, such as `pre-stop.sh`, inside the cgroups and namespaces of the Container.
Resources consumed by the command are counted against the Container.
* HTTP - Executes an HTTP request against a specific endpoint on the Container.
* Sleep - Pauses the container for a specified duration.### Hook handler execution
When a Container lifecycle management hook is called,
the Kubernetes management system executes the handler according to the hook action,
`httpGet`, `tcpSocket` ([deprecated](/docs/reference/generated/kubernetes-api/v1.35/#lifecyclehandler-v1-core))
and `sleep` are executed by the kubelet process, and `exec` is executed in the container.
The `PostStart` hook handler call is initiated when a container is created,
meaning the container ENTRYPOINT and the `PostStart` hook are triggered simultaneously.
(This means it generally doesn't make sense to use an HTTP hook for `PostStart`, since
there is no guarantee that the container's process will have fully started up when the
hook runs.)
If the `PostStart` hook takes too long to execute or if it hangs,
it can prevent the container from transitioning to a `running` state.
`PreStop` hooks are not executed asynchronously from the signal to stop the Container; the hook must
complete its execution before the TERM signal can be sent. If a `PreStop` hook hangs during
execution, the Pod's phase will be `Terminating` and remain there until the Pod is killed after its
`terminationGracePeriodSeconds` expires. This grace period applies to the total time it takes for
both the `PreStop` hook to execute and for the Container to stop normally. If, for example,
`terminationGracePeriodSeconds` is 60, and the hook takes 55 seconds to complete, and the Container
takes 10 seconds to stop normally after receiving the signal, then the Container will be killed
before it can stop normally, since `terminationGracePeriodSeconds` is less than the total time
(55+10) it takes for these two things to happen.
If either a `PostStart` or `PreStop` hook fails,
it kills the Container.
Users should make their hook handlers as lightweight as possible.
There are cases, however, when long running commands make sense,
such as when saving state prior to stopping a Container.