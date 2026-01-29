---
url: https://docs.aws.amazon.com/lambda/latest/dg/security-iam-awsmanpol.html
title: AWS managed policies for AWS Lambda
word_count: 2002
filtered: true
elements_removed: 0
density_score: 0.88
---

AWS managed policies for AWS Lambda - AWS Lambda
AWS managed policies for AWS Lambda - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#security-iam-awsmanpol)
[AWSLambda\_FullAccess](#lambda-security-iam-awsmanpol-AWSLambda_FullAccess)[AWSLambda\_ReadOnlyAccess](#lambda-security-iam-awsmanpol-AWSLambda_ReadOnlyAccess)[AWSLambdaBasicExecutionRole](#lambda-security-iam-awsmanpol-AWSLambdaBasicExecutionRole)[AWSLambdaBasicDurableExecutionRolePolicy](#lambda-security-iam-awsmanpol-AWSLambdaBasicDurableExecutionRolePolicy)[AWSLambdaDynamoDBExecutionRole](#lambda-security-iam-awsmanpol-AWSLambdaDynamoDBExecutionRole)[AWSLambdaENIManagementAccess](#lambda-security-iam-awsmanpol-AWSLambdaENIManagementAccess)[AWSLambdaInvocation-DynamoDB](#lambda-security-iam-awsmanpol-AWSLambdaInvocation-DynamoDB)[AWSLambdaKinesisExecutionRole](#lambda-security-iam-awsmanpol-AWSLambdaKinesisExecutionRole)[AWSLambdaMSKExecutionRole](#lambda-security-iam-awsmanpol-AWSLambdaMSKExecutionRole)[AWSLambdaRole](#lambda-security-iam-awsmanpol-AWSLambdaRole)[AWSLambdaSQSQueueExecutionRole](#lambda-security-iam-awsmanpol-AWSLambdaSQSQueueExecutionRole)[AWSLambdaVPCAccessExecutionRole](#lambda-security-iam-awsmanpol-AWSLambdaVPCAccessExecutionRole)[AWSLambdaManagedEC2ResourceOperator](#lambda-security-iam-awsmanpol-AWSLambdaManagedEC2ResourceOperator)[AWSLambdaServiceRolePolicy](#lambda-security-iam-awsmanpol-AWSLambdaServiceRolePolicy)[Policy updates](#lambda-security-iam-awsmanpol-updates)
# AWS managed policies for AWS Lambda
An AWS managed policy is a standalone policy that is created and administered by AWS. AWS managed policies are designed
to provide permissions for many common use cases so that you can start assigning permissions to users, groups, and roles.
Keep in mind that AWS managed policies might not grant least-privilege permissions for your specific use cases because
they're available for all AWS customers to use. We recommend that you reduce permissions further by defining
[
customer managed policies](https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies_managed-vs-inline.html#customer-managed-policies) that are specific to your use cases.
You cannot change the permissions defined in AWS managed policies. If AWS updates the permissions defined in an AWS
managed policy, the update affects all principal identities (users, groups, and roles) that the policy is attached to. AWS is
most likely to update an AWS managed policy when a new AWS service is launched or new API operations become available for
existing services.
For more information, see [AWS managed policies](https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies_managed-vs-inline.html#aws-managed-policies) in the
*IAM User Guide*.
###### Topics
* [AWS managed policy: AWSLambda\_FullAccess](#lambda-security-iam-awsmanpol-AWSLambda_FullAccess)
* [AWS managed policy: AWSLambda\_ReadOnlyAccess](#lambda-security-iam-awsmanpol-AWSLambda_ReadOnlyAccess)
* [AWS managed policy: AWSLambdaBasicExecutionRole](#lambda-security-iam-awsmanpol-AWSLambdaBasicExecutionRole)
* [AWS managed policy: AWSLambdaBasicDurableExecutionRolePolicy](#lambda-security-iam-awsmanpol-AWSLambdaBasicDurableExecutionRolePolicy)
* [AWS managed policy: AWSLambdaDynamoDBExecutionRole](#lambda-security-iam-awsmanpol-AWSLambdaDynamoDBExecutionRole)
* [AWS managed policy: AWSLambdaENIManagementAccess](#lambda-security-iam-awsmanpol-AWSLambdaENIManagementAccess)
* [AWS managed policy: AWSLambdaInvocation-DynamoDB](#lambda-security-iam-awsmanpol-AWSLambdaInvocation-DynamoDB)
* [AWS managed policy: AWSLambdaKinesisExecutionRole](#lambda-security-iam-awsmanpol-AWSLambdaKinesisExecutionRole)
* [AWS managed policy: AWSLambdaMSKExecutionRole](#lambda-security-iam-awsmanpol-AWSLambdaMSKExecutionRole)
* [AWS managed policy: AWSLambdaRole](#lambda-security-iam-awsmanpol-AWSLambdaRole)
* [AWS managed policy: AWSLambdaSQSQueueExecutionRole](#lambda-security-iam-awsmanpol-AWSLambdaSQSQueueExecutionRole)
* [AWS managed policy: AWSLambdaVPCAccessExecutionRole](#lambda-security-iam-awsmanpol-AWSLambdaVPCAccessExecutionRole)
* [AWS managed policy: AWSLambdaManagedEC2ResourceOperator](#lambda-security-iam-awsmanpol-AWSLambdaManagedEC2ResourceOperator)
* [AWS managed policy: AWSLambdaServiceRolePolicy](#lambda-security-iam-awsmanpol-AWSLambdaServiceRolePolicy)
* [Lambda updates to AWS managed
policies](#lambda-security-iam-awsmanpol-updates)
## AWS managed policy: AWSLambda\_FullAccess
This policy grants full access to Lambda actions. It also grants permissions to other AWS services that are used to develop and maintain Lambda resources.
You can attach the `AWSLambda\_FullAccess` policy to your users, groups, and roles.
**Permissions details**
This policy includes the following permissions:
* `lambda` – Allows principals full access to Lambda.
* `cloudformation` – Allows principals to describe AWS CloudFormation stacks and list the resources in those stacks.
* `cloudwatch` – Allows principals to list Amazon CloudWatch metrics and get metric data.
* `ec2` – Allows principals to describe security groups, subnets, and VPCs.
* `iam` – Allows principals to get policies, policy versions,
roles, role policies, attached role policies, and the list of roles. This policy
also allows principals to pass roles to Lambda. The `PassRole`
permission is used when you assign an execution role to a function. The
`CreateServiceLinkedRole` permission is used when creating a
service-linked role.
* `kms` – Allows principals to list aliases and describe key
for volume encryption.
* `logs` – Allows principals to describe log streams, get log events, filter log events, and to start and stop Live Tail sessions.
* `states` – Allows principals to describe and list AWS Step Functions state machines.
* `tag` – Allows principals to get resources based on their tags.
* `xray` – Allows principals to get AWS X-Ray trace summaries and retrieve a list of traces specified by ID.
For more information about this policy, including the JSON policy document and policy versions, see
[AWSLambda\_FullAccess](https://docs.aws.amazon.com/aws-managed-policy/latest/reference/AWSLambda_FullAccess.html)
in the *AWS Managed Policy Reference Guide*.
## AWS managed policy: AWSLambda\_ReadOnlyAccess
This policy grants read-only access to Lambda resources and to other AWS services that are used to develop and maintain Lambda resources.
You can attach the `AWSLambda\_ReadOnlyAccess` policy to your users, groups, and roles.
**Permissions details**
This policy includes the following permissions:
* `lambda` – Allows principals to get and list all resources.
* `cloudformation` – Allows principals to describe and list AWS CloudFormation stacks and list the resources in those stacks.
* `cloudwatch` – Allows principals to list Amazon CloudWatch metrics and get metric data.
* `ec2` – Allows principals to describe security groups, subnets, and VPCs.
* `iam` – Allows principals to get policies, policy versions, roles, role policies, attached role policies, and the list of roles.
* `kms` – Allows principals to list aliases.
* `logs` – Allows principals to describe log streams, get log events, filter log events, and to start and stop Live Tail sessions.
* `states` – Allows principals to describe and list AWS Step Functions state machines.
* `tag` – Allows principals to get resources based on their tags.
* `xray` – Allows principals to get AWS X-Ray trace summaries and retrieve a list of traces specified by ID.
For more information about this policy, including the JSON policy document and policy versions, see
[AWSLambda\_ReadOnlyAccess](https://docs.aws.amazon.com/aws-managed-policy/latest/reference/AWSLambda_ReadOnlyAccess.html)
in the *AWS Managed Policy Reference Guide*.
## AWS managed policy: AWSLambdaBasicExecutionRole
This policy grants permissions to upload logs to CloudWatch Logs.
You can attach the `AWSLambdaBasicExecutionRole` policy to your users, groups, and roles.
For more information about this policy, including the JSON policy document and policy versions, see
[AWSLambdaBasicExecutionRole](https://docs.aws.amazon.com/aws-managed-policy/latest/reference/AWSLambdaBasicExecutionRole.html)
in the *AWS Managed Policy Reference Guide*.
## AWS managed policy: AWSLambdaBasicDurableExecutionRolePolicy
This policy provides write permissions to CloudWatch Logs and read/write permissions to durable execution APIs used by Lambda durable functions. This policy provides the essential permissions required for Lambda durable functions, which use durable execution APIs to persist progress and maintain state across function invocations.
You can attach the `AWSLambdaBasicDurableExecutionRolePolicy` policy to your users, groups, and roles.
**Permissions details**
This policy includes the following permissions:
* `logs` – Allows principals to create log groups and log streams, and write log events to CloudWatch Logs.
* `lambda` – Allows principals to checkpoint durable execution state and retrieve durable execution state for Lambda durable functions.
To view more details about the policy, including the latest version of the JSON policy
document, see [AWSLambdaBasicDurableExecutionRolePolicy](https://docs.aws.amazon.com/aws-managed-policy/latest/reference/AWSLambdaBasicDurableExecutionRolePolicy.html) in the *AWS Managed Policy Reference
Guide*.
## AWS managed policy: AWSLambdaDynamoDBExecutionRole
This policy grants permissions to read records from an Amazon DynamoDB stream and write to CloudWatch Logs.
You can attach the `AWSLambdaDynamoDBExecutionRole` policy to your users, groups, and roles.
For more information about this policy, including the JSON policy document and policy versions, see
[AWSLambdaDynamoDBExecutionRole](https://docs.aws.amazon.com/aws-managed-policy/latest/reference/AWSLambdaDynamoDBExecutionRole.html)
in the *AWS Managed Policy Reference Guide*.
## AWS managed policy: AWSLambdaENIManagementAccess
This policy grants permissions to create, describe, and delete elastic network interfaces used by a VPC-enabled Lambda function.
You can attach the `AWSLambdaENIManagementAccess` policy to your users, groups, and roles.
For more information about this policy, including the JSON policy document and policy versions, see
[AWSLambdaENIManagementAccess](https://docs.aws.amazon.com/aws-managed-policy/latest/reference/AWSLambdaENIManagementAccess.html)
in the *AWS Managed Policy Reference Guide*.
## AWS managed policy: AWSLambdaInvocation-DynamoDB
This policy grants read access to Amazon DynamoDB Streams.
You can attach the `AWSLambdaInvocation-DynamoDB` policy to your users, groups, and roles.
For more information about this policy, including the JSON policy document and policy versions, see
[AWSLambdaInvocation-DynamoDB](https://docs.aws.amazon.com/aws-managed-policy/latest/reference/AWSLambdaInvocation-DynamoDB.html)
in the *AWS Managed Policy Reference Guide*.
## AWS managed policy: AWSLambdaKinesisExecutionRole
This policy grants permissions to read events from an Amazon Kinesis data stream and write to CloudWatch Logs.
You can attach the `AWSLambdaKinesisExecutionRole` policy to your users, groups, and roles.
For more information about this policy, including the JSON policy document and policy versions, see
[AWSLambdaKinesisExecutionRole](https://docs.aws.amazon.com/aws-managed-policy/latest/reference/AWSLambdaKinesisExecutionRole.html)
in the *AWS Managed Policy Reference Guide*.
## AWS managed policy: AWSLambdaMSKExecutionRole
This policy grants permissions to read and access records from an Amazon Managed Streaming for Apache Kafka cluster, manage elastic network interfaces, and write to CloudWatch Logs.
You can attach the `AWSLambdaMSKExecutionRole` policy to your users, groups, and roles.
For more information about this policy, including the JSON policy document and policy versions, see
[AWSLambdaMSKExecutionRole](https://docs.aws.amazon.com/aws-managed-policy/latest/reference/AWSLambdaMSKExecutionRole.html)
in the *AWS Managed Policy Reference Guide*.
## AWS managed policy: AWSLambdaRole
This policy grants permissions to invoke Lambda functions.
You can attach the `AWSLambdaRole` policy to your users, groups, and roles.
For more information about this policy, including the JSON policy document and policy versions, see
[AWSLambdaRole](https://docs.aws.amazon.com/aws-managed-policy/latest/reference/AWSLambdaRole.html)
in the *AWS Managed Policy Reference Guide*.
## AWS managed policy: AWSLambdaSQSQueueExecutionRole
This policy grants permissions to read and delete messages from an Amazon Simple Queue Service queue, and grants write permissions to CloudWatch Logs.
You can attach the `AWSLambdaSQSQueueExecutionRole` policy to your users, groups, and roles.
For more information about this policy, including the JSON policy document and policy versions, see
[AWSLambdaSQSQueueExecutionRole](https://docs.aws.amazon.com/aws-managed-policy/latest/reference/AWSLambdaSQSQueueExecutionRole.html)
in the *AWS Managed Policy Reference Guide*.
## AWS managed policy: AWSLambdaVPCAccessExecutionRole
This policy grants permissions to manage elastic network interfaces within an Amazon Virtual Private Cloud and write to CloudWatch Logs.
You can attach the `AWSLambdaVPCAccessExecutionRole` policy to your users, groups, and roles.
For more information about this policy, including the JSON policy document and policy versions, see
[AWSLambdaVPCAccessExecutionRole](https://docs.aws.amazon.com/aws-managed-policy/latest/reference/AWSLambdaVPCAccessExecutionRole.html)
in the *AWS Managed Policy Reference Guide*.
## AWS managed policy: AWSLambdaManagedEC2ResourceOperator
This policy enables automated Amazon Elastic Compute Cloud instance management for Lambda capacity providers. It grants permissions to the Lambda scaler service to perform instance lifecycle operations on your behalf.
You can attach the `AWSLambdaManagedEC2ResourceOperator` policy to your users, groups, and roles.
**Permissions details**
This policy includes the following permissions:
* `ec2:RunInstances` – Allows Lambda to launch new Amazon EC2 instances with the condition that ec2:ManagedResourceOperator equals scaler.lambda.amazonaws.com and restricts AMI usage to Amazon-owned images only.
* `ec2:DescribeInstances` and `ec2:DescribeInstanceStatus` – Allows Lambda to monitor instance status and retrieve instance information.
* `ec2:CreateTags` – Allows Lambda to tag Amazon EC2 resources for management and identification purposes.
* `ec2:DescribeAvailabilityZones` – Allows Lambda to view available zones for instance placement decisions.
* `ec2:DescribeCapacityReservations` – Allows Lambda to check capacity reservations for optimal instance placement.
* `ec2:DescribeInstanceTypes` and `ec2:DescribeInstanceTypeOfferings` – Allows Lambda to review available instance types and their offerings.
* `ec2:DescribeSubnets` – Allows Lambda to examine subnet configurations for network planning.
* `ec2:DescribeSecurityGroups` – Allows Lambda to retrieve security group information for network interface configuration.
* `ec2:CreateNetworkInterface` – Allows Lambda to create network interfaces and manage subnet and security group associations.
* `ec2:AttachNetworkInterface` – Allows Lambda to attach
network interfaces to Amazon EC2 instances with the condition that
`ec2:ManagedResourceOperator` equals [scaler.lambda.amazonaws.com](http://scaler.lambda.amazonaws.com/).
For more information about this policy, including the JSON policy document and policy versions, see
[AWSLambdaManagedEC2ResourceOperator](https://docs.aws.amazon.com/aws-managed-policy/latest/reference/AWSLambdaManagedEC2ResourceOperator.html)
in the *AWS Managed Policy Reference Guide*.
## AWS managed policy: AWSLambdaServiceRolePolicy
This policy is attached to the service-linked role named AWSServiceRoleForLambda to allow Lambda to terminate instances managed as part of Lambda capacity providers.
**Permissions details**
This policy includes the following permissions:
* `ec2:TerminateInstances` – Allows Lambda to terminate EC2 instances with the condition that ec2:ManagedResourceOperator equals scaler.lambda.amazonaws.com.
* `ec2:DescribeInstanceStatus` and `ec2:DescribeInstances` – Allows Lambda to describe EC2 instances.
For more information about this policy, see [Using service-linked roles for Lambda](./using-service-linked-roles.html).
## Lambda updates to AWS managed
policies
|Change|Description|Date|
|
[AWSLambdaManagedEC2ResourceOperator](https://docs.aws.amazon.com/aws-managed-policy/latest/reference/AWSLambdaManagedEC2ResourceOperator.html) – New policy
|
Lambda added a new managed policy to enable automated Amazon EC2 instance management for Lambda capacity providers, allowing the scaler service to perform instance lifecycle operations.
|November 30, 2025|
|
[AWSLambdaServiceRolePolicy](https://docs.aws.amazon.com/aws-managed-policy/latest/reference/AWSLambdaServiceRolePolicy.html) – New policy
|
Lambda added a new managed policy for the service-linked role to allow Lambda to terminate instances managed as part of Lambda capacity providers.
|November 30, 2025|
|
[AWSLambda\_FullAccess](https://docs.aws.amazon.com/aws-managed-policy/latest/reference/AWSLambda_FullAccess.html) – Change
|
Lambda updated the `AWSLambda\_FullAccess` policy to allow the `kms:DescribeKey` and `iam:CreateServiceLinkedRole` actions.
|November 30, 2025|
|
[AWSLambdaBasicDurableExecutionRolePolicy](https://docs.aws.amazon.com/aws-managed-policy/latest/reference/AWSLambdaBasicDurableExecutionRolePolicy.html) – New managed policy
|
Lambda released a new managed policy `AWSLambdaBasicDurableExecutionRolePolicy` that provides write permissions to CloudWatch Logs and read/write permissions to durable execution APIs used by Lambda durable functions.
|December 1, 2025|
|
[AWSLambda\_ReadOnlyAccess](https://docs.aws.amazon.com/aws-managed-policy/latest/reference/AWSLambda_ReadOnlyAccess.html) and [AWSLambda\_FullAccess](https://docs.aws.amazon.com/aws-managed-policy/latest/reference/AWSLambda_FullAccess.html) – Change
|
Lambda updated the `AWSLambda\_ReadOnlyAccess` and `AWSLambda\_FullAccess` policies to allow the `logs:StartLiveTail` and `logs:StopLiveTail` actions.
|March 17, 2025|
|
[AWSLambdaVPCAccessExecutionRole](https://docs.aws.amazon.com/aws-managed-policy/latest/reference/AWSLambdaVPCAccessExecutionRole.html) – Change
|
Lambda updated the `AWSLambdaVPCAccessExecutionRole` policy to allow the action `ec2:DescribeSubnets`.
|January 5, 2024|
|
[AWSLambda\_ReadOnlyAccess](https://docs.aws.amazon.com/aws-managed-policy/latest/reference/AWSLambda_ReadOnlyAccess.html) – Change
|
Lambda updated the `AWSLambda\_ReadOnlyAccess` policy to allow principals to list CloudFormation stacks.
|July 27, 2023|
|
AWS Lambda started tracking changes
|
AWS Lambda started tracking changes for its AWS managed policies.
|July 27, 2023|
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Identity-based policy examples
Troubleshooting
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.