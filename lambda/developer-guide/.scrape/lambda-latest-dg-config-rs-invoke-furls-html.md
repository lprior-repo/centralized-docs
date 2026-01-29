---
url: https://docs.aws.amazon.com/lambda/latest/dg/config-rs-invoke-furls.html
title: Invoking a response streaming enabled function using Lambda function URLs
word_count: 400
filtered: true
elements_removed: 0
density_score: 0.83
---

Invoking a response streaming enabled function using Lambda function URLs - AWS Lambda
Invoking a response streaming enabled function using Lambda function URLs - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#config-rs-invoke-furls)
# Invoking a response streaming enabled function using Lambda function URLs
###### Note
Your Lambda function can now stream response payloads through the [Amazon API Gateway proxy integration](https://docs.aws.amazon.com/apigateway/latest/developerguide/response-transfer-mode-lambda.html).
You can invoke response streaming enabled functions by changing the invoke mode of your
function's URL. The invoke mode determines which API operation Lambda uses to invoke your
function. The available invoke modes are:
* `BUFFERED` – This is the default option. Lambda invokes your function
using the `Invoke` API operation. Invocation results are available when the
payload is complete. The maximum payload size is 6 MB.
* `RESPONSE\_STREAM` – Enables your function to stream payload results
as they become available. Lambda invokes your function using the
`InvokeWithResponseStream` API operation. The maximum response payload size
is 200 MB.
You can still invoke your function without response streaming by directly calling the
`Invoke` API operation. However, Lambda streams all response payloads for
invocations that come through the function's URL until you change the invoke mode to
`BUFFERED`.
Console
###### To set the invoke mode of a function URL (console)
1. Open the [Functions page](https://console.aws.amazon.com/lambda/home#/functions) of the Lambda console.
2. Choose the name of the function that you want to set the invoke mode for.
3. Choose the **Configuration** tab, and then choose **Function
URL**.
4. Choose **Edit**, then choose **Additional
settings**.
5. Under **Invoke mode**, choose your desired invoke mode.
6. Choose **Save**.
AWS CLI
**To set the invoke mode of a function's URL (AWS CLI)**
```
`aws lambda update-function-url-config \\
--function-name `my-function` \\
--invoke-mode RESPONSE\_STREAM`
```
CloudFormation
**To set the invoke mode of a function's URL (CloudFormation)**
```
`MyFunctionUrl:
Type: AWS::Lambda::Url
Properties:
AuthType: AWS\_IAM
InvokeMode: RESPONSE\_STREAM`
```
For more information about configuring function URLs, see [Lambda function URLs](./urls-configuration.html).
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Writing functions
Tutorial: Creating a response
streaming function with a function URL
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.