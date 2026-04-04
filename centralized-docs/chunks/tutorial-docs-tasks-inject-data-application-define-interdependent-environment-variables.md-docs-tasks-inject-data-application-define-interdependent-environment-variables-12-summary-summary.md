---
doc_id: tutorial/docs-tasks-inject-data-application-define-interdependent-environment-variables.md/docs-tasks-inject-data-application-define-interdependent-environment-variables
chunk_id: tutorial/docs-tasks-inject-data-application-define-interdependent-environment-variables.md/docs-tasks-inject-data-application-define-interdependent-environment-variables#12-summary
chunk_level: summary
chunk_type: prose
heading: Before you begin
token_count: 120
summary: As shown above, you have defined the correct dependency reference of `SERVICE\_ADDRESS`, bad dependency reference of `UNCHANGED\_REFERENCE` and skip dependent references of `ESCAPED\_REFERENCE`. When...
---

As shown above, you have defined the correct dependency reference of `SERVICE\_ADDRESS`, bad dependency reference of `UNCHANGED\_REFERENCE` and skip dependent references of `ESCAPED\_REFERENCE`.
When an environment variable is already defined when being referenced,
the reference can be correctly resolved, such as in the `SERVICE\_ADDRESS` case.
Note that order matters in the `env` list. An environment variable is not considered
"defined" if it is specified further down the list. That is why `UNCHANGED\_REFERENCE`
fails to resolve `$(PROTOCOL)` in the example above.