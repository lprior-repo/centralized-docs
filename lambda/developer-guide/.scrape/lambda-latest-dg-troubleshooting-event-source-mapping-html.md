---
url: https://docs.aws.amazon.com/lambda/latest/dg/troubleshooting-event-source-mapping.html
title: Troubleshoot event source mapping issues in Lambda
word_count: 773
filtered: true
elements_removed: 0
density_score: 0.81
---

Troubleshoot event source mapping issues in Lambda - AWS Lambda
Troubleshoot event source mapping issues in Lambda - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#troubleshooting-event-source-mapping)
[Identifying and managing throttling](#esm-throttling)[Errors in the processing function](#esm-processing-function)[Identifying and handling backpressure](#esm-backpressure)
# Troubleshoot event source mapping issues in Lambda
Issues in Lambda that relate to an [event source mapping](./invocation-eventsourcemapping.html)
can be more complex because they involve debugging across multiple services. Moreover, event source behavior
can differ based on the exact event source used. This section lists common issues that involve event source
mappings, and provides guidance on how to identify and troubleshoot them.
###### Note
This section uses an Amazon SQS event source for illustration, but the principles apply to other
event source mappings that queue messages for Lambda functions.
## Identifying and managing throttling
In Lambda, throttling occurs when you reach your function's or account's concurrency limit. Consider
the following example, where there is a Lambda function that reads messages from an Amazon SQS queue. This
Lambda function simulates 30 second invocations, and has a batch size of 1. This means that the function
processes only 1 message every 30 seconds:
```
const doWork = (ms) =&gt; new Promise(resolve =&gt; setTimeout(resolve, ms))
exports.handler = async (event) =&gt; {
await doWork(30000)
}
```
With such a long invocation time, messages begin arriving in the queue more rapidly than they are
processed. If your account's unreserved concurrency is 100, Lambda scales up to 100 concurrent executions,
and then throttling occurs. You can see this pattern in the CloudWatch metrics for the function:
![debugging ops figure 10](https://docs.aws.amazon.com/images/lambda/latest/dg/images/debugging-ops-figure-10.png)
CloudWatch metrics for the function show no errors, but the **Concurrent executions** chart
shows that the maximum concurrency of 100 is reached. As a result, the **Throttles** chart
shows the throttling in place.
You can detect throttling with CloudWatch alarms, and setting an alarm anytime the throttling metric for a
function is greater than 0. After you've identified the throttling issue, you have a few options for
resolution:
* Request a concurrency increase from AWS Support in this Region.
* Identify performance issues in the function to improve the speed of processing and therefore
improve throughput.
* Increase the batch size of the function, so more messages are processed by each invocation.
## Errors in the processing function
If the processing function throws errors, Lambda returns the messages to the SQS queue. Lambda prevents
your function from scaling to prevent errors at scale. The following SQS metrics in CloudWatch indicate an issue
with queue processing:
![debugging ops figure 11](https://docs.aws.amazon.com/images/lambda/latest/dg/images/debugging-ops-figure-11.png)
In particular, both the age of the oldest message and the number of messages visible are increasing,
while no messages are deleted. The queue continues to grow but messages are not being processed. The
CloudWatch metrics for the processing Lambda function also indicate that there is a problem:
![debugging ops figure 12](https://docs.aws.amazon.com/images/lambda/latest/dg/images/debugging-ops-figure-12.png)
The **Error count** metric is non-zero and growing, while **Concurrent
executions** have reduced and throttling has stopped. This shows that Lambda has stopped scaling up
your function due to errors. The CloudWatch logs for the function provide details of the type of error.
You can resolve this issue by identifying the function causing the error, then finding and resolving
the error. After you fix the error and deploy the new function code, the CloudWatch metrics should show the
processing recover:
![debugging ops figure 13](https://docs.aws.amazon.com/images/lambda/latest/dg/images/debugging-ops-figure-13.png)
Here, the **Error count** metric drops to zero and the **Success rate**
metric returns to 100%. Lambda starts scaling up the function again, as shown in the **Concurrent
executions** graph.
## Identifying and handling backpressure
If an event producer consistently generates messages for an SQS queue faster than a Lambda function can
process them, backpressure occurs. In this case, SQS monitoring should show the age of the oldest message
growing linearly, along with the approximate number of messages visible. You can detect backpressure in
queues using CloudWatch alarms.
The steps to resolve backpressure depend on your workload. If the primary goal is to increase processing
capability and throughput by the Lambda function, you have a few options:
* Request a concurrency increase in the specific Region from AWS Support.
* Increase the batch size of the function, so more messages are processed by each invocation.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Execution
Networking
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.