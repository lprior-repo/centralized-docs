---
url: https://docs.aws.amazon.com/step-functions/latest/dg/redrive-map-run.html
title: Redriving Map Runs in Step Functions executions
word_count: 1888
filtered: true
elements_removed: 0
density_score: 0.80
---

Redriving Map Runs in Step Functions executions - AWS Step Functions
Redriving Map Runs in Step Functions executions - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/dg/step-functions-dg.pdf#redrive-map-run)
[Redrive eligibility for child workflows in a Map Run](#redrive-eligibility-map-run)[Child workflow execution redrive behavior](#redrive-child-workflow-behavior)[Scenarios of input used on Map Run redrive](#maprun-redrive-input)[IAM permission to redrive a Map Run](#maprun-iam-permission)[Redriving Map Run in console](#redrive-maprun-console)[Redriving Map Run using API](#redrive-maprun-api)
# Redriving Map Runs in Step Functions executions
You can restart unsuccessful child workflow executions in a Map Run by [redriving](./redrive-executions.html) your
[parent
workflow](./state-map-distributed.html#dist-map-orchestrate-parallel-workloads-key-terms). A redriven
parent workflow redrives all the unsuccessful states, including Distributed Map.
A parent workflow redrives unsuccessful states if there's no `&lt;stateType&gt;Exited` event corresponding to the
`&lt;stateType&gt;Entered` event for a state when the parent workflow completed its
execution. For example, if the event history doesn't contain the `MapStateExited`
event for a `MapStateEntered` event, you can redrive the parent
workflow to redrive all the unsuccessful child workflow executions in the
Map Run.
A Map Run is either not started or fails in the original execution attempt when the state machine doesn't have the required permission to access the [ItemReader (Map)](./input-output-itemreader.html),
[ResultWriter (Map)](./input-output-resultwriter.html), or both. If the Map Run wasn't started in the original execution attempt of the parent workflow,
redriving the parent workflow starts the Map Run for the first time. To resolve this, add the required
permissions to your state machine role, and then redrive the parent
workflow. If you redrive the parent workflow without adding the
required permissions, it attempts to start a new Map Run run that will fail again. For
information about the permissions that you might need, see [IAM policies for using Distributed Map states](./iam-policies-eg-dist-map.html).
###### Topics
* [Redrive eligibility for child workflows in a Map Run](./redrive-map-run.html#redrive-eligibility-map-run)
* [Child workflow execution redrive behavior](./redrive-map-run.html#redrive-child-workflow-behavior)
* [Scenarios of input used on Map Run redrive](./redrive-map-run.html#maprun-redrive-input)
* [IAM permission to redrive a Map Run](./redrive-map-run.html#maprun-iam-permission)
* [Redriving Map Run in console](./redrive-map-run.html#redrive-maprun-console)
* [Redriving Map Run using API](./redrive-map-run.html#redrive-maprun-api)
## Redrive eligibility for child workflows in a Map Run
You can redrive the unsuccessful child workflow executions in a Map Run if the following conditions are met:
* You started the parent workflow execution on or after November 15, 2023. Executions that you started prior to this date aren't eligible for redrive.
* You haven't exceeded the hard limit of 1000 redrives of a given
Map Run. If you've exceeded this limit, you'll receive the `[States.Runtime](./concepts-error-handling.html#states-runtime-error)`
error.
* The parent workflow is redrivable. If the parent workflow
isn't
redrivable, you
can't
redrive the child workflow executions in a Map Run. For
more information about redrive eligibility of a workflow, see
[Redrive eligibility for
unsuccessful executions](./redrive-executions.html#redrive-eligibility).
* The child workflow executions of type Standard in your Map Run
haven't
exceeded the 25,000 execution event history limit. Child workflow executions
that have exceeded the event history limit are counted towards the [tolerated failure threshold](./state-map-distributed.html#maprun-fail-threshold) and
considered as failed. For more information about the redrive eligibility of an execution, see [Redrive eligibility for
unsuccessful executions](./redrive-executions.html#redrive-eligibility).
A new Map Run is started and the existing Map Run isn't redriven in the following cases even if the Map Run failed in the original execution attempt:
* Map Run failed because of the `[States.DataLimitExceeded](./concepts-error-handling.html#error-data-limit-exceed)` error.
* Map Run failed because of the JSON data interpolation error, `[States.Runtime](./concepts-error-handling.html#states-runtime-error)`. For example, you selected a non-existent JSON node in [Filtering state output using OutputPath](./input-output-example.html#input-output-outputpath).
A Map Run can continue to run even after the parent workflow stops or times out. In these scenarios, the redrive doesn't happen immediately:
* Map Run might still be canceling in progress child workflow executions of type
Standard, or waiting for child workflow executions of type Express to complete their
executions.
* Map Run might still be writing results to the [ResultWriter (Map)](./input-output-resultwriter.html),
if you configured it to export results.
In these cases, the running Map Run completes its operations before attempting to redrive.
## Child workflow execution redrive behavior
The redriven child workflow executions in a Map Run exhibit the behavior as described in the following table.
|Express child workflow|Standard child workflow|
|All child workflow executions that failed or timed out in the original execution attempt are started using the [StartExecution](https://docs.aws.amazon.com/step-functions/latest/apireference/API_StartExecution.html) API action. The first state in [ItemProcessor](./state-map-distributed.html#distitemprocessor)
is run first.|All child workflow executions that failed, timed out, or canceled in
the original execution attempt are redriven using the
[RedriveExecution](https://docs.aws.amazon.com/step-functions/latest/apireference/API_RedriveExecution.html)
API action. These child workflows are redriven from the
last state in ItemProcessor that resulted in their unsuccessful
execution.|
|
Unsuccessful executions can always be redriven.
This is because Express child workflow executions are always started
as a new execution using the StartExecution API
action.
|
Unsuccessful Standard child workflow executions can't always be redriven. If an execution isn't redrivable, it won't be attempted again. The last error or output of the execution is permanent. This is possible when an execution exceeds 25,000 history events, or its redrivable period of 14 days has expired.
A Standard child workflow execution might not be redrivable if the parent workflow execution has closed within 14 days, but the child workflow execution closed earlier than 14 days.
|
|Express child workflow executions use the same execution ARN as the
original execution attempt, but you
can't
distinctly identify their individual redrives.|Standard child workflow executions use the same execution ARN as the
original execution
attempt.
You can distinctly identify the individual
redrives in the console and using APIs, such as
[GetExecutionHistory](https://docs.aws.amazon.com/step-functions/latest/apireference/API_GetExecutionHistory.html) and [DescribeExecution](https://docs.aws.amazon.com/step-functions/latest/apireference/API_DescribeExecution.html). For more information, see [Examining redriven
executions](./redrive-executions.html#examine-redriven-executions).|
If you've
redriven
a Map Run, and it has reached its concurrency limit, the child workflow executions in
that Map Run transition to the pending state. The execution status of the Map Run
also transitions to the **Pending redrive** state. Until
the specified concurrency limit can allow for more child workflow executions to run, the
execution remains in the **Pending
redrive** state.
For example, say that the concurrency limit of the Distributed Map in your workflow is 3000, and the number of child workflows to be rerun is 6000. This causes 3000 child workflows to run in parallel while the remaining 3000 workflows remain in the **Pending redrive** state. After the first batch of 3000 child workflows complete their execution, the remaining 3000 child workflows are run.
When a Map Run has completed its execution or is aborted, the count of child workflow executions in the **Pending redrive** state is reset to 0.
## Scenarios of input used on Map Run redrive
Depending on how you provided input to the Distributed Map in the original execution attempt, a redriven Map Run will use the input as described in the following table.
|Input in the original execution attempt|Input used on Map Run redrive|
|Input passed from a previous state or the execution
input.|The redriven Map Run uses the same input.|
|Input passed using [ItemReader (Map)](./input-output-itemreader.html) and the Map Run didn't start the child workflow executions because one of the following conditions is true:
* Map Run failed with the `[States.ItemReaderFailed](./concepts-error-handling.html#itemreaderfailed)` error.
* Map Run failed with the `[States.ResultWriterFailed](./concepts-error-handling.html#resultwriterfailed)` error.
* The parent workflow execution was timed out or canceled before the Map Run was started.
|The redriven Map Run uses the input in the Amazon S3 bucket.|
|Input passed using ItemReader. The Map Run failed after starting or attempting to start child workflow executions.|The redriven Map Run uses the same input provided in the original execution attempt.|
## IAM permission to redrive a Map Run
Step Functions needs appropriate permission to redrive a Map Run. The following IAM policy example grants the least privilege required to your state machine for redriving a Map Run. Remember to replace the `italicized` text with your resource-specific information.
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Action": [
"states:RedriveExecution"
],
"Resource": "arn:aws:states:us-east-2:`123456789012`:execution:`myStateMachineName`/`myMapRunLabel`:\*"
}
]
}`
`
```
## Redriving Map Run in console
The following image shows the execution graph of a state machine that contains a
Distributed Map. This execution failed because the Map Run failed. To
redrive the Map Run, you must redrive the parent
workflow.
![Graph of a failed state machine execution caused by a failed Map Run.](https://docs.aws.amazon.com/images/step-functions/latest/dg/images/redrive-eg-failed-maprun.png)
###### To redrive a Map Run from the console
1. Open the [Step Functions
console](https://console.aws.amazon.com/states/home?region=us-east-1#/), and then choose an existing state machine that contains a
Distributed Map that failed execution.
2. On the state machine detail page, under **Executions**, choose a failed execution instance of this state machine.
3. Choose **Redrive**.
4. In the **Redrive** dialog box, choose **Redrive execution**.
###### Tip
You can also redrive a Map Run from the *Execution Details* or *Map Run Details* page.
If you're on the *Execution Details* page, do one of
the following to redrive the execution:
* Choose **Recover**, and then select
**Redrive from
failure**.
* Choose **Actions**, and then select
**Redrive**.
If you're on the *Map Run Details* page, choose
**Recover**, and then select
**Redrive from
failure**.
Notice that redrive uses the same state machine definition and
ARN. It continues running the execution from the step that failed in the
original execution attempt. In this example,
it's
the Distributed Map step named **Map** and the **Process
input** step inside it. After restarting the unsuccessful child
workflow executions of the Map Run, redrive will continue
execution for the **Done** step.
* From the *Execution Details* page, choose **Map Run** to see the details of the redriven Map Run.
On this page, you can view the results of the redriven execution. For example, in the [Map Run execution summary](./concepts-examine-map-run.html#map-run-exec-summary) section, you can see **Redrive count**, which represents the number of times the Map Run has been redriven. In the **Events** section, you can see the redrive related execution events appended to the events of the original execution attempt. For example, the `MapRunRedriven` event.
After you've redriven a Map Run, you can examine its redrive details in the console or using the [GetExecutionHistory](https://docs.aws.amazon.com/step-functions/latest/apireference/API_GetExecutionHistory.html) and [DescribeExecution](https://docs.aws.amazon.com/step-functions/latest/apireference/API_DescribeExecution.html) API actions. For more information about examining a redriven execution, see [Examining redriven
executions](./redrive-executions.html#examine-redriven-executions).
## Redriving Map Run using API
You can redrive an [eligible](#redrive-eligibility-map-run) Map Run using the
[RedriveExecution](https://docs.aws.amazon.com/step-functions/latest/apireference/API_RedriveExecution.html)
API on the parent workflow. This API restarts unsuccessful child workflow executions in a Map Run.
In the AWS Command Line Interface
(AWS CLI),
run the following command to redrive an unsuccessful state machine
execution. Remember to replace the `italicized` text with your resource-specific information.
```
`aws stepfunctions redrive-execution --execution-arn arn:aws:states:us-east-2:`account-id`:execution:`myStateMachine`:`foo``
```
After you have redriven a Map Run, you can examine its redrive details in the console or using the [DescribeMapRun](https://docs.aws.amazon.com/step-functions/latest/apireference/API_DescribeMapRun.html) API action. To examine the redrive details of Standard workflow executions in a Map Run, you can use the [GetExecutionHistory](https://docs.aws.amazon.com/step-functions/latest/apireference/API_GetExecutionHistory.html) or [DescribeExecution](https://docs.aws.amazon.com/step-functions/latest/apireference/API_DescribeExecution.html) API action. For more information about examining a redriven execution, see [Examining redriven
executions](./redrive-executions.html#examine-redriven-executions).
You can examine the redrive details of Express workflow executions in a Map Run on the [Step Functions console](https://console.aws.amazon.com/states/home?region=us-east-1#/) if you've enabled logging on the parent workflow. For more information, see [Using CloudWatch Logs to log execution history in Step Functions](./cw-logs.html).
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Viewing Map Runs
Processing input and output
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.