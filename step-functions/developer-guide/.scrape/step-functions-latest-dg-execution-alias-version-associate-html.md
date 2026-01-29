---
url: https://docs.aws.amazon.com/step-functions/latest/dg/execution-alias-version-associate.html
title: How Step Functions associates executions with a version or alias
word_count: 909
filtered: true
elements_removed: 0
density_score: 0.85
---

How Step Functions associates executions with a version or alias - AWS Step Functions
How Step Functions associates executions with a version or alias - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/dg/step-functions-dg.pdf#execution-alias-version-associate)
[Viewing executions started with a version or an alias](#view-version-alias-executions)
# How Step Functions associates executions with a version or alias
Step Functions associates an execution with a version or alias based on the Amazon Resource Name (ARN) that
you use to invoke the [StartExecution](https://docs.aws.amazon.com/step-functions/latest/apireference/API_StartExecution.html) API
action. Step Functions performs this action at the execution start time.
You can start a state machine execution using a qualified or an unqualified ARN.
* **Qualified ARN** – Refers to a state machine ARN suffixed with a version number or an alias name.
The following qualified ARN example refers to version `3` of a state
machine named `myStateMachine`.
```
``arn:aws:`states:``region``:`account-id`:stateMachine:`myStateMachine`:3`
```
The following qualified ARN example refers to an alias named `PROD` of a state machine named `myStateMachine`.
```
``arn:aws:`states:``region``:`account-id`:stateMachine:`myStateMachine`:`PROD``
```
* **Unqualified ARN** – Refers to a state machine ARN without a version number or an alias name suffix.
```
``arn:aws:`states:``region``:`account-id`:stateMachine:`myStateMachine``
```
For example, if your qualified ARN refers to version `3`, Step Functions associates the
execution with this version. It doesn't associate the execution with any aliases that point
to the version `3`.
If your qualified ARN refers to an alias, Step Functions associates the execution with that alias and the version to which the alias points. An execution can only be associated with one alias.
###### Note
If you start an execution with an unqualified ARN, Step Functions doesn't associate that execution with a version even if the version uses the same state machine [revision](./concepts-cd-aliasing-versioning.html#statemachinerev). For example, if version 3 uses the latest revision, but you start an execution with an unqualified ARN, Step Functions doesn't associate that execution with the version 3.
## Viewing executions started with a version or an alias
Step Functions provides the following ways in which you can view the executions started with a version or an alias:
### Using API actions
You can view all the executions associated with a version or an alias by invoking the [DescribeExecution](https://docs.aws.amazon.com/step-functions/latest/apireference/API_DescribeExecution.html) and [ListExecutions](https://docs.aws.amazon.com/step-functions/latest/apireference/API_ListExecutions.html) API actions. These API actions return the ARN of the version or alias that was used to start the execution. These actions also return other details including status and ARN of the execution.
You can also provide a state machine alias ARN or version ARN to list the executions associated with a specific alias or version.
The following example response of the [ListExecutions](https://docs.aws.amazon.com/step-functions/latest/apireference/API_ListExecutions.html) API action shows the ARN of the alias used to start
a state machine execution named `myFirstExecution`.
The `italicized` text in the following code snippet represents resource-specific information.
```
`{
"executions": [
{
"executionArn": "arn:aws:states:``region``:`account-id`:execution:myStateMachine:`myFirstExecution`",
"stateMachineArn": "arn:aws:states:``region``:`account-id`:stateMachine:`myStateMachine`",
"stateMachineAliasArn": "arn:aws:states:``region``:`account-id`:stateMachine:myStateMachine:`PROD`",
"name": "`myFirstExecution`",
"status": "SUCCEEDED",
"startDate": "2023-04-20T23:07:09.477000+00:00",
"stopDate": "2023-04-20T23:07:09.732000+00:00"
}
]
}`
```
### Using Step Functions console
You can also view the executions started by a version or an alias from the [Step Functions
console](https://console.aws.amazon.com/states/home?region=us-east-1#/). The following procedure shows how you can view the executions started with a specific version:
1. Open the [Step Functions
console](https://console.aws.amazon.com/states/home?region=us-east-1#/), and then choose an existing state machine for which you've
published a [version](./concepts-state-machine-version.html#procedure-create-versions) or
created an [alias](./concepts-state-machine-alias.html#procedure-create-aliases). This
example shows how to view the executions started with a specific state
machine version.
2. Choose the **Versions** tab, and then choose a
version from the **Versions** list.
###### Tip
Filter by property or value box to search for a specific
version.
3. On the *Version details page*, you can see a list of all the in-progress and past state machine executions started with the selected version.
The following image shows the *Version Details* console page. This page lists executions started by the version *4* of a state machine named ``MathAddDemo``. This list also displays an execution that was started by an alias named ``PROD``. This alias routed the execution traffic to version *4*.
![Illustrative screenshot of the state machine version details console page.](https://docs.aws.amazon.com/images/step-functions/latest/dg/images/view-version-executions.png)
### Using CloudWatch metrics
For each state machine execution that you start with a [Qualified ARN](#qualifiedARN), Step Functions emits additional metrics
with the same name and value as the metrics emitted currently. These additional
metrics contain dimensions for each of the version identifier and alias name with
which you start an execution. With these metrics, you can monitor state machine
executions at the version level and determine when a rollback scenario might be
necessary. You can also [create Amazon CloudWatch
alarms](./procedure-cw-metrics.html#monitoring-using-cloudwatch-console-set-alarm) based on these metrics.
Step Functions emits the following metrics for executions that you start with an alias
or a version:
* `ExecutionTime`
* `ExecutionsAborted`
* `ExecutionsFailed`
* `ExecutionsStarted`
* `ExecutionsSucceeded`
* `ExecutionsTimedOut `
If you started the execution with a version ARN, Step Functions publishes the metric with the `StateMachineArn` and a second metric with `StateMachineArn` and `Version` dimensions.
If you started the execution with an alias ARN, Step Functions emits the following metrics:
* Two metrics for the unqualified ARN and version.
* A metric with the `StateMachineArn` and `Alias` dimensions.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Versions and alias authorization
Deployment example
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.