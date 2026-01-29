---
url: https://docs.aws.amazon.com/step-functions/latest/dg/state-map.html
title: Map workflow state
word_count: 891
filtered: true
elements_removed: 0
density_score: 0.84
---

Map workflow state - AWS Step Functions
Map workflow state - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/dg/step-functions-dg.pdf#state-map)
[Map processing modes](#concepts-map-process-modes)
# Map workflow state
Use the `Map` state to run a set of workflow steps for each item in a
dataset. The `Map` state's iterations run in parallel, which makes it possible to
process a dataset quickly. `Map` states can use a variety of input types,
including a JSON array, a list of Amazon S3 objects, or a CSV file.
Step Functions provides two types of processing modes for using the `Map` state in your
workflows: *Inline* mode and *Distributed*
mode.
###### Tip
To deploy an example of a workflow that uses a `Map` state, see [Processing arrays of data with Choice and Map](https://catalog.workshops.aws/stepfunctions/choice-and-map) in *The AWS Step Functions Workshop*.
## Map state processing modes
Step Functions provides the following processing modes for the `Map` state depending on how you want to process the items in a dataset.
* **Inline** – Limited-concurrency mode. In this mode, each iteration of the `Map` state runs in the context of the workflow that contains the `Map` state. Step Functions adds the execution history of these iterations to the parent workflow's execution history.
By default, `Map` states run in Inline mode.
In this mode, the `Map` state accepts only a JSON array as input. Also, this mode supports up to 40 concurrent iterations.
For more information, see [Using Map state in Inline mode in Step Functions workflows](./state-map-inline.html).
* **Distributed** – High-concurrency mode. In this mode, the `Map` state runs each iteration as a child workflow execution, which enables high concurrency of up to 10,000 parallel child workflow executions. Each child workflow execution has its own, separate execution history from that of the parent workflow.
In this mode, the `Map` state can accept either a JSON array or an Amazon S3
data source, such as a CSV file, as its input.
For more information, see [Distributed mode](./state-map-distributed.html).
The mode you should use depends on how you want to process the
items in a dataset. Use the `Map` state in Inline mode if your workflow's execution history won't exceed 25,000 entries, or if you don't require more than 40 concurrent iterations.
Use the `Map` state in Distributed mode when you need to orchestrate large-scale parallel workloads that meet any combination of the following conditions:
* The size of your dataset exceeds 256 KiB.
* The workflow's execution event history would exceed 25,000 entries.
* You need a concurrency of more than 40 concurrent iterations.
### Inline mode and Distributed mode differences
The following table highlights the differences between the Inline and Distributed
modes.
|Inline mode|Distributed mode|
|**Supported data sources**|
|
Accepts a JSON array passed from a previous step in the workflow as input.
|
Accepts
the following data sources as input:
* JSON array passed from a previous step in the workflow
* JSON file in an Amazon S3 bucket that contains an array
* CSV file in an Amazon S3 bucket
* Amazon S3 object list
* Amazon S3 inventory
|
|**Map iterations**|
|
In this mode, each iteration of the `Map` state runs in the context of the workflow that contains the `Map` state. Step Functions adds the execution history of these iterations to the parent workflow's execution history.
|
In this mode, the `Map` state runs each iteration as a child workflow execution, which enables high concurrency of up to 10,000 parallel child workflow executions. Each child workflow execution has its own, separate execution history from that of the parent workflow.
|
|**Maximum concurrency for parallel iterations**|
|
Lets you run up to 40 iterations as concurrently as possible.
|
Lets you run up to 10,000 parallel child workflow executions to process millions of data items at one
time.
|
|**Input payload and event history sizes**|
|
Enforces a limit of 256 KiB on the input payload size and 25,000 entries in the execution event history.
|
Lets you overcome the payload size limitation because the `Map` state can read input directly from Amazon S3 data sources.
In this mode, you can also overcome execution history limitations
because the child workflow executions started by the `Map`
state maintain their own, separate execution histories from the parent
workflow's execution history.
|
|**Monitoring and observability**|
|
You can review the workflow's execution history from the console or by
invoking the `[GetExecutionHistory](https://docs.aws.amazon.com/step-functions/latest/apireference/API_GetExecutionHistory.html)` API action.
You can also view the execution history through CloudWatch and
X-Ray.
|
When you run a `Map` state in Distributed mode, Step Functions creates a Map Run resource. A Map Run refers to a set of child workflow executions that a *Distributed Map state* starts. You can view a Map Run in the Step Functions console. You can also invoke the `[DescribeMapRun](https://docs.aws.amazon.com/step-functions/latest/apireference/API_DescribeMapRun.html)` API action. A Map Run also emits metrics to CloudWatch.
For more information, see [Viewing a Distributed Map Run execution in
Step Functions](./concepts-examine-map-run.html).
|
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Parallel
Inline mode
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.