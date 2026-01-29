---
url: https://docs.aws.amazon.com/lambda/latest/api/API_FunctionUrlConfig.html
title: API FunctionUrlConfig.html
word_count: 259
filtered: true
elements_removed: 0
density_score: 0.84
---

FunctionUrlConfig - AWS Lambda
FunctionUrlConfig - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/api/lambda-api.pdf#API_FunctionUrlConfig)
[Contents](#API_FunctionUrlConfig_Contents)[See Also](#API_FunctionUrlConfig_SeeAlso)
## Contents
**
AuthType
**
The type of authentication that your function URL uses. Set to `AWS\_IAM` if you want to restrict access to authenticated
users only. Set to `NONE` if you want to bypass IAM authentication to create a public endpoint. For more information,
see [Security and auth model for Lambda function URLs](https://docs.aws.amazon.com/lambda/latest/dg/urls-auth.html).
Type: String
Valid Values: `NONE | AWS\_IAM`
Required: Yes
**
CreationTime
**
When the function URL was created, in [ISO-8601 format](https://www.w3.org/TR/NOTE-datetime) (YYYY-MM-DDThh:mm:ss.sTZD).
Type: String
Required: Yes
**
FunctionArn
**
The Amazon Resource Name (ARN) of your function.
Type: String
Length Constraints: Minimum length of 0. Maximum length of 10000.
Pattern: `arn:(aws[a-zA-Z-]\*)?:lambda:(eusc-)?[a-z]{2}((-gov)|(-iso([a-z]?)))?-[a-z]+-\\d{1}:\\d{12}:function:[a-zA-Z0-9-\_]+(:(\\$LATEST|[a-zA-Z0-9-\_]+))?`
Required: Yes
**
FunctionUrl
**
The HTTP URL endpoint for your function.
Type: String
Length Constraints: Minimum length of 40. Maximum length of 100.
Required: Yes
**
LastModifiedTime
**
When the function URL configuration was last updated, in [ISO-8601 format](https://www.w3.org/TR/NOTE-datetime) (YYYY-MM-DDThh:mm:ss.sTZD).
Type: String
Required: Yes
**
Cors
**
The [cross-origin resource sharing (CORS)](https://developer.mozilla.org/en-US/docs/Web/HTTP/CORS) settings
for your function URL.
Type: [Cors](./API_Cors.html) object
Required: No
**
InvokeMode
**
Use one of the following options:
* `BUFFERED` – This is the default option. Lambda invokes your function
using the `Invoke` API operation. Invocation results are available when the
payload is complete. The maximum payload size is 6 MB.
* `RESPONSE\_STREAM` – Your function streams payload results as they become available.
Lambda invokes your function using the `InvokeWithResponseStream`
API operation. The maximum response payload size is 200 MB.
Type: String
Valid Values: `BUFFERED | RESPONSE\_STREAM`
Required: No