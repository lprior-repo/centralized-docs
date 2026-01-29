---
url: https://docs.aws.amazon.com/lambda/latest/dg/example_cross_LambdaScheduledEvents_section.html
title: Use scheduled events to invoke a Lambda function
word_count: 578
filtered: true
elements_removed: 0
density_score: 0.86
---

Use scheduled events to invoke a Lambda function - AWS Lambda
Use scheduled events to invoke a Lambda function - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#example_cross_LambdaScheduledEvents_section)
# Use scheduled events to invoke a Lambda function
The following code examples show how to create an AWS Lambda function invoked by an Amazon EventBridge scheduled event.
Java
**SDK for Java 2.x**
Shows how to create an Amazon EventBridge scheduled event that invokes an AWS Lambda function.
Configure EventBridge to use a cron expression to schedule when the Lambda function is invoked.
In this example, you create a Lambda function by using the Lambda Java runtime API.
This example invokes different AWS services to perform a specific use case. This example demonstrates how to
create an app that sends a mobile text message to your employees that congratulates
them at the one year anniversary date.
For complete source code and instructions on how to set up and run, see the full example on
[GitHub](https://github.com/awsdocs/aws-doc-sdk-examples/tree/main/javav2/usecases/creating_scheduled_events).
###### Services used in this example
* CloudWatch Logs
* DynamoDB
* EventBridge
* Lambda
* Amazon SNS
JavaScript
**SDK for JavaScript (v3)**
Shows how to create an Amazon EventBridge scheduled event that invokes an AWS Lambda function.
Configure EventBridge to use a cron expression to schedule when the Lambda function is invoked.
In this example, you create a Lambda function by using the Lambda JavaScript runtime API.
This example invokes different AWS services to perform a specific use case. This example demonstrates how to
create an app that sends a mobile text message to your employees that congratulates
them at the one year anniversary date.
For complete source code and instructions on how to set up and run, see the full example on
[GitHub](https://github.com/awsdocs/aws-doc-sdk-examples/tree/main/javascriptv3/example_code/cross-services/lambda-scheduled-events).
This example is also available in the
[AWS SDK for JavaScript v3 developer guide](https://docs.aws.amazon.com/sdk-for-javascript/v3/developer-guide/scheduled-events-invoking-lambda-example.html).
###### Services used in this example
* CloudWatch Logs
* DynamoDB
* EventBridge
* Lambda
* Amazon SNS
Python
**SDK for Python (Boto3)**
This example shows how to register an AWS Lambda function as the target of a
scheduled Amazon EventBridge event. The Lambda handler writes a friendly message and the
full event data to Amazon CloudWatch Logs for later retrieval.
* Deploys a Lambda function.
* Creates an EventBridge scheduled event and makes the Lambda function the target.
* Grants permission to let EventBridge invoke the Lambda function.
* Prints the latest data from CloudWatch Logs to show the result of the scheduled invocations.
* Cleans up all resources created during the demo.
This example is best viewed on GitHub. For complete source code and
instructions on how to set up and run, see the full example on
[GitHub](https://github.com/awsdocs/aws-doc-sdk-examples/tree/main/python/example_code/lambda#readme).
###### Services used in this example
* CloudWatch Logs
* DynamoDB
* EventBridge
* Lambda
* Amazon SNS
For a complete list of AWS SDK developer guides and code examples, see
[Using Lambda with an AWS SDK](./sdk-general-information-section.html).
This topic also includes information about getting started and details about previous SDK versions.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Use Step Functions to invoke Lambda functions
Use the Neptune API to query graph data
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.