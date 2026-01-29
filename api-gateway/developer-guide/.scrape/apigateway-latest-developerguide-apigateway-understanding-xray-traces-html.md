---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-understanding-xray-traces.html
title: AWS X-Ray traces for
word_count: 524
filtered: true
elements_removed: 0
density_score: 0.65
---

AWS X-Ray traces for Amazon API Gateway APIs - Amazon API Gateway
AWS X-Ray traces for Amazon API Gateway APIs - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#apigateway-understanding-xray-traces)
[Examples of
trace objects for an API Gateway API](#apigateway-understanding-xray-traces-example-segments)[Understanding the
trace](#apigateway-understanding-xray-traces-segments)
# AWS X-Ray traces for
Amazon API Gateway APIs
This section discusses AWS X-Ray trace segments, subsegments, and other trace fields for
Amazon API Gateway APIs.
Before you read this section, review the following topics in the X-Ray Developer
Guide:
* [Use an AWS Management Console](https://docs.aws.amazon.com/xray/latest/devguide/aws-xray-interface-console.html)
* [X-Ray segment documents](https://docs.aws.amazon.com/xray/latest/devguide/aws-xray-interface-api.html#xray-api-segmentdocuments)
* [Concepts](https://docs.aws.amazon.com/xray/latest/devguide/aws-xray.html#xray-concepts)
###### Topics
* [Examples of
trace objects for an API Gateway API](#apigateway-understanding-xray-traces-example-segments)
* [Understanding the
trace](#apigateway-understanding-xray-traces-segments)
## Examples of
trace objects for an API Gateway API
This section discusses some of the objects you may see in a trace for an API Gateway
API.
**Annotations**
Annotations can appear in segments and subsegments. They are used as filtering expressions in sampling rules
to filter traces. For more information, see [Configure sampling
rules](https://docs.aws.amazon.com/xray/latest/devguide/aws-xray-interface-console.html#xray-console-sampling).
Following is an example of an `annotations` object, in which an API stage is identified by
the API ID and the API stage name:
```
`"annotations": {
"aws:api\_id": "a1b2c3d4e5",
"aws:api\_stage": "dev"
}`
```
For more information about annotations, see
[X-Ray segment documents](https://docs.aws.amazon.com/xray/latest/devguide/aws-xray-interface-api.html#xray-api-segmentdocuments), and then choose **X-Ray segment documents**, **Annotations**.
**AWS resource data**
The `aws` object appears only in segments. Following is an
example of an `aws` object that matches the Default sampling rule. For an
in-depth explanation of sampling rules, see [Configure sampling
rules](https://docs.aws.amazon.com/xray/latest/devguide/aws-xray-interface-console.html#xray-console-sampling).
```
`"aws": {
"xray": {
"sampling\_rule\_name": "Default"
},
"api\_gateway": {
"account\_id": "123412341234",
"rest\_api\_id": "a1b2c3d4e5",
"stage": "dev",
"request\_id": "a1b2c3d4-a1b2-a1b2-a1b2-a1b2c3d4e5f6"
}
}`
```
For more information about the `aws` object, see
[X-Ray segment documents](https://docs.aws.amazon.com/xray/latest/devguide/aws-xray-interface-api.html#xray-api-segmentdocuments), and then choose **X-Ray segment documents**, **AWS resource data**.
## Understanding the
trace
Following is a trace segment for an API Gateway stage. For a detailed explanation of the
fields that make up the trace segment, see [X-Ray segment documents](https://docs.aws.amazon.com/xray/latest/devguide/aws-xray-interface-api.html#xray-api-segmentdocuments).
```
` {
"Document": {
"id": "a1b2c3d4a1b2c3d4",
"name": "testxray/dev",
"start\_time": 1533928226.229,
"end\_time": 1533928226.614,
"metadata": {
"default": {
"extended\_request\_id": "abcde12345abcde=",
"request\_id": "a1b2c3d4-a1b2-a1b2-a1b2-a1b2c3d4e5f6"
}
},
"http": {
"request": {
"url": "https://example.com/dev?username=demo&amp;&amp;message=hellofromdemo/",
"method": "GET",
"client\_ip": "192.0.2.0",
"x\_forwarded\_for": true
},
"response": {
"status": 200,
"content\_length": 0
}
},
"aws": {
"xray": {
"sampling\_rule\_name": "Default"
},
"api\_gateway": {
"account\_id": "123412341234",
"rest\_api\_id": "a1b2c3d4e5",
"stage": "dev",
"request\_id": "a1b2c3d4-a1b2-a1b2-a1b2-a1b2c3d4e5f6"
}
},
"annotations": {
"aws:api\_id": "a1b2c3d4e5",
"aws:api\_stage": "dev"
},
"trace\_id": "1-a1b2c3d4-a1b2c3d4a1b2c3d4a1b2c3d4",
"origin": "AWS::ApiGateway::Stage",
"resource\_arn": "arn:aws:apigateway:us-east-1::/restapis/a1b2c3d4e5/stages/dev",
"subsegments": [
{
"id": "abcdefgh12345678",
"name": "Lambda",
"start\_time": 1533928226.233,
"end\_time": 1533928226.6130002,
"http": {
"request": {
"url": "https://example.com/2015-03-31/functions/arn:aws:lambda:us-east-1:123412341234:function:xray123/invocations",
"method": "GET"
},
"response": {
"status": 200,
"content\_length": 62
}
},
"aws": {
"function\_name": "xray123",
"region": "us-east-1",
"operation": "Invoke",
"resource\_names": [
"xray123"
]
},
"namespace": "aws"
}
]
},
"Id": "a1b2c3d4a1b2c3d4"
}`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Configure AWS X-Ray sampling rules
API Gateway portals
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.