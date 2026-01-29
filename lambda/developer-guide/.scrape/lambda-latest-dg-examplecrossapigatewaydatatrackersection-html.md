---
url: https://docs.aws.amazon.com/lambda/latest/dg/example_cross_ApiGatewayDataTracker_section.html
title: Create an API Gateway REST API to track COVID-19 data
word_count: 352
filtered: true
elements_removed: 0
density_score: 0.87
---

Create an API Gateway REST API to track COVID-19 data - AWS Lambda
Create an API Gateway REST API to track COVID-19 data - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#example_cross_ApiGatewayDataTracker_section)
# Create an API Gateway REST API to track COVID-19 data
The following code example shows how to create a REST API that simulates a system to track daily cases of COVID-19 in the United States, using fictional data.
Python
**SDK for Python (Boto3)**
Shows how to use AWS Chalice with the AWS SDK for Python (Boto3) to
create a serverless REST API that uses Amazon API Gateway, AWS Lambda, and
Amazon DynamoDB. The REST API simulates a system that tracks daily cases
of COVID-19 in the United States, using fictional data. Learn how to:
* Use AWS Chalice to define routes in Lambda functions that
are called to handle REST requests that come through API Gateway.
* Use Lambda functions to retrieve and store data in a DynamoDB
table to serve REST requests.
* Define table structure and security role resources in an AWS CloudFormation template.
* Use AWS Chalice and CloudFormation to package and deploy all necessary resources.
* Use CloudFormation to clean up all created resources.
For complete source code and instructions on how to set up and run, see the full example on
[GitHub](https://github.com/awsdocs/aws-doc-sdk-examples/tree/main/python/cross_service/apigateway_covid-19_tracker).
###### Services used in this example
* API Gateway
* CloudFormation
* DynamoDB
* Lambda
For a complete list of AWS SDK developer guides and code examples, see
[Using Lambda with an AWS SDK](./sdk-general-information-section.html).
This topic also includes information about getting started and details about previous SDK versions.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Automatically migrate known users with a Lambda function
Create a lending library REST API
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.