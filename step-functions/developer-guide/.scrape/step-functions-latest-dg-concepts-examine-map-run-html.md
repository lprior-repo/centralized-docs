---
url: https://docs.aws.amazon.com/step-functions/latest/dg/concepts-examine-map-run.html
title: Viewing a Distributed Map Run execution in
word_count: 1239
filtered: true
elements_removed: 0
density_score: 0.78
---

Viewing a Distributed Map Run execution in Step Functions - AWS Step Functions
Viewing a Distributed Map Run execution in Step Functions - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/dg/step-functions-dg.pdf#concepts-examine-map-run)
[Map Run execution summary](#map-run-exec-summary)[Error message](#map-run-error-banner)[Item processing status](#map-run-item-process-status)[Listing executions](#map-run-exec-table)
# Viewing a Distributed Map Run execution in
Step Functions
The Step Functions console provides a *Map Run Details* page which displays
all the information related to a *Distributed Map state* execution. For example, you can view the status
of the *Distributed Map state*'s execution, the Map Run's ARN, and the statuses of the items processed
in the child workflow executions started by the *Distributed Map state*. You can also view a list of all
child workflow executions and access their details. If your Map Run was [redriven](./redrive-map-run.html), you will see redrive details in the Map Run execution summary too.
When you run a `Map` state in Distributed mode, Step Functions creates a Map Run resource. A Map Run refers to a set of child workflow executions that a *Distributed Map state* starts, and the runtime settings that control these executions. Step Functions assigns an Amazon Resource Name (ARN) to your Map Run. You can examine a Map Run in the Step Functions console. You can also invoke the `[DescribeMapRun](https://docs.aws.amazon.com/step-functions/latest/apireference/API_DescribeMapRun.html)` API action.
Child workflow executions of a Map Run emit metrics to CloudWatch;. These metrics will have a labelled State Machine ARN with the following format:
`arn:`partition`:states:`region`:`account`:stateMachine:`stateMachineName`/`MapRunLabel or UUID``
The *Map Run Details* has three sections: *Map Run execution summary*, *Item processing status*, and *Listing executions*.
## Map Run execution summary
The *Map Run Execution summary* provides an overview of the execution details of the *Distributed Map state*.
**Details**
Shows execution status of the *Distributed Map state*, the Map Run ARN, and type of the child workflow executions started by the *Distributed Map state*. You can view additional configurations, such as tolerated failure threshold for the Map Run and the maximum concurrency specified for child workflow executions.
**Input and output**
Shows the input received by the *Distributed Map state* and the corresponding output that it generates.
You can view the input dataset and its location, and the input
filters applied to the individual data items in that dataset. If you export
the output of the *Distributed Map state* execution, this tab shows the path to the Amazon S3
bucket that contains the execution results. Otherwise, it points you to the
parent workflow's *Execution Details* page to view the
execution output.
## Error message
If your Map Run failed, the *Map Run Details* page displays an
error message with the reason for failure.
From the **Recover** dropdown button on this error message, you can either redrive the unsuccessful child workflow executions started by this Map Run or start a new execution of the parent workflow.
See [Redriving Map Runs](./redrive-map-run.html) to learn how to restart your workflow.
## Item processing status
The **Item processing status** section displays the status of the
items processed in a Map Run. For example, **Pending** indicates that
a child workflow execution hasn’t started processing the item yet.
Item statuses are dependent on the status of the child workflow executions processing
the items. If a child workflow execution failed, times out, or if a user cancels the
execution, Step Functions doesn't receive any information about the processing result of the
items inside that child workflow execution. All items processed by that execution share
the child workflow execution's status.
For example, say that you want to process 100 items in two child workflow executions,
where each execution processes a batch of 50 items. If one of the executions fails and
the other succeeds, you'll have 50 successful and 50 failed items.
The following table explains the types of processing statuses available for all
items:
|Status|Description|
|
**Pending**
|
Indicates an item that the child workflow execution hasn't started
processing. If a Map Run stops, fails, or a user cancels the
execution before processing of an item starts, the item remains in
**Pending** status.
For example, if a Map Run fails with 10 unprocessed items, these 10 items remain in the **Pending** status.
|
|
**Running**
|
Indicates an item currently being processed by the child workflow
execution.
|
|
**Succeeded**
|
Indicates that the child workflow execution successfully processed
the item.
A successful child workflow execution can't have any failed items.
If one item in the dataset fails during execution, the entire child
workflow execution fails.
|
|
**Failed**
|
Indicates that the child workflow execution either failed to
process the item, or the execution timed out. If any one item
processed by a child workflow execution fails, the entire child
workflow execution fails.
For example, consider a child workflow execution that processed
1000 items. If any one item in that dataset fails during execution,
then Step Functions considers the entire child workflow execution as
failed.
When you [redrive](./redrive-map-run.html) a Map Run, the count of items
with this status is reset to 0.
|
|
**Aborted**
|
Indicates that the child workflow execution started processing the
item, but either the user cancelled the execution, or Step Functions stopped
the execution because the Map Run failed.
For example, consider a **Running** child
workflow execution that's processing 50 items. If the Map Run
stops because of a failure or because a user cancelled the
execution, the child workflow execution and the status of all 50
items changes to **Aborted**.
If
you use a child workflow execution of the
**Express** type, you can't stop the
execution.
When you [redrive](./redrive-map-run.html) a Map Run that starts child workflow executions of type Express, the count of items with this status is reset to 0. This is because Express child workflows are restarted using the [StartExecution](https://docs.aws.amazon.com/step-functions/latest/apireference/API_StartExecution.html) API action instead of being redriven.
|
## Listing executions
The **Executions** section lists all of the child workflow executions
for a specific Map Run. Use the **Search by exact execution name**
field to search for a specific child workflow execution. To see details about a specific execution, select a child
workflow execution from the list and choose the **View details** button
to open its [Execution details](./concepts-view-execution-details.html) page.
You can also use the API or AWS CLI to list child workflow executions started by the Map Run:
* Using the API, call [ListExecutions](https://docs.aws.amazon.com/step-functions/latest/apireference/API_ListExecutions.html) with the `mapRunArn` parameter set to the ARN of the parent workflow.
* Using the AWS CLI, call [list-executions](https://docs.aws.amazon.com/cli/latest/reference/stepfunctions/list-executions.html) with the `map-run-arn` parameter set to the ARN of the parent workflow.
###### Important
The retention policy for child workflow executions is 90 days.
Completed child
workflow executions that are older will not be displayed in
the **Executions** table, even if the *Distributed Map state* or
parent workflow continues to run longer than the retention period. You can view
execution details, including results, of these child workflow executions if you
export the *Distributed Map state* output to an Amazon S3 bucket using `[ResultWriter (Map)](./input-output-resultwriter.html)`.
###### Tip
Choose the refresh button to view the most current list of all child workflow
executions.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Redriving
state machines
Redriving Map Runs
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.