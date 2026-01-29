---
url: https://docs.aws.amazon.com/step-functions/latest/dg/concepts-cd-aliasing-versioning.html
title: concepts cd aliasing versioning.html
word_count: 342
filtered: true
elements_removed: 0
density_score: 0.80
---

Manage continuous deployments with versions and aliases in Step Functions - AWS Step Functions
Manage continuous deployments with versions and aliases in Step Functions - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/dg/step-functions-dg.pdf#concepts-cd-aliasing-versioning)
#
Manage continuous deployments with versions and aliases in Step Functions
You
can use
Step Functions
to manage continuous deployments of your workflows through state machine
*versions* and *aliases*. A
*version* is a numbered, immutable snapshot of a state machine
that
you can run. An
*alias* is a pointer for up to two versions of a state
machine.
You can maintain multiple versions of your state machines and manage their deployment in your production workflow. With aliases, you can route traffic between
different workflow versions and gradually deploy those workflows to the production
environment.
Additionally, you can start state machine
executions
using a version or an alias. If you don't use a version or alias when you start a state
machine execution, Step Functions uses the latest revision of the state machine definition.
###### State machine revision
A state machine can have one or more revisions. When you update a state machine using the [UpdateStateMachine](https://docs.aws.amazon.com/step-functions/latest/apireference/API_UpdateStateMachine.html) API action, it creates a new state
machine revision. A *revision* is an immutable, read-only snapshot of
a state machine’s definition and configuration. You can't start a state machine
execution from a revision, and revisions don't have an ARN. Revisions have a `revisionId`, which is a universally unique identifier (UUID).
###### Contents
* [Versions](./concepts-state-machine-version.html)
* [Aliases](./concepts-state-machine-alias.html)
* [Versions and alias authorization](./auth-version-alias.html)
* [Associating executions with a version or alias](./execution-alias-version-associate.html)
* [Deployment example](./example-alias-version-deployment.html)
* [Gradual deployment of versions](./version-rolling-deployment.html)
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Testing with mocked service integrations
Versions
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.