---
url: https://docs.aws.amazon.com/lambda/latest/dg/runtime-management-configure-settings.html
title: Configuring Lambda runtime management settings
word_count: 343
filtered: true
elements_removed: 0
density_score: 0.88
---

Configuring Lambda runtime management settings - AWS Lambda
Configuring Lambda runtime management settings - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#runtime-management-configure-settings)
# Configuring Lambda runtime management settings
You can configure runtime management settings using the Lambda console or the AWS Command Line Interface
(AWS CLI).
###### Note
You can configure runtime management settings separately for each [function version](./configuration-versions.html).
###### To configure how Lambda updates your runtime version (console)
1. Open the [Functions page](https://console.aws.amazon.com/lambda/home#/functions) of the Lambda console.
2. Choose the name of a function.
3. On the **Code** tab, under **Runtime settings**,
choose **Edit runtime management configuration**.
4. Under **Runtime management configuration**, choose one of the
following:
* To have your function update to the latest runtime version automatically, choose
**Auto**.
* To have your function update to the latest runtime version when you change the function,
choose **Function update**.
* To have your function update to the latest runtime version only when you change the runtime
version ARN, choose **Manual**. You can find the runtime version ARN under **Runtime management
configuration**. You can also find the ARN in the `INIT\_START`
line of your function logs.
For more information about these options, see [Runtime update modes](./runtimes-update.html#runtime-management-controls).
* Choose **Save**.
**To configure how Lambda updates your runtime version
(AWS CLI)**
To configure runtime management for a function, run the [put-runtime-management-config](https://awscli.amazonaws.com/v2/documentation/api/latest/reference/lambda/put-runtime-management-config.html) AWS CLI command. When using `Manual` mode, you must also provide the runtime version ARN.
```
``aws lambda put-runtime-management-config \\
--function-name `my-function` \\
--update-runtime-on Manual \\
--runtime-version-arn `arn:aws:lambda:us-east-2::runtime:8eeff65f6809a3ce81507fe733fe09b835899b99481ba22fd75b5a7338290ec1```
```
You should see output similar to the following:
```
`{
"UpdateRuntimeOn": "Manual",
"FunctionArn": "arn:aws:lambda:us-east-2:111122223333:function:my-function",
"RuntimeVersionArn": "arn:aws:lambda:us-east-2::runtime:8eeff65f6809a3ce81507fe733fe09b835899b99481ba22fd75b5a7338290ec1"
}`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Runtime version updates
Runtime version roll-back
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.