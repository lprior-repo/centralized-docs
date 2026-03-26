---
doc_id: ref/docs-concepts-containers-container-lifecycle-hooks.md/docs-concepts-containers-container-lifecycle-hooks
chunk_id: ref/docs-concepts-containers-container-lifecycle-hooks.md/docs-concepts-containers-container-lifecycle-hooks#12-summary
chunk_level: summary
chunk_type: prose
heading: Container hooks
token_count: 108
summary: takes 10 seconds to stop normally after receiving the signal, then the Container will be killed before it can stop normally, since `terminationGracePeriodSeconds` is less than the total time (55+10)...
---

takes 10 seconds to stop normally after receiving the signal, then the Container will be killed
before it can stop normally, since `terminationGracePeriodSeconds` is less than the total time
(55+10) it takes for these two things to happen.
If either a `PostStart` or `PreStop` hook fails,
it kills the Container.
Users should make their hook handlers as lightweight as possible.
There are cases, however, when long running commands make sense,
such as when saving state prior to stopping a Container.