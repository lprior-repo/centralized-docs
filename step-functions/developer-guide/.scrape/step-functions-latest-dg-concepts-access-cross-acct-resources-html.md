---
url: https://docs.aws.amazon.com/step-functions/latest/dg/concepts-access-cross-acct-resources.html
title: Accessing resources in other AWS accounts in Step Functions
word_count: 1060
filtered: true
elements_removed: 0
density_score: 0.83
---

Accessing resources in other AWS accounts in Step Functions - AWS Step Functions
Accessing resources in other AWS accounts in Step Functions - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/dg/step-functions-dg.pdf#concepts-access-cross-acct-resources)
[Key cross-account resource concepts](#key-terms-cross-acct-access)[Invoking cross-account resources](#invoke-cross-acct-resource)[Cross-account access](#concepts-cross-acct-sync-pattern)
# Accessing resources in other AWS accounts in Step Functions
Step Functions provides cross-account access to resources configured in different AWS accounts in your workflows. Using Step Functions service integrations, you can invoke any cross-account AWS resource even if that AWS service does not support resource-based policies or cross-account calls.
For example, assume you own two AWS accounts, called Development and Testing, in the
same AWS Region. Using cross-account access, your workflow in the Development account can
access resources, such as Amazon S3 buckets, Amazon DynamoDB tables, and Lambda functions that are
available in the Testing account.
###### Important
IAM roles and resource-based policies delegate access across accounts only within a single partition. For example, assume that you have an account in
US West (N. California) in the standard `aws` partition. You also have an account in China (Beijing) in the `aws-cn` partition. You can't use an Amazon S3
resource-based policy in your account in China (Beijing) to allow access for users in your standard `aws` account.
For more information about cross-account access, see [Cross-account policy
evaluation logic](https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_evaluation-logic-cross-account.html) in the *IAM User Guide*.
Although each AWS account maintains complete control over its own resources, with Step Functions,
you can reorganize, swap, add, or remove steps in your workflows without the need to
customize any code. You can do this even as the processes change or applications
evolve.
You can also invoke executions of nested state machines so they're available across
different accounts. Doing so efficiently separates and isolates your workflows. When you use the [.sync](./connect-to-resource.html#connect-sync) service integration pattern in your workflows that access another Step Functions workflow in a different account, Step Functions uses polling that consumes your assigned quota. For more information, see [Run a Job (.sync)](./connect-to-resource.html#connect-sync).
###### Note
Cross-Region AWS SDK integration and cross-Region AWS resource access aren't available in Step Functions.
## Key cross-account resource concepts
**[Execution role](./procedure-create-iam-role.html)**
An IAM role that Step Functions uses to run code and access AWS resources, such
as the AWS Lambda function's Invoke action.
**[Service integration](./integrate-services.html)**
The AWS SDK integration API actions that can be called from within a `Task` state in your workflows.
**source account**
An AWS account that owns the state machine and has started its execution.
**target account**
An AWS account to which you make cross-account calls.
**target role**
An IAM role in the target account that the state machine assumes for making calls to resources
that the target account owns.
**[Run a Job (.sync)](./connect-to-resource.html#connect-sync)**
A service integration pattern used to call services, such as AWS Batch. It also makes a Step Functions
state machine wait for a job to complete before progressing to the next
state. To indicate that Step Functions should wait, append the `.sync`
suffix in the `Resource` field in your `Task` state
definition.
## Invoking cross-account resources
To invoke a cross-account resource in your workflows, do the following:
1. Create an IAM role
in the target account that contains the resource. This role grants the source account,
containing the state machine, permissions to access the target account's resources.
2. In the `Task` state's definition, specify the target IAM role to be assumed by the state machine before invoking the
cross-account resource.
3. Modify the trust policy in the target IAM role to allow the source account to assume this role
temporarily. The trust policy must include the Amazon Resource Name (ARN) of the
state machine defined in the source account. Also, define the appropriate
permissions in the target IAM role to call the AWS resource.
4. Update the source account’s
execution role to include the required permission for assuming the target IAM role.
For an
example, see [Accessing cross-account AWS resources in Step Functions](./tutorial-access-cross-acct-resources.html) in the tutorials.
###### Note
You can configure your state machine to assume an IAM role for accessing
resources from multiple AWS accounts. However, a state machine can assume only one
IAM role at a given time.
![Concept to access cross-account resources](https://docs.aws.amazon.com/images/step-functions/latest/dg/images/cross-account-support-concept.png)
## Cross-account access for .sync integration pattern
When you use the `[.sync](./connect-to-resource.html#connect-sync)` service
integration patterns in your workflows, Step Functions polls the invoked cross-account resource to
confirm the task is complete. This causes a slight delay between the actual task completion time
and the time when Step Functions recognizes the task as complete. The target IAM role needs the required
permissions for a `.sync` invocation to complete this polling loop. To do this, the
target IAM role must have a trust policy that allows the source account to assume it.
Additionally, the target IAM role needs the required permissions to complete the polling
loop.
###### Note
For nested Express Workflows, `arn:aws:states:::states:startExecution.sync` isn't currently supported. Use
`arn:aws:states:::aws-sdk:sfn:startSyncExecution` instead.
### Trust policy update for .sync calls
Update the trust policy of your target IAM role as shown in the following example. The
`sts:ExternalId` field further controls who can assume the role. The state
machine's name must include only characters that the AWS Security Token Service `AssumeRole` API
supports. For more information, see [AssumeRole](https://docs.aws.amazon.com/STS/latest/APIReference/API_AssumeRole.html) in the *AWS Security Token Service API Reference*.
```
`{
"Version": "2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Action": "sts:AssumeRole",
"Principal": {
"AWS": "arn:aws:iam::`sourceAccountID`:role/`InvokeRole`",
},
"Condition": {
"StringEquals": {
"sts:ExternalId": "arn:aws:states:us-east-2:`sourceAccountID`:stateMachine:`stateMachineName`"
}
}
}
]
}`
```
### Permissions required for .sync calls
To grant the permissions required for your state machine, update the required permissions
for the target IAM role. For more information, see [How Step Functions generates IAM policies for integrated
services](./service-integration-iam-templates.html). For example, to start a state machine, add the following
permissions.
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Action": [
"states:StartExecution"
],
"Resource": [
"arn:aws:states:`us-east-1`:`123456789012`:stateMachine:`myStateMachineName`"
]
},
{
"Effect": "Allow",
"Action": [
"states:DescribeExecution",
"states:StopExecution"
],
"Resource": [
"arn:aws:states:`us-east-1`:`123456789012`:execution:`myStateMachineName`:\*"
]
}
]
}`
`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Creating granular permissions for non-admin users
Create VPC endpoints
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.