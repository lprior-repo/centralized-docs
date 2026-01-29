---
url: https://docs.aws.amazon.com/lambda/latest/dg/example_cross_ApiGatewayWebsocketChat_section.html
title: Create a websocket chat application with API Gateway
word_count: 282
filtered: true
elements_removed: 0
density_score: 0.88
---

Create a websocket chat application with API Gateway - AWS Lambda
Create a websocket chat application with API Gateway - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#example_cross_ApiGatewayWebsocketChat_section)
# Create a websocket chat application with API Gateway
The following code example shows how to create a chat application that is served by a websocket API built on Amazon API Gateway.
Python
**SDK for Python (Boto3)**
Shows how to use the AWS SDK for Python (Boto3) with Amazon API Gateway V2 to
create a websocket API that integrates with AWS Lambda and Amazon DynamoDB.
* Create a websocket API served by API Gateway.
* Define a Lambda handler that stores connections in DynamoDB and posts messages to
other chat participants.
* Connect to the websocket chat application and send messages with the Websockets
package.
For complete source code and instructions on how to set up and run, see the full example on
[GitHub](https://github.com/awsdocs/aws-doc-sdk-examples/tree/main/python/cross_service/apigateway_websocket_chat).
###### Services used in this example
* API Gateway
* DynamoDB
* Lambda
For a complete list of AWS SDK developer guides and code examples, see
[Using Lambda with an AWS SDK](./sdk-general-information-section.html).
This topic also includes information about getting started and details about previous SDK versions.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Create a serverless application to manage photos
Create an application to analyze customer feedback
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.