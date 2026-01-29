---
url: https://docs.aws.amazon.com/step-functions/latest/dg/using-eventbridge-scheduler.html
title: Using Amazon EventBridge Scheduler to start a Step Functions state machine execution
word_count: 1325
filtered: true
elements_removed: 0
density_score: 0.66
---

Using Amazon EventBridge Scheduler to start a Step Functions state machine execution - AWS Step Functions
Using Amazon EventBridge Scheduler to start a Step Functions state machine execution - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/dg/step-functions-dg.pdf#using-eventbridge-scheduler)
[Set up the execution role](#using-eventbridge-scheduler-execution-role)[Create a schedule](#using-eventbridge-scheduler-create)[Related resources](#using-eventbridge-scheduler-related-resources)
# Using Amazon EventBridge Scheduler to start a Step Functions state machine execution
[Amazon EventBridge Scheduler](https://docs.aws.amazon.com/scheduler/latest/UserGuide/what-is-scheduler.html) is a serverless scheduler that allows you to create, run, and manage tasks from one central, managed service. With EventBridge Scheduler, you can create schedules using cron and rate expressions for recurring patterns, or configure one-time invocations. You can set up flexible time windows for delivery, define retry limits, and set the maximum retention time for failed API invocations.
For example, with EventBridge Scheduler, you can start a state machine execution on a schedule when a security related event occurs or to automate a data processing job.
This page explains how to use EventBridge Scheduler to start execution of a Step Functions state machine on a schedule.
###### Topics
* [Set up the execution role](#using-eventbridge-scheduler-execution-role)
* [Create a schedule](#using-eventbridge-scheduler-create)
* [Related resources](#using-eventbridge-scheduler-related-resources)
## Set up the execution role
When you create a new schedule, EventBridge Scheduler must have permission to invoke its target API operation on your behalf. You grant these permissions to EventBridge Scheduler
using an *execution role*. The permission policy you attach to your schedule's execution role defines the required permissions.
These permissions depend on the target API you want EventBridge Scheduler to invoke.
When you use the EventBridge Scheduler console to create a schedule, as in the following procedure, EventBridge Scheduler automatically sets up an execution role based on your selected target.
If you want to create a schedule using one of the EventBridge Scheduler SDKs, the AWS CLI, or CloudFormation, you must have an existing execution role that grants the permissions
EventBridge Scheduler requires to invoke a target. For more information about manually setting up an execution role for your schedule, see [Setting up an execution role](https://docs.aws.amazon.com/scheduler/latest/UserGuide/setting-up.html#setting-up-execution-role)
in the *EventBridge Scheduler User Guide*.
###### To create a schedule by using the console
1. Open the Amazon EventBridge Scheduler console at [https://console.aws.amazon.com/scheduler/home](https://console.aws.amazon.com/scheduler/home/).
2.
On the **Schedules** page, choose **Create schedule**.
3.
On the **Specify schedule detail** page, in the **Schedule name and description** section, do the following:
1. For **Schedule name**, enter a name for your
schedule. For example, `MyTestSchedule`.
2. (Optional) For **Description**, enter a
description for your schedule. For example, `My first
schedule`.
3. For **Schedule group**, choose a schedule group from
the dropdown list. If you don't have a group, choose
**default**. To create a schedule group, choose
**create your own schedule**.
You use schedule groups to add tags to groups of schedules.
4.
1. Choose your schedule options.
|Occurrence|Do this...|
|
**One-time schedule**
A one-time schedule invokes a target only once
at the date and time that you specify.
|
For **Date and time**, do the
following:
* Enter a valid date in
`YYYY/MM/DD` format.
* Enter a timestamp in 24-hour
`hh:mm` format.
* For **Timezone**, choose
the timezone.
|
|
**Recurring schedule**
A recurring schedule invokes a target at a
rate that you specify using a
**cron** expression or rate
expression.
|
1. For **Schedule type**, do
one of the following:
* To use a cron expression to define the
schedule, choose **Cron-based
schedule** and enter the cron
expression.
* To use a rate expression to define the
schedule, choose **Rate-based
schedule** and enter the rate
expression.
For more information about cron and rate
expressions, see [Schedule types on EventBridge Scheduler](https://docs.aws.amazon.com/scheduler/latest/UserGuide/schedule-types.html#cron-based) in the *Amazon EventBridge Scheduler User Guide*.
* For **Flexible time
window**, choose **Off**
to turn off the option, or choose one of the
pre-defined time windows.
For example, if you choose **15
minutes** and you set a recurring
schedule to invoke its target once every hour, the
schedule runs within 15 minutes after the start of
every hour.
|
* (Optional) If you chose **Recurring schedule** in the previous step,
in the **Timeframe** section, do the following:
1. For **Timezone**,
choose a timezone.
2. For **Start date and time**, enter a valid date in
`YYYY/MM/DD` format, and then specify a timestamp in
24-hour `hh:mm` format.
3. For **End date and time**, enter a valid date in
`YYYY/MM/DD` format, and then specify a timestamp in
24-hour `hh:mm` format.
4. Choose **Next**.
5. On the **Select target** page, choose the AWS API operation that EventBridge Scheduler invokes:
1. Choose **AWS Step Functions StartExecution**.
2. In the **StartExecution** section, select a state machine or choose **Create new state machine**.
Currently, you can't run Synchronous Express workflows on a schedule.
3. Enter a JSON payload for the execution. Even if your state machine doesn't require any JSON payload, you must still include input in JSON format as shown in the following example.
```
`{
"Comment": "sampleJSONData"
}`
```
4. Choose **Next**.
5. On the **Settings** page, do the following:
1. To turn on the schedule, under **Schedule
state**, toggle **Enable schedule**.
2. To configure a retry policy for your schedule, under
**Retry policy and dead-letter queue (DLQ)**,
do the following:
* Toggle **Retry**.
* For **Maximum age of event**,
enter the maximum **hour(s)** and
**min(s)** that EventBridge Scheduler must keep an
unprocessed event.
* The maximum time is 24 hours.
* For **Maximum retries**, enter the
maximum number of times EventBridge Scheduler retries the schedule if the
target returns an error.
The maximum value is 185 retries.
With retry policies, if a schedule fails to invoke its target,
EventBridge Scheduler re-runs the schedule. If configured, you must set the maximum
retention time and retries for the schedule.
* Choose where EventBridge Scheduler stores undelivered events.
|**Dead-letter queue (DLQ)**
option|Do this...|
|Don't store|Choose **None**.|
|Store the event in the same AWS account where
you're creating the schedule|
1. Choose **Select an Amazon SQS queue in
my AWS account as a DLQ**.
2. Choose the Amazon Resource Name (ARN) of
the Amazon SQS queue.
|
|Store the event in a different AWS account from
where you're creating the schedule|
1. Choose **Specify an Amazon SQS queue in
other AWS accounts as a DLQ**.
2. Enter the Amazon Resource Name (ARN) of
the Amazon SQS queue.
|
3. To use a customer managed key to encrypt your target input, under
**Encryption**, choose **Customize
encryption settings (advanced)**.
If you choose this option, enter an existing KMS key ARN or choose
**Create an AWS KMS key** to navigate to the
AWS KMS console. For more information about how EventBridge Scheduler encrypts your data
at rest, see [Encryption at
rest](https://docs.aws.amazon.com/scheduler/latest/UserGuide/encryption-rest.html) in the *Amazon EventBridge Scheduler User
Guide*.
4. To have EventBridge Scheduler create a new execution role for you, choose
**Create new role for this schedule**.
Then, enter a name for **Role name**. If you choose
this option, EventBridge Scheduler attaches the required permissions necessary for
your templated target to the role.
5. Choose **Next**.
6. In the **Review and create schedule** page, review the
details of your schedule. In each section, choose **Edit** to
go back to that step and edit its details.
7. Choose **Create schedule**.
You can view a list of your new and existing schedules on the
**Schedules** page. Under the
**Status** column, verify that your new schedule is
**Enabled**.
To confirm that EventBridge Scheduler invoked the state machine, check the [state machine's Amazon CloudWatch logs](./cw-logs.html).
## Related resources
For more information about EventBridge Scheduler, see the following:
* [EventBridge Scheduler User Guide](https://docs.aws.amazon.com/scheduler/latest/UserGuide/what-is-scheduler.html)
* [EventBridge Scheduler API Reference](https://docs.aws.amazon.com/scheduler/latest/APIReference/Welcome.html)
* [EventBridge Scheduler Pricing](https://aws.amazon.com/eventbridge/pricing/#Scheduler)
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Start from a
Task
Viewing workflow runs
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.