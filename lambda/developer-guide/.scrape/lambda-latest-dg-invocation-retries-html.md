---
url: https://docs.aws.amazon.com/lambda/latest/dg/invocation-retries.html
title: Understanding retry behavior in Lambda
word_count: 616
filtered: true
elements_removed: 0
density_score: 0.85
---

Understanding retry behavior in Lambda - AWS Lambda
Understanding retry behavior in Lambda - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#invocation-retries)
# Understanding retry behavior in Lambda
When you invoke a function directly, you determine the strategy for handling errors related to your function code.
Lambda does not automatically retry these types of errors on your behalf. To retry, you can manually re-invoke your
function, send the failed event to a queue for debugging, or ignore the error. Your function's code might have run
completely, partially, or not at all. If you retry, ensure that your function's code can handle the same event
multiple times without causing duplicate transactions or other unwanted side effects.
When you invoke a function indirectly, you need to be aware of the retry behavior of the invoker and any service
that the request encounters along the way. This includes the following scenarios.
* **Asynchronous invocation** – Lambda retries function errors twice. If
the function doesn't have enough capacity to handle all incoming requests, events might wait in the queue for
hours to be sent to the function. You can configure a dead-letter queue on the function to capture
events that weren't successfully processed. For more information, see [Adding a dead-letter queue](./invocation-async-retain-records.html#invocation-dlq).
* **Event source mappings** – Event source mappings that read from streams
retry the entire batch of items. Repeated errors block processing of the affected shard until the error is
resolved or the items expire. To detect stalled shards, you can monitor the [Iterator Age](./monitoring-metrics.html) metric.
For event source mappings that read from a queue, you determine the length of time between retries and
destination for failed events by configuring the visibility timeout and redrive policy on the source queue. For
more information, see [How Lambda processes records from stream and queue-based event sources](./invocation-eventsourcemapping.html) and the service-specific topics under [Invoking Lambda with events from other AWS services](./lambda-services.html).
* **AWS services** – AWS services can invoke your function [synchronously](./invocation-sync.html) or asynchronously. For synchronous invocation, the service
decides whether to retry. For example, Amazon S3 batch operations retries the operation if the Lambda function
returns a `TemporaryFailure` response code. Services that proxy requests from an upstream user or
client may have a retry strategy or may relay the error response back to the requester. For example, API Gateway
always relays the error response back to the requester.
For asynchronous invocation, the retry logic is the same regardless of the invocation source. By default, Lambda retries a failed asynchronous invocation up to two times. For more information, see [How Lambda handles errors and retries with asynchronous invocation](./invocation-async-error-handling.html).
* **Other accounts and clients** – When you grant access to other
accounts, you can use [resource-based policies](./access-control-resource-based.html) to restrict
the services or resources they can configure to invoke your function. To protect your function from being
overloaded, consider putting an API layer in front of your function with [Amazon API Gateway](./services-apigateway.html).
To help you deal with errors in Lambda applications, Lambda integrates with services like Amazon CloudWatch and AWS X-Ray.
You can use a combination of logs, metrics, alarms, and tracing to quickly detect and identify issues in your
function code, API, or other resources that support your application. For more information, see [Monitoring, debugging, and troubleshooting Lambda functions](./lambda-monitoring.html).
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Function states
Recursive loop detection
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.