---
doc_id: tutorial/docs-tasks-inject-data-application-define-interdependent-environment-variables.md/docs-tasks-inject-data-application-define-interdependent-environment-variables
chunk_id: tutorial/docs-tasks-inject-data-application-define-interdependent-environment-variables.md/docs-tasks-inject-data-application-define-interdependent-environment-variables#4-standard
chunk_level: standard
chunk_type: prose
heading: What's next
token_count: 371
summary: 3. Check the logs for the container running in your Pod: ``` `kubectl logs pod/dependent-envars-demo ` ``` ``` ` UNCHANGED\_REFERENCE=$(PROTOCOL)://172.17.0.1:80...
---

3. Check the logs for the container running in your Pod:
```
`kubectl logs pod/dependent-envars-demo
`
```
```
`
UNCHANGED\_REFERENCE=$(PROTOCOL)://172.17.0.1:80
SERVICE\_ADDRESS=https://172.17.0.1:80
ESCAPED\_REFERENCE=$(PROTOCOL)://172.17.0.1:80
`
```
As shown above, you have defined the correct dependency reference of `SERVICE\_ADDRESS`, bad dependency reference of `UNCHANGED\_REFERENCE` and skip dependent references of `ESCAPED\_REFERENCE`.
When an environment variable is already defined when being referenced,
the reference can be correctly resolved, such as in the `SERVICE\_ADDRESS` case.
Note that order matters in the `env` list. An environment variable is not considered
"defined" if it is specified further down the list. That is why `UNCHANGED\_REFERENCE`
fails to resolve `$(PROTOCOL)` in the example above.
When the environment variable is undefined or only includes some variables, the undefined environment variable is treated as a normal string, such as `UNCHANGED\_REFERENCE`. Note that incorrectly parsed environment variables, in general, will not block the container from starting.
The `$(VAR\_NAME)` syntax can be escaped with a double `$`, ie: `$$(VAR\_NAME)`.
Escaped references are never expanded, regardless of whether the referenced variable
is defined or not. This can be seen from the `ESCAPED\_REFERENCE` case above.
## What's next
* Learn more about [environment variables](/docs/tasks/inject-data-application/environment-variable-expose-pod-information/).
* See [EnvVarSource](/docs/reference/generated/kubernetes-api/v1.35/#envvarsource-v1-core).