---
url: https://docs.aws.amazon.com/lambda/latest/dg/example_cross_StepFunctionsMessenger_section.html
title: Create a messenger application with Step Functions
word_count: 322
filtered: true
elements_removed: 0
density_score: 0.88
---

Create a messenger application with Step Functions - AWS Lambda
Create a messenger application with Step Functions - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#example_cross_StepFunctionsMessenger_section)
# Create a messenger application with Step Functions
The following code example shows how to create an AWS Step Functions messenger application that retrieves message records from a database table.
Python
**SDK for Python (Boto3)**
Shows how to use the AWS SDK for Python (Boto3) with AWS Step Functions to create a messenger application that
retrieves message records from an Amazon DynamoDB table and sends them with Amazon Simple Queue Service (Amazon SQS).
The state machine integrates with an AWS Lambda function to scan the database for unsent messages.
* Create a state machine that retrieves and updates message records from an Amazon DynamoDB table.
* Update the state machine definition to also send messages to Amazon Simple Queue Service (Amazon SQS).
* Start and stop state machine runs.
* Connect to Lambda, DynamoDB, and Amazon SQS from a state machine by using service integrations.
For complete source code and instructions on how to set up and run, see the full example on
[GitHub](https://github.com/awsdocs/aws-doc-sdk-examples/tree/main/python/cross_service/stepfunctions_messenger).
###### Services used in this example
* DynamoDB
* Lambda
* Amazon SQS
* Step Functions
For a complete list of AWS SDK developer guides and code examples, see
[Using Lambda with an AWS SDK](./sdk-general-information-section.html).
This topic also includes information about getting started and details about previous SDK versions.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Create a lending library REST API
Create a serverless application to manage photos
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.