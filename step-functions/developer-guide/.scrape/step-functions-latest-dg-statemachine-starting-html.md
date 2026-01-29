---
url: https://docs.aws.amazon.com/step-functions/latest/dg/statemachine-starting.html
title: Starting state machine executions in Step Functions
word_count: 283
filtered: true
elements_removed: 0
density_score: 0.86
---

Starting state machine executions in Step Functions - AWS Step Functions
Starting state machine executions in Step Functions - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/dg/step-functions-dg.pdf#statemachine-starting)
# Starting state machine executions in Step Functions
A state machine *execution* occurs when an AWS Step Functions state machine runs
and performs its tasks. Each Step Functions state machine can have multiple simultaneous executions,
which you can initiate from the [Step Functions
console](https://console.aws.amazon.com/states/home?region=us-east-1#/), or by using the AWS SDKs, the Step Functions API actions, or the AWS Command Line Interface
(AWS CLI). An execution receives JSON input and produces JSON output. You can start a Step Functions
execution in the following ways:
* Start an execution in the Step Functions console.
You can start a state machine in the console, watch the execution, and debug failures.
* Call the [StartExecution](https://docs.aws.amazon.com/step-functions/latest/apireference/API_StartExecution.html) API action.
* Use Amazon EventBridge to [start an execution](./tutorial-cloudwatch-events-s3.html) in response to an event.
* Use Amazon EventBridge Scheduler to [start a state machine execution](./using-eventbridge-scheduler.html) on a schedule.
* Start a [nested workflow execution](./concepts-nested-workflows.html) from a Task state.
* Start an execution with [Amazon API Gateway](./tutorial-api-gateway.html).
###### Tip
To learn how to monitor running executions, see the tutorial: [Examining state machine executions in Step Functions](./debug-sm-exec-using-ui.html)
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Exporting to IaC templates
Start from a
Task
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.