---
url: https://docs.aws.amazon.com/lambda/latest/dg/tenant-isolation-configure.html
title: Enabling tenant isolation for Lambda functions
word_count: 413
filtered: true
elements_removed: 0
density_score: 0.87
---

Enabling tenant isolation for Lambda functions - AWS Lambda
Enabling tenant isolation for Lambda functions - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#tenant-isolation-configure)
[Console](#tenant-isolation-console)[AWS CLI](#tenant-isolation-cli)[API](#tenant-isolation-api)[CloudFormation](#tenant-isolation-cfn)
# Enabling tenant isolation for Lambda functions
To activate tenant isolation mode, create a new Lambda function. You cannot enable tenant isolation on existing functions.
###### Topics
* [Enabling tenant isolation (console)](#tenant-isolation-console)
* [Enabling tenant isolation (AWS CLI)](#tenant-isolation-cli)
* [Enabling tenant isolation (API)](#tenant-isolation-api)
* [Enabling tenant isolation (CloudFormation)](#tenant-isolation-cfn)
###### To create a Lambda function using the console
1. Open the [Functions page](https://console.aws.amazon.com/lambda/home#/functions) of the Lambda console.
2. Choose **Create function**.
3. Select **Author from scratch**.
4. In the **Basic information** pane, for **Function name**, enter
``image-analysis``.
5. For **Runtime**, choose any of the [supported Lambda runtimes](./lambda-runtimes.html#runtimes-supported).
6. Under additional configurations, **Tenant isolation mode**,
select **Enable**.
7. Review your settings, and choose **Create function**.
## Enabling tenant isolation (AWS CLI)
**Create function with tenant isolation**
When creating a new function using the CLI, add the `--tenancy-config
'{"TenantIsolationMode": "PER\_TENANT"}'` option to your [create-function](https://docs.aws.amazon.com/cli/latest/reference/lambda/create-function.html) request. Example:
```
``aws lambda create-function \\
--function-name `image-analysis` \\
--runtime `nodejs24.x` \\
--zip-file fileb://image-analysis-function.zip \\
--handler image-analysis-function.handler \\
--role `arn:aws:iam:123456789012:role/execution-role` \\
--tenancy-config '{"TenantIsolationMode": "PER\_TENANT"}'``
```
###### To enable tenant isolation using the Lambda API
1. Create a new function with tenant isolation enabled by using the [CreateFunction](https://docs.aws.amazon.com/lambda/latest/api/API_CreateFunction.html) API action with the `TenancyConfig` parameter.
2. Confirm that tenant isolation is enabled for the function by using the [GetFunctionConfiguration](https://docs.aws.amazon.com/lambda/latest/api/API_GetFunctionConfiguration.html) action. If the response shows that `TenantIsolationMode` is `PER\_TENANT`, then tenant isolation is enabled for the function:
```
`"TenancyConfig": {
"TenantIsolationMode": "PER\_TENANT"
}`
```
Invoke the function version with the [Invoke](https://docs.aws.amazon.com/lambda/latest/api/API_Invoke.html) action. For more information, see [Invoking Lambda functions with tenant isolation](./tenant-isolation-invoke.html).
## Enabling tenant isolation (CloudFormation)
The following CloudFormation template creates a new Lambda function with tenant isolation enabled:
```
``MyLambdaFunction:
Type: AWS::Lambda::Function
Properties:
FunctionName: `my-sample-python-lambda`
Runtime: `python3.14`
Role: !GetAtt LambdaExecutionRole.Arn
Handler: index.lambda\_handler
TenancyConfig:
TenantIsolationMode: PER\_TENANT
Code:
ZipFile: |
import json
def lambda\_handler(event, context):
return {
'statusCode': `200`,
'body': json.dumps(f'Hello from Lambda! Tenant-ID: {context.tenant\_id}')
}
Timeout: `10`
MemorySize: `128```
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Tenant isolation
Invoking functions with tenant isolation
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.