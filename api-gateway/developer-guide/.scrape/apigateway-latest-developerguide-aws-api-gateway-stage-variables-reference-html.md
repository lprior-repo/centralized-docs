---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/aws-api-gateway-stage-variables-reference.html
title: API Gateway stage variables reference for REST APIs in API Gateway
word_count: 428
filtered: true
elements_removed: 0
density_score: 0.91
---

API Gateway stage variables reference for REST APIs in API Gateway - Amazon API Gateway
API Gateway stage variables reference for REST APIs in API Gateway - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#aws-api-gateway-stage-variables-reference)
[Parameter mapping expressions](#stage-variables-in-parameter-mapping-expressions)[Mapping templates](#stage-variables-in-mapping-templates)[HTTP integration URIs](#stage-variables-in-integration-HTTP-uris)[AWS integration URIs](#stage-variables-in-integration-aws-uris)[AWS integration URIs (Lambda
functions)](#stage-variables-in-integration-lambda-functions)[Amazon Cognito user pool](#stage-variables-in-integration-lambda-functions)[AWS integration credentials](#stage-variables-in-integration-aws-credentials)
# API Gateway stage variables reference for REST APIs in API Gateway
You can use API Gateway stage variables in the following cases.
## Parameter mapping expressions
A stage variable can be used in a parameter mapping expression for an API method's request or response
header parameter, without any partial substitution. In the following example, the stage variable is referenced
without the `$` and the enclosing `{...}`.
* `stageVariables.&lt;&lt;variable\_name&gt;&gt;`
## Mapping templates
A stage variable can be used anywhere in a mapping template, as shown in the following examples.
* `{ "name" : "$stageVariables.&lt;&lt;variable\_name&gt;&gt;"}`
* `{ "name" : "${stageVariables.&lt;&lt;variable\_name&gt;&gt;}"}`
## HTTP integration URIs
A stage variable can be used as part of an HTTP integration URL, as shown in the following examples:
* A full URI without protocol – `http://${stageVariables.&lt;&lt;variable\_name&gt;&gt;}`
* A full domain –
`http://${stageVariables.&lt;&lt;variable\_name&gt;&gt;}/resource/operation`
* A subdomain –
`http://${stageVariables.&lt;&lt;variable\_name&gt;&gt;}.example.com/resource/operation`
* A path – `http://example.com/${stageVariables.&lt;&lt;variable\_name&gt;&gt;}/bar`
* A query string – `http://example.com/foo?q=${stageVariables.&lt;&lt;variable\_name&gt;&gt;}`
## AWS integration URIs
A stage variable can be used as part of AWS URI action or path components, as shown in the following
example.
* `arn:aws:apigateway:&lt;region&gt;:&lt;service&gt;:${stageVariables.&lt;&lt;variable\_name&gt;&gt;}`
## AWS integration URIs (Lambda
functions)
A stage variable can be used in place of a Lambda function name, or version/alias, as shown in the following
examples.
* `arn:aws:apigateway:&lt;&lt;region&gt;&gt;:lambda:path/2015-03-31/functions/arn:aws:lambda:&lt;&lt;region&gt;&gt;:&lt;&lt;account\_id&gt;&gt;:function:${stageVariables.&lt;&lt;function\_variable\_name&gt;&gt;}/invocations`
* `arn:aws:apigateway:&lt;&lt;region&gt;&gt;:lambda:path/2015-03-31/functions/arn:aws:lambda:&lt;&lt;region&gt;&gt;:&lt;&lt;account\_id&gt;&gt;:function:&lt;&lt;function\_name&gt;&gt;:${stageVariables.&lt;&lt;version\_variable\_name&gt;&gt;}/invocations`
###### Note
To use a stage variable for a Lambda function, the function must be in the same account as the API. Stage
variables don't support cross-account Lambda functions.
## Amazon Cognito user pool
A stage variable can be used in place of a Amazon Cognito user pool for a `COGNITO\_USER\_POOLS` authorizer.
* `arn:aws:cognito-idp:&lt;&lt;region&gt;&gt;:&lt;&lt;account\_id&gt;&gt;:userpool/${stageVariables.&lt;&lt;variable\_name&gt;&gt;}`
## AWS integration credentials
A stage variable can be used as part of AWS user/role credential ARN, as shown in the following example.
* `arn:aws:iam::&lt;&lt;account\_id&gt;&gt;:${stageVariables.&lt;&lt;variable\_name&gt;&gt;}`
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Set up stage variables for REST APIs in API Gateway
Set up a canary release deployment
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.