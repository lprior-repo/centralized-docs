---
url: https://docs.aws.amazon.com/lambda/latest/dg/durable-configuration.html
title: Configure Lambda durable functions
word_count: 529
filtered: true
elements_removed: 0
density_score: 0.82
---

Configure Lambda durable functions - AWS Lambda
Configure Lambda durable functions - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#durable-configuration)
[Enable durable execution](#durable-config-settings)[IAM permissions for durable functions](#durable-iam-permissions)[Configuration best practices](#durable-config-best-practices)
# Configure Lambda durable functions
To enable durable execution for your Lambda function, you need to configure specific settings that control how long your function can run, how long state data is retained, and what permissions are required.
## Enable durable execution
To enable durable execution for your Lambda function, configure the `DurableConfig` in your function configuration. This setting controls execution timeout, state retention, and versioning behavior.
AWS CLI
```
`
aws lambda update-function-configuration \\
--function-name my-durable-function \\
--durable-config '{
"ExecutionTimeout": 3600,
"RetentionPeriodInDays": 30,
"AllowInvokeLatest": true
}'
`
```
CloudFormation
```
`
Resources:
MyDurableFunction:
Type: AWS::Lambda::Function
Properties:
FunctionName: my-durable-function
Runtime: nodejs18.x
Handler: index.handler
Code:
ZipFile: |
// Your durable function code
DurableConfig:
ExecutionTimeout: 3600
RetentionPeriodInDays: 30
AllowInvokeLatest: true
`
```
**Configuration parameters:**
* `ExecutionTimeout` - Maximum execution time in seconds (up to 31,536,000 for one year)
* `RetentionPeriodInDays` - How long to retain execution state and history (1-365 days)
* `AllowInvokeLatest` - Whether to allow invoking the $LATEST version for durable execution
## IAM permissions for durable functions
Durable functions require additional IAM permissions beyond standard Lambda execution roles. Your function's execution role must include permissions for state management and durable execution APIs.
**Minimum required permissions:**
```
`
{
"Version": "2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Action": [
"lambda:InvokeFunction",
"lambda:GetFunction",
"lambda:ManageDurableState",
"lambda:GetDurableExecution",
"lambda:ListDurableExecutions"
],
"Resource": "arn:aws:lambda:\*:\*:function:\*"
},
{
"Effect": "Allow",
"Action": [
"logs:CreateLogGroup",
"logs:CreateLogStream",
"logs:PutLogEvents"
],
"Resource": "arn:aws:logs:\*:\*:\*"
}
]
}
`
```
**CloudFormation execution role example:**
```
`
DurableFunctionRole:
Type: AWS::IAM::Role
Properties:
AssumeRolePolicyDocument:
Version: '2012-10-17'
Statement:
- Effect: Allow
Principal:
Service: lambda.amazonaws.com
Action: sts:AssumeRole
ManagedPolicyArns:
- arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole
Policies:
- PolicyName: DurableFunctionPolicy
PolicyDocument:
Version: '2012-10-17'
Statement:
- Effect: Allow
Action:
- lambda:ManageDurableState
- lambda:GetDurableExecution
- lambda:ListDurableExecutions
Resource: '\*'
`
```
## Configuration best practices
Follow these best practices when configuring durable functions for production use:
* **Set appropriate execution timeouts** - Configure `ExecutionTimeout` based on your workflow's maximum expected duration. Don't set unnecessarily long timeouts as they affect cost and resource allocation.
* **Balance retention with storage costs** - Set `RetentionPeriodInDays` based on your debugging and audit requirements. Longer retention periods increase storage costs.
* **Use versioning in production** - Set `AllowInvokeLatest` to `false` in production environments and use specific function versions or aliases for durable executions.
* **Monitor state size** - Large state objects increase storage costs and can impact performance. Keep state minimal and use external storage for large data.
* **Configure appropriate logging** - Enable detailed logging for troubleshooting long-running workflows, but be mindful of log volume and costs.
**Production configuration example:**
```
`
{
"ExecutionTimeout": 86400,
"RetentionPeriodInDays": 7,
"AllowInvokeLatest": false
}
`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Timeout
Environment variables
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.