---
url: https://docs.aws.amazon.com/step-functions/latest/dg/integrate-optimized.html
title: Integrating optimized services with Step Functions
word_count: 417
filtered: true
elements_removed: 0
density_score: 0.85
---

Integrating optimized services with Step Functions - AWS Step Functions
Integrating optimized services with Step Functions - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/dg/step-functions-dg.pdf#integrate-optimized)
# Integrating optimized services with Step Functions
Your workflow can call optimized services directly using the `Resource` field
of a `Task` state. The following topics explain the supported APIs, parameters, and request/response syntax in Amazon States Language for coordinating AWS services.
Depending on workflow type and availability, your workflows call services using one of three service integration patterns:
* [Request a Response (default)](./connect-to-resource.html#connect-default) - wait for HTTP response, then go to the next state
* [Run a Job (.sync)](./connect-to-resource.html#connect-sync) - wait for the job to complete
* [Wait for Callback (.waitForTaskToken)](./connect-to-resource.html#connect-wait-token) - pause a workflow until a task token is returned
Standard Workflows and Express Workflows support the same **integrations** but not the same **integration
patterns**.
* **Standard Workflows** support *Request Response* integrations. Certain services support *Run a Job
(.sync)*, or *Wait for Callback
(.waitForTaskToken)* , and both in some cases. See the following optimized integrations table for details.
* **Express Workflows** only support *Request Response* integrations.
To help decide between the two types, see [Choosing workflow type in Step Functions](./choosing-workflow-type.html).
**AWS SDK integrations in Step Functions**
|Integrated service|Request Response|Run a Job - *.sync*|Wait for Callback - *.waitForTaskToken*|
|[Over two hundred services](./supported-services-awssdk.html#supported-services-awssdk-list)|Standard &amp; Express|*Not supported*|Standard|
**Optimized integrations in Step Functions**
|Integrated service|Request Response|Run a Job - *.sync*|Wait for Callback - *.waitForTaskToken*|
|[Amazon API Gateway](./connect-api-gateway.html)|Standard &amp; Express|*Not supported*|Standard|
|[Amazon Athena](./connect-athena.html)|Standard &amp; Express|Standard|*Not supported*|
|[AWS Batch](./connect-batch.html)|Standard &amp; Express|Standard|*Not supported*|
|[Amazon Bedrock](./connect-bedrock.html)|Standard &amp; Express|Standard|Standard|
|[AWS CodeBuild](./connect-codebuild.html)|Standard &amp; Express|Standard|*Not supported*|
|[Amazon DynamoDB](./connect-ddb.html)|Standard &amp; Express|*Not supported*|*Not supported*|
|[Amazon ECS/Fargate](./connect-ecs.html)|Standard &amp; Express|Standard|Standard|
|[Amazon EKS](./connect-eks.html)|Standard &amp; Express|Standard|Standard|
|[Amazon EMR](./connect-emr.html)|Standard &amp; Express|Standard|*Not supported*|
|[Amazon EMR on EKS](./connect-emr-eks.html)|Standard &amp; Express|Standard|*Not supported*|
|[Amazon EMR Serverless](./connect-emr-serverless.html)|Standard &amp; Express|Standard|*Not supported*|
|[Amazon EventBridge](./connect-eventbridge.html)|Standard &amp; Express|*Not supported*|Standard|
|[AWS Glue](./connect-glue.html)|Standard &amp; Express|Standard|*Not supported*|
|[AWS Glue DataBrew](./connect-databrew.html)|Standard &amp; Express|Standard|*Not supported*|
|[AWS Lambda](./connect-lambda.html)|Standard &amp; Express|*Not supported*|Standard|
|[AWS Elemental MediaConvert](./connect-mediaconvert.html)|Standard &amp; Express|Standard|*Not supported*|
|[Amazon SageMaker AI](./connect-sagemaker.html)|Standard &amp; Express|Standard|*Not supported*|
|[Amazon SNS](./connect-sns.html)|Standard &amp; Express|*Not supported*|Standard|
|[Amazon SQS](./connect-sqs.html)|Standard &amp; Express|*Not supported*|Standard|
|[AWS Step Functions](./connect-stepfunctions.html)|Standard &amp; Express|Standard|Standard|
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Pass parameters
Amazon API Gateway
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.