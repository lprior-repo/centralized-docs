---
url: https://docs.aws.amazon.com/lambda/latest/dg/lambda-managed-instances-quotas.html
title: Lambda Managed Instances quotas
word_count: 569
filtered: true
elements_removed: 0
density_score: 0.91
---

Lambda Managed Instances quotas - AWS Lambda
Lambda Managed Instances quotas - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#lambda-managed-instances-quotas)
[Lambda API request quotas](#lambda-managed-instances-api-request-quotas)[Lambda Managed Instances resource quotas](#lambda-managed-instances-resource-quotas)[Event source mapping quotas](#lambda-managed-instances-event-source-quotas)[Requesting a quota increase](#lambda-managed-instances-requesting-quota-increase)[Next steps](#lambda-managed-instances-quotas-next-steps)
# Lambda Managed Instances quotas
This page describes the service quotas for AWS Lambda Managed Instances. These quotas are separate from AWS Lambda (default) quotas. Some quotas can be increased upon request.
## Lambda API request quotas
These quotas control the rate at which you can make API calls to manage Lambda Managed Instances capacity providers. The read and write API rate limits apply to all capacity provider operations combined, including creating, updating, describing, and deleting capacity providers.
|Resource|Quota|
|The maximum combined rate (requests per second) for all capacity provider read APIs|15 requests per second. Cannot be increased.|
|The maximum combined rate (requests per second) for all capacity provider write APIs|1 request per second. Cannot be increased.|
## Lambda Managed Instances resource quotas
These quotas define the limits for core Lambda Managed Instances resources within your AWS account. They govern the number of capacity providers you can create and the number of function versions that can be associated with each capacity provider.
|Resource|Quota|
|Capacity providers|1,000. The maximum number of capacity providers created in an account.|
|Function versions per capacity provider|100. The maximum number of function versions per capacity provider. Cannot be increased.|
## Event source mapping quotas
These quotas control the throughput and configuration limits for processing events from various AWS services on Lambda Managed Instances. The throughput limits ensure predictable performance while the mapping count limits help maintain service stability. Event source mappings on Lambda Managed Instances support Amazon SQS, DynamoDB Streams, Amazon Kinesis Data Streams, Amazon MSK, and self-managed Apache Kafka as event sources.
|Resource|Quota|
|Standard SQS event source mapping throughput on Lambda Managed Instances|5 MB per second. Cannot be increased.|
|Standard Kafka event source mapping throughput on Lambda Managed Instances|1 MB per second. Cannot be increased.|
|Standard Kafka event source mappings on Lambda Managed Instances|100 event source mappings. Cannot be increased.|
|Kinesis event source mapping throughput on Lambda Managed Instances|25 MB per second. Can be increased.|
|DynamoDB event source mapping throughput on Lambda Managed Instances|10 MB per second. Can be increased.|
|Invoke request throughput for asynchronous invocations on Lambda Managed Instances|5 MB per second. Can be increased.|
## Requesting a quota increase
For quotas that can be increased, you can request an increase through the Service Quotas console.
**To request a quota increase**
1. Open the Service Quotas console at [console.aws.amazon.com/servicequotas/](http://console.aws.amazon.com/servicequotas/).
2. In the navigation pane, choose **AWS services**.
3. Choose **AWS Lambda**.
4. Select the quota you want to increase.
5. Choose **Request quota increase**.
6. Enter the new quota value and provide a justification for the increase.
7. Choose **Request**.
## Next steps
* Learn about [capacity providers for Lambda Managed Instances](./lambda-managed-instances-capacity-providers.html)
* Understand [scaling for Lambda Managed Instances](./lambda-managed-instances-scaling.html)
* Review runtime-specific guides for [Java](./lambda-managed-instances-java-runtime.html), [Node.js](./lambda-managed-instances-nodejs-runtime.html), and [Python](./lambda-managed-instances-python-runtime.html)
* Configure [VPC connectivity for your capacity providers](./lambda-managed-instances-networking.html)
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Monitoring
Best practices
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.