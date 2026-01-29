---
url: https://docs.aws.amazon.com/lambda/latest/dg/durable-security.html
title: Security and permissions for Lambda durable functions
word_count: 739
filtered: true
elements_removed: 0
density_score: 0.85
---

Security and permissions for Lambda durable functions - AWS Lambda
Security and permissions for Lambda durable functions - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#durable-security)
[Execution role permissions](#durable-execution-role)[State encryption](#durable-state-encryption)[CloudTrail logging](#durable-cloudtrail-logging)[Cross-account considerations](#durable-cross-account-access)[Inherited Lambda security features](#durable-inherited-security)
# Security and permissions for Lambda durable functions
Lambda durable functions require specific IAM permissions to manage checkpoint operations. Follow the principle of least privilege by granting only the permissions your function needs.
## Execution role permissions
Your durable function's execution role needs permissions to create checkpoints and retrieve execution state. The following policy shows the minimum required permissions:
```
`{
"Version": "2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Action": [
"lambda:CheckpointDurableExecution",
"lambda:GetDurableExecutionState"
],
"Resource": "arn:aws:lambda:region:account-id:function:function-name:\*"
}
]
}`
```
When you create a durable function using the console, Lambda automatically adds these permissions to the execution role. If you create the function using the AWS CLI or AWS CloudFormation, add these permissions to your execution role.
###### Least privilege principle
Scope the `Resource` element to specific function ARNs instead of using wildcards. This limits the execution role to checkpoint operations for only the functions that need them.
**Example: Scoped permissions for multiple functions**
```
`{
"Version": "2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Action": [
"lambda:CheckpointDurableExecution",
"lambda:GetDurableExecutionState"
],
"Resource": [
"arn:aws:lambda:us-east-1:123456789012:function:orderProcessor:\*",
"arn:aws:lambda:us-east-1:123456789012:function:paymentHandler:\*"
]
}
]
}`
```
Alternatively, you can use the AWS managed policy `AWSLambdaBasicDurableExecutionRolePolicy` which includes the required durable execution permissions along with basic Lambda execution permissions for Amazon CloudWatch Logs.
## State encryption
Lambda durable functions automatically enable encryption at rest using AWS owned keys at no charge. Each function execution maintains isolated state that other executions cannot access. Customer managed keys (CMK) are not supported.
Checkpoint data includes:
* Step results and return values
* Execution progress and timeline
* Wait state information
All data is encrypted in transit using TLS when Lambda reads or writes checkpoint data.
### Custom encryption with custom serializers and deserializers
For critical security requirements, you can implement your own encryption and decryption mechanism using custom serializers and deserializers (SerDer) using durable SDK. This approach gives you full control over the encryption keys and algorithms used to protect checkpoint data.
###### Important
When you use custom encryption, you lose visibility of operation results in the Lambda console and API responses. Checkpoint data appears encrypted in execution history and cannot be inspected without decryption.
Your function's execution role needs `kms:Encrypt` and `kms:Decrypt` permissions for the AWS KMS key used in the custom SerDer implementation.
## CloudTrail logging
Lambda logs checkpoint operations as data events in AWS CloudTrail. You can use CloudTrail to audit when checkpoints are created, track execution state changes, and monitor access to durable execution data.
Checkpoint operations appear in CloudTrail logs with the following event names:
* `CheckpointDurableExecution` - Logged when a step completes and creates a checkpoint
* `GetDurableExecutionState` - Logged when Lambda retrieves execution state during replay
To enable data event logging for durable functions, configure a CloudTrail trail to log Lambda data events. For more information, see [Logging data events](https://docs.aws.amazon.com/awscloudtrail/latest/userguide/logging-data-events-with-cloudtrail.html) in the CloudTrail User Guide.
**Example: CloudTrail log entry for checkpoint operation**
```
`{
"eventVersion": "1.08",
"eventTime": "2024-11-16T10:30:45Z",
"eventName": "CheckpointDurableExecution",
"eventSource": "lambda.amazonaws.com",
"requestParameters": {
"functionName": "myDurableFunction",
"executionId": "exec-abc123",
"stepId": "step-1"
},
"responseElements": null,
"eventType": "AwsApiCall"
}`
```
## Cross-account considerations
If you invoke durable functions across AWS accounts, the calling account needs `lambda:InvokeFunction` permission, but checkpoint operations always use the execution role in the function's account. The calling account cannot access checkpoint data or execution state directly.
This isolation ensures that checkpoint data remains secure within the function's account, even when invoked from external accounts.
## Inherited Lambda security features
Durable functions inherit all security, governance, and compliance features from Lambda, including VPC connectivity, environment variable encryption, dead letter queues, reserved concurrency, function URLs, code signing, and compliance certifications (SOC, PCI DSS, HIPAA, etc.).
For detailed information about Lambda security features, see [Security in AWS Lambda](https://docs.aws.amazon.com/lambda/latest/dg/lambda-security.html) in the Lambda Developer Guide. The only additional security considerations for durable functions are the checkpoint permissions documented in this guide.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Examples
Durable execution SDK
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.