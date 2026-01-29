---
url: https://docs.aws.amazon.com/lambda/latest/dg/lambda-managed-instances-operator-role.html
title: Lambda operator role for Lambda Managed Instances
word_count: 510
filtered: true
elements_removed: 0
density_score: 0.82
---

Lambda operator role for Lambda Managed Instances - AWS Lambda
Lambda operator role for Lambda Managed Instances - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#lambda-managed-instances-operator-role)
[Creating an operator role](#lambda-managed-instances-creating-operator-role)[Service-Linked Role for Lambda Managed Instances](#lambda-managed-instances-service-linked-role-for-lmi)
# Lambda operator role for Lambda Managed Instances
When you use Lambda Managed Instances, Lambda needs permissions to manage compute capacity in your account. The operator role provides these permissions through IAM policies that allow Lambda to manage EC2 instances in the capacity provider.
Lambda assumes the operator role when performing these management operations, similar to how Lambda assumes an execution role when your function runs.
## Creating an operator role
You can create an operator role in the IAM console or with the AWS CLI. The role must include:
* **Permissions policy** – Grants permissions to manage capacity providers and associated resources
* **Trust policy** – Allows the Lambda service (`lambda.amazonaws.com`) to assume the role
### Permissions policy
The operator role needs permissions to manage capacity providers and the underlying compute resources. At minimum, the role requires the permissions in the [AWSLambdaManagedEC2ResourceOperator](https://us-east-1.console.aws.amazon.com/iam/home?region=us-east-1#/policies/details/arn:aws:iam::aws:policy/AWSLambdaManagedEC2ResourceOperator) managed policy, currently:
```
`{
"Version": "2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Action": [
"ec2:RunInstances",
"ec2:CreateTags",
"ec2:AttachNetworkInterface"
],
"Resource": [
"arn:aws:ec2:\*:\*:instance/\*",
"arn:aws:ec2:\*:\*:network-interface/\*",
"arn:aws:ec2:\*:\*:volume/\*"
],
"Condition": {
"StringEquals": {
"ec2:ManagedResourceOperator": "scaler.lambda.amazonaws.com"
}
}
},
{
"Effect": "Allow",
"Action": [
"ec2:DescribeAvailabilityZones",
"ec2:DescribeCapacityReservations",
"ec2:DescribeInstances",
"ec2:DescribeInstanceStatus",
"ec2:DescribeInstanceTypeOfferings",
"ec2:DescribeInstanceTypes",
"ec2:DescribeSecurityGroups",
"ec2:DescribeSubnets"
],
"Resource": "\*"
},
{
"Effect": "Allow",
"Action": [
"ec2:RunInstances",
"ec2:CreateNetworkInterface"
],
"Resource": [
"arn:aws:ec2:\*:\*:subnet/\*",
"arn:aws:ec2:\*:\*:security-group/\*"
]
},
{
"Effect": "Allow",
"Action": [
"ec2:RunInstances"
],
"Resource": [
"arn:aws:ec2:\*:\*:image/\*"
],
"Condition": {
"StringEquals": {
"ec2:Owner": "amazon"
}
}
}
]
}`
```
### Trust policy
The trust policy allows Lambda to assume the operator role:
```
`{
"Version": "2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Principal": {
"Service": "lambda.amazonaws.com"
},
"Action": "sts:AssumeRole"
}
]
}`
```
## Service-Linked Role for Lambda Managed Instances
To responsibly manage the lifecycle of Lambda Managed Instances, Lambda requires persistent access to terminate managed instances in your account. Lambda uses an AWS Identity and Access Management (IAM) service-linked role (SLR) to perform these operations.
**Automatic creation**: The service-linked role is automatically created the first time you create a capacity provider. The user creating the first capacity provider must have the `iam:CreateServiceLinkedRole` permission for the `lambda.amazonaws.com` principal.
**Permissions**: The service-linked role grants Lambda the following permissions on managed instances:
* `ec2:TerminateInstances` – To terminate instances at the end of their lifecycle
* `ec2:DescribeInstances` – To enumerate managed instances
**Deletion**: You can only delete this service-linked role after you have deleted all Lambda Managed Instances capacity providers in your account.
For more information about service-linked roles, see [Using service-linked roles for
Lambda](./using-service-linked-roles.html).
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Security
Execution environment
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.