---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/import-api-aws-variables.html
title: AWS variables for OpenAPI import
word_count: 285
filtered: true
elements_removed: 0
density_score: 0.81
---

AWS variables for OpenAPI import - Amazon API Gateway
AWS variables for OpenAPI import - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#import-api-aws-variables)
[AWS variables example](#import-api-aws-variables-example)
# AWS variables for OpenAPI import
You can use the following AWS variables in OpenAPI definitions. API Gateway resolves the variables when the API is imported. To specify a variable,
use `${`variable-name`}`. The following table describes the available AWS variables.
|Variable name|Description|
|`AWS::AccountId`|The AWS account ID that imports the API. For example, 123456789012.|
|`AWS::Partition`|The AWS partition in which the API is imported. For standard AWS Regions, the partition is `aws`.|
|`AWS::Region`|The AWS Region in which the API is imported. For example, `us-east-2`.|
## AWS variables example
The following example uses AWS variables to specify an AWS Lambda function for an integration.
OpenAPI 3.0
```
`openapi: "3.0.1"
info:
title: "tasks-api"
version: "v1.0"
paths:
/:
get:
summary: List tasks
description: Returns a list of tasks
responses:
200:
description: "OK"
content:
application/json:
schema:
type: array
items:
$ref: "#/components/schemas/Task"
500:
description: "Internal Server Error"
content: {}
x-amazon-apigateway-integration:
uri:
arn:${AWS::Partition}:apigateway:${AWS::Region}:lambda:path/2015-03-31/functions/arn:${AWS::Partition}:lambda:${AWS::Region}:${AWS::AccountId}:function:`LambdaFunctionName`/invocations
responses:
default:
statusCode: "200"
passthroughBehavior: "when\_no\_match"
httpMethod: "POST"
contentHandling: "CONVERT\_TO\_TEXT"
type: "aws\_proxy"
components:
schemas:
Task:
type: object
properties:
id:
type: integer
name:
type: string
description:
type: string`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Set the OpenAPI
basePath property
Errors and warnings from importing your API into API Gateway
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.