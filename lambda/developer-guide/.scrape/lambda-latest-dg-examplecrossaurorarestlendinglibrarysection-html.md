---
url: https://docs.aws.amazon.com/lambda/latest/dg/example_cross_AuroraRestLendingLibrary_section.html
title: Create a lending library REST API
word_count: 336
filtered: true
elements_removed: 0
density_score: 0.87
---

Create a lending library REST API - AWS Lambda
Create a lending library REST API - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#example_cross_AuroraRestLendingLibrary_section)
# Create a lending library REST API
The following code example shows how to create a lending library where patrons can borrow and return books by using a REST API backed by an Amazon Aurora database.
Python
**SDK for Python (Boto3)**
Shows how to use the AWS SDK for Python (Boto3) with the Amazon Relational Database Service (Amazon RDS) API and AWS Chalice to create a REST API
backed by an Amazon Aurora database. The web service is fully serverless and represents
a simple lending library where patrons can borrow and return books. Learn how to:
* Create and manage a serverless Aurora database cluster.
* Use AWS Secrets Manager to manage database credentials.
* Implement a data storage layer that uses Amazon RDS to move data into
and out of the database.
* Use AWS Chalice to deploy a serverless REST API to Amazon API Gateway and AWS Lambda.
* Use the Requests package to send requests to the web service.
For complete source code and instructions on how to set up and run, see the full example on
[GitHub](https://github.com/awsdocs/aws-doc-sdk-examples/tree/main/python/cross_service/aurora_rest_lending_library).
###### Services used in this example
* API Gateway
* Aurora
* Lambda
* Secrets Manager
For a complete list of AWS SDK developer guides and code examples, see
[Using Lambda with an AWS SDK](./sdk-general-information-section.html).
This topic also includes information about getting started and details about previous SDK versions.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Create a REST API to track COVID-19 data
Create a messenger application
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.