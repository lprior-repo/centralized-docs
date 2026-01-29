---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/http-api-stages.stage-variables-reference.html
title: API Gateway stage
word_count: 294
filtered: true
elements_removed: 0
density_score: 0.90
---

API Gateway stage variables reference for HTTP APIs in API Gateway - Amazon API Gateway
API Gateway stage variables reference for HTTP APIs in API Gateway - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#http-api-stages.stage-variables-reference)
[HTTP
integration URIs](#http-api-stages.stage-variables-in-integration-HTTP-uris)[Lambda functions](#http-api-stages.stage-variables-in-integration-lambda-functions)[AWS integration credentials](#http-api-stages.stage-variables-in-integration-aws-credentials)
# API Gateway stage
variables reference for HTTP APIs in API Gateway
You can use API Gateway stage variables for HTTP APIs in the following cases.
## HTTP
integration URIs
You can use a stage variable as part of an HTTP integration URI, as shown in
the following examples.
* A full URI without protocol – `http://${stageVariables.&lt;&lt;variable\_name&gt;&gt;}`
* A full domain – `http://${stageVariables.&lt;&lt;variable\_name&gt;&gt;}/resource/operation`
* A subdomain – `http://${stageVariables.&lt;&lt;variable\_name&gt;&gt;}.example.com/resource/operation`
* A path – `http://example.com/${stageVariables.&lt;&lt;variable\_name&gt;&gt;}/bar`
* A query string – `http://example.com/foo?q=${stageVariables.&lt;&lt;variable\_name&gt;&gt;}`
## Lambda functions
You can use a stage variable in place of a Lambda function integration name or alias, as
shown in the following examples.
* `arn:aws:apigateway:&lt;&lt;region&gt;&gt;:lambda:path/2015-03-31/functions/arn:aws:lambda:&lt;&lt;region&gt;&gt;:&lt;&lt;account\_id&gt;&gt;:function:${stageVariables.&lt;&lt;function\_variable\_name&gt;&gt;}/invocations`
* `arn:aws:apigateway:&lt;&lt;region&gt;&gt;:lambda:path/2015-03-31/functions/arn:aws:lambda:&lt;&lt;region&gt;&gt;:&lt;&lt;account\_id&gt;&gt;:function:&lt;&lt;function\_name&gt;&gt;:${stageVariables.&lt;&lt;version\_variable\_name&gt;&gt;}/invocations`
###### Note
To use a stage variable for a Lambda function, the function must be in the same account as the API. Stage
variables don't support cross-account Lambda functions.
## AWS integration credentials
You can use a stage variable as part of an AWS user or role credential ARN,
as shown in the following example.
* `arn:aws:iam::&lt;&lt;account\_id&gt;&gt;:${stageVariables.&lt;&lt;variable\_name&gt;&gt;}`
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Use stage variables for HTTP APIs in API Gateway
Security policy for HTTP APIs in API Gateway
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.