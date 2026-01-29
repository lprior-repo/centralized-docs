---
url: https://docs.aws.amazon.com/lambda/latest/dg/security-resilience.html
title: Resilience in AWS Lambda
word_count: 528
filtered: true
elements_removed: 0
density_score: 0.83
---

Resilience in AWS Lambda - AWS Lambda
Resilience in AWS Lambda - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#security-resilience)
# Resilience in AWS Lambda
The AWS global infrastructure is built around AWS Regions and Availability Zones. AWS Regions provide
multiple physically separated and isolated Availability Zones, which are connected with low-latency,
high-throughput, and highly redundant networking. With Availability Zones, you can design and operate applications
and databases that automatically fail over between Availability Zones without interruption. Availability Zones are
more highly available, fault tolerant, and scalable than traditional single or multiple data center infrastructures.
For more information about AWS Regions and Availability Zones, see [AWS global infrastructure](https://aws.amazon.com/about-aws/global-infrastructure/).
In addition to the AWS global infrastructure, Lambda offers several features to help support your data
resiliency and backup needs.
* **Versioning** – You can use versioning in Lambda to save your function's
code and configuration as you develop it. Together with aliases, you can use versioning to perform blue/green
and rolling deployments. For details, see [Manage Lambda function versions](./configuration-versions.html).
* **Scaling** – When your function receives a request while it's
processing a previous request, Lambda launches another instance of your function to handle the increased load.
Lambda automatically scales to handle 1,000 concurrent executions per Region, a [quota](./gettingstarted-limits.html) that can be increased if needed. For details, see
[Understanding Lambda function scaling](./lambda-concurrency.html).
* **High availability** – Lambda runs your function in multiple
Availability Zones to ensure that it is available to process events in case of a service interruption in a
single zone. If you configure your function to connect to a virtual private cloud (VPC) in your account, specify
subnets in multiple Availability Zones to ensure high availability. For details, see [Giving Lambda functions access to resources in an Amazon VPC](./configuration-vpc.html).
* **Reserved concurrency** – To make sure that your function can always
scale to handle additional requests, you can reserve concurrency for it. Setting reserved concurrency for a
function ensures that it can scale to, but not exceed, a specified number of concurrent invocations. This
ensures that you don't lose requests due to other functions consuming all of the available concurrency. For
details, see [Configuring reserved concurrency for a function](./configuration-concurrency.html).
* **Retries** – For asynchronous invocations and a subset of invocations
triggered by other services, Lambda automatically retries on error with delays between retries. Other clients and
AWS services that invoke functions synchronously are responsible for performing retries. For details, see
[Understanding retry behavior in Lambda](./invocation-retries.html).
* **Dead-letter queue** – For asynchronous invocations, you can configure
Lambda to send requests to a dead-letter queue if all retries fail. A dead-letter queue is an Amazon SNS topic or
Amazon SQS queue that receives events for troubleshooting or reprocessing. For details, see [Adding a dead-letter queue](./invocation-async-retain-records.html#invocation-dlq).
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Compliance validation
Infrastructure security
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.