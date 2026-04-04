---
doc_id: tutorial/docs-tasks-inject-data-application-define-interdependent-environment-variables.md/docs-tasks-inject-data-application-define-interdependent-environment-variables
chunk_id: tutorial/docs-tasks-inject-data-application-define-interdependent-environment-variables.md/docs-tasks-inject-data-application-define-interdependent-environment-variables#13-summary
chunk_level: summary
chunk_type: prose
heading: Before you begin
token_count: 128
summary: `UNCHANGED\_REFERENCE` fails to resolve `$(PROTOCOL)` in the example above. When the environment variable is undefined or only includes some variables, the undefined environment variable is treated...
---

`UNCHANGED\_REFERENCE`
fails to resolve `$(PROTOCOL)` in the example above.
When the environment variable is undefined or only includes some variables, the undefined environment variable is treated as a normal string, such as `UNCHANGED\_REFERENCE`. Note that incorrectly parsed environment variables, in general, will not block the container from starting.
The `$(VAR\_NAME)` syntax can be escaped with a double `$`, ie: `$$(VAR\_NAME)`.
Escaped references are never expanded, regardless of whether the referenced variable
is defined or not. This can be seen from the `ESCAPED\_REFERENCE` case above.