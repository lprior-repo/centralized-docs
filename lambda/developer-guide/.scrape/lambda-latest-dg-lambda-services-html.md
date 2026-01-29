---
url: https://docs.aws.amazon.com/lambda/latest/dg/lambda-services.html
title: Invoking Lambda with events from other AWS services
word_count: 769
filtered: true
elements_removed: 0
density_score: 0.86
---

Invoking Lambda with events from other AWS services - AWS Lambda
Invoking Lambda with events from other AWS services - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#lambda-services)
[Creating a trigger](#lambda-invocation-trigger)[Services list](#listing-of-services-and-links-to-more-information)
# Invoking Lambda with events from other AWS services
Some AWS services can directly invoke Lambda functions using *triggers*. These services push events to Lambda, and the function is invoked immediately when the specified event occurs. Triggers are suitable for discrete events and real-time processing. When you [create a trigger using the Lambda console](#lambda-invocation-trigger), the console interacts with the corresponding AWS service to configure the event notification on that service. The trigger is actually stored and managed by the service that generates the events, not by Lambda.
The events are data structured in JSON format. The JSON structure varies depending on the service that
generates it and the event type, but they all contain the data that the function needs to process the
event.
A function can have multiple triggers. Each trigger acts as a client invoking your function independently, and each event that
Lambda passes to your function has data from only one trigger. Lambda converts the event document into an object and passes it to your function handler.
Depending on the service, the event-driven invocation can be [synchronous](./invocation-sync.html) or [asynchronous](./invocation-async.html).
* For synchronous invocation, the service that generates the event waits for the response from your
function. That service defines the data that the function needs to return in the response. The service
controls the error strategy, such as whether to retry on errors.
* For asynchronous invocation, Lambda queues the event before passing it to your function. When Lambda
queues the event, it immediately sends a success response to the service that generated the event. After the
function processes the event, Lambda doesn’t return a response to the event-generating service.
## Creating a trigger
The easiest way to create a trigger is to use the Lambda console. When you create a trigger using the console, Lambda automatically adds the required permissions to the function's [resource-based policy](./access-control-resource-based.html).
###### To create a trigger using the Lambda console
1. Open the [Functions page](https://console.aws.amazon.com/lambda/home#/functions) of the Lambda console.
2. Select the function you want to create a trigger for.
3. In the **Function overview** pane, choose
**Add trigger**.
4. Select the AWS service you want to invoke your function.
5. Fill out the options in the **Trigger configuration** pane
and choose **Add**. Depending on the AWS service you choose to
invoke your function, the trigger configuration options will be different.
## Services that can invoke Lambda functions
The following table lists services that can invoke Lambda functions.
|Service|Method of invocation|
|
[Amazon Managed Streaming for Apache Kafka](./with-msk.html)
|
[Event source mapping](./invocation-eventsourcemapping.html)
|
|
[Self-managed Apache Kafka](./with-kafka.html)
|
[Event source mapping](./invocation-eventsourcemapping.html)
|
|
[Amazon API Gateway](./services-apigateway.html)
|
Event-driven; synchronous invocation
|
|
[AWS CloudFormation](./services-cloudformation.html)
|
Event-driven; asynchronous invocation
|
|
[Amazon CloudWatch Logs](https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/SubscriptionFilters.html#LambdaFunctionExample)
|
Event-driven; asynchronous invocation
|
|
[AWS CodeCommit](https://docs.aws.amazon.com/codecommit/latest/userguide/how-to-notify-lambda-cc.html)
|
Event-driven; asynchronous invocation
|
|
[AWS CodePipeline](https://docs.aws.amazon.com/codepipeline/latest/userguide/actions-invoke-lambda-function.html)
|
Event-driven; asynchronous invocation
|
|
[Amazon Cognito](https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-events.html)
|
Event-driven; synchronous invocation
|
|
[AWS Config](./governance-config.html)
|
Event-driven; asynchronous invocation
|
|
[Amazon Connect](https://docs.aws.amazon.com/connect/latest/adminguide/connect-lambda-functions.html)
|
Event-driven; synchronous invocation
|
|
[Amazon DocumentDB](./with-documentdb.html)
|
[Event source mapping](./invocation-eventsourcemapping.html)
|
|
[Amazon DynamoDB](./with-ddb.html)
|
[Event source mapping](./invocation-eventsourcemapping.html)
|
|
[Elastic Load Balancing (Application Load Balancer)](./services-alb.html)
|
Event-driven; synchronous invocation
|
|
[Amazon EventBridge (CloudWatch Events)](https://docs.aws.amazon.com/eventbridge/latest/userguide/eb-what-is.html)
|
Event-driven; asynchronous invocation (event buses), synchronous or asynchronous invocation (pipes and schedules)
|
|
[AWS IoT](./services-iot.html)
|
Event-driven; asynchronous invocation
|
|
[Amazon Kinesis](./with-kinesis.html)
|
[Event source mapping](./invocation-eventsourcemapping.html)
|
|
[Amazon Data Firehose](https://docs.aws.amazon.com/firehose/latest/dev/data-transformation.html)
|
Event-driven; synchronous invocation
|
|
[Amazon Lex](https://docs.aws.amazon.com/lexv2/latest/dg/lambda.html)
|
Event-driven; synchronous invocation
|
|
[Amazon MQ](./with-mq.html)
|
[Event source mapping](./invocation-eventsourcemapping.html)
|
|
[Amazon Simple Email Service](https://docs.aws.amazon.com/ses/latest/dg/receiving-email-action-lambda.html)
|
Event-driven; asynchronous invocation
|
|
[Amazon Simple Notification Service](./with-sns.html)
|
Event-driven; asynchronous invocation
|
|
[Amazon Simple Queue Service](./with-sqs.html)
|
[Event source mapping](./invocation-eventsourcemapping.html)
|
|
[Amazon Simple Storage Service (Amazon S3)](./with-s3.html)
|
Event-driven; asynchronous invocation
|
|
[Amazon Simple Storage Service Batch](./services-s3-batch.html)
|
Event-driven; synchronous invocation
|
|
[Secrets Manager](https://docs.aws.amazon.com/secretsmanager/latest/userguide/rotate-secrets_lambda.html)
|
Secret rotation
|
|
[AWS Step Functions](https://docs.aws.amazon.com/step-functions/latest/dg/connect-lambda.html)
|
Event-driven; synchronous or asynchronous invocation
|
|
[Amazon VPC Lattice](https://docs.aws.amazon.com/vpc-lattice/latest/ug/lambda-functions.html)
|
Event-driven; synchronous invocation
|
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Troubleshooting
Apache Kafka
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.