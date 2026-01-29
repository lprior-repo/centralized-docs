---
url: https://docs.aws.amazon.com/lambda/latest/dg/configuration-concurrency.html
title: Configuring reserved concurrency for a function
word_count: 1010
filtered: true
elements_removed: 0
density_score: 0.85
---

Configuring reserved concurrency for a function - AWS Lambda
Configuring reserved concurrency for a function - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#configuration-concurrency)
[Configuring reserved concurrency](#configuring-concurrency-reserved)[Accurately estimating required reserved concurrency for a function](#estimating-reserved-concurrency)
# Configuring reserved concurrency for a function
In Lambda, [concurrency](./lambda-concurrency.html) is the number of
in-flight requests that your function is currently handling. There are two types
of concurrency controls available:
* Reserved concurrency – This sets both the maximum and minimum number of concurrent instances allocated to your function.
When a function has reserved concurrency, no other function can use that concurrency. Reserved concurrency is useful for
ensuring that your most critical functions always have enough concurrency to handle incoming requests. Additionally, reserved
concurrency can be used for limiting concurrency to prevent overwhelming downstream resources, like database connections.
Reserved concurrency acts as both a lower and upper bound - it reserves the specified capacity exclusively for your function
while also preventing it from scaling beyond that limit. Configuring reserved concurrency for a function incurs no additional charges.
* Provisioned concurrency – This is the number of pre-initialized execution
environments allocated to your function. These execution environments are ready to respond
immediately to incoming function requests. Provisioned concurrency is useful for reducing
cold start latencies for functions and designed to make functions available with double-digit
millisecond response times. Generally, interactive workloads benefit the most from the feature.
Those are applications with users initiating requests, such as web and mobile applications, and
are the most sensitive to latency. Asynchronous workloads, such as data processing pipelines,
are often less latency sensitive and so do not usually need provisioned concurrency. Configuring
provisioned concurrency incurs additional charges to your AWS account.
This topic details how to manage and configure reserved concurrency. For a conceptual
overview of these two types of concurrency controls, see [Reserved concurrency
and provisioned concurrency](https://docs.aws.amazon.com/lambda/latest/dg/lambda-concurrency.html#reserved-and-provisioned). For information on configuring provisioned concurrency,
see [Configuring provisioned concurrency for a function](./provisioned-concurrency.html).
###### Note
Lambda functions linked to an Amazon MQ event source mapping have a default maximum concurrency. For Apache Active MQ,
the maximum number of concurrent instances is 5. For Rabbit MQ, the maximum number of concurrent instances is 1. Setting reserved or provisioned concurrency for your
function doesn't change these limits. To request an increase in the default maximum concurrency when using Amazon MQ, contact Support.
###### Sections
* [Configuring reserved concurrency](#configuring-concurrency-reserved)
* [Accurately estimating required reserved concurrency for a function](#estimating-reserved-concurrency)
## Configuring reserved concurrency
You can configure reserved concurrency settings for a function using the Lambda
console or the Lambda API.
###### To reserve concurrency for a function (console)
1. Open the [Functions page](https://console.aws.amazon.com/lambda/home#/functions) of the Lambda console.
2. Choose the function you want to reserve concurrency for.
3. Choose **Configuration** and then choose
**Concurrency**.
4. Under **Concurrency**, choose **Edit**.
5. Choose **Reserve concurrency**. Enter the amount of concurrency
to reserve for the function.
6. Choose **Save**.
You can reserve up to the **Unreserved account concurrency** value
minus 100. The remaining 100 units of concurrency are for functions that aren't using
reserved concurrency. For example, if your account has a concurrency limit of 1,000,
you cannot reserve all 1,000 units of concurrency to a single function.
![An error occurs if you try to reserve too much concurrency.](https://docs.aws.amazon.com/images/lambda/latest/dg/images/concurrency-reserve-over-limit.png)
Reserving concurrency for a function impacts the concurrency pool that's
available to other functions. For example, if you reserve 100 units of concurrency
for `function-a`, other functions in your account must share the
remaining 900 units of concurrency, even if `function-a` doesn't use
all 100 reserved concurrency units.
To intentionally throttle a function, set its reserved concurrency to 0. This
stops your function from processing any events until you remove the limit.
To configure reserved concurrency with the Lambda API, use the following API
operations.
* [PutFunctionConcurrency](https://docs.aws.amazon.com/lambda/latest/api/API_PutFunctionConcurrency.html)
* [GetFunctionConcurrency](https://docs.aws.amazon.com/lambda/latest/api/API_GetFunctionConcurrency.html)
* [DeleteFunctionConcurrency](https://docs.aws.amazon.com/lambda/latest/api/API_DeleteFunctionConcurrency.html)
For example, to configure reserved concurrency with the AWS Command Line Interface (CLI), use
the `put-function-concurrency` command. The following command reserves
100 concurrency units for a function named `my-function`:
```
`aws lambda put-function-concurrency --function-name my-function \\
--reserved-concurrent-executions 100`
```
You should see output that looks like the following:
```
`{
"ReservedConcurrentExecutions": 100
}`
```
## Accurately estimating required reserved concurrency for a function
If your function is currently serving traffic, you can easily view its concurrency
metrics using [CloudWatch metrics](https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/working_with_metrics.html).
Specifically, the `ConcurrentExecutions` metric shows you the number of
concurrent invocations for each function in your account.
![Graph showing concurrency for a function over time.](https://docs.aws.amazon.com/images/lambda/latest/dg/images/concurrency-concurrent-executions-metrics.png)
The previous graph suggests that this function serves an average of 5 to 10
concurrent requests at any given time, and peaks at 20 requests on a typical day.
Suppose that there are many other functions in your account. **
If this function is critical to your application and you don't want to drop any
requests**, use a number greater than or equal to 20 as your reserved
concurrency setting.
Alternatively, recall that you can also [
calculate concurrency](https://docs.aws.amazon.com/lambda/latest/dg/lambda-concurrency.html#calculating-concurrency) using the following formula:
```
`Concurrency = (average requests per second) \* (average request duration in seconds)`
```
Multiplying average requests per second with the average request duration in
seconds gives you a rough estimate of how much concurrency you need to reserve.
You can estimate average requests per second using the `Invocation`
metric, and the average request duration in seconds using the `Duration`
metric. See [Using CloudWatch metrics with Lambda](./monitoring-metrics.html)
for more details.
You should also be familiar with your upstream and downstream throughput constraints.
While Lambda functions scale seamlessly with load, upstream and downstream dependencies may not have the
same throughput capabilities. If you need to limit how high your function can scale, configure reserved
concurrency on your function.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Function scaling
Configuring provisioned concurrency
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.