---
url: https://docs.aws.amazon.com/lambda/latest/dg/using-service-linked-roles.html
title: Using service-linked roles for
word_count: 889
filtered: true
elements_removed: 0
density_score: 0.89
---

Using service-linked roles for Lambda - AWS Lambda
Using service-linked roles for Lambda - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#using-service-linked-roles)
[Service-linked role permissions for Lambda](#slr-permissions)[Creating a service-linked role for Lambda](#create-slr)[Editing a service-linked role for Lambda](#edit-slr)[Deleting a service-linked role for Lambda](#delete-slr)[Supported Regions for Lambda service-linked roles](#slr-regions)
# Using service-linked roles for
Lambda
Lambda uses AWS Identity and Access Management (IAM) [service-linked roles](https://docs.aws.amazon.com/IAM/latest/UserGuide/id_roles_terms-and-concepts.html#iam-term-service-linked-role). A service-linked role is a unique type of IAM role that is
linked directly to Lambda. Service-linked roles are predefined by Lambda and
include permissions that the service requires to call other AWS services on your behalf.
Lambda defines the permissions of its service-linked roles, and only Lambda
can assume its roles. The defined permissions include the trust policy and the permissions
policy, and that permissions policy cannot be attached to any other IAM entity.
You can delete a service-linked role only after first deleting their related resources. This
protects your Lambda resources because you can't inadvertently remove permission to
access the resources.
For information about other services that support service-linked roles, see [AWS services that work with IAM](https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_aws-services-that-work-with-iam.html) and look for the services that have **Yes** in the **Service-linked roles** column.
Choose a **Yes** with a link to view the service-linked role
documentation for that service.
## Service-linked role permissions for Lambda
Lambda uses the service-linked role named **AWSServiceRoleForLambda**. The
service-linked role trusts the following services to assume the role:
* `lambda.amazonaws.com`
The role permissions policy named AWSLambdaServiceRolePolicy allows Lambda to complete the
following actions on the specified resources:
* Action: `ec2:TerminateInstances` on
`arn:aws:ec2:\*:\*:instance/\*` with the condition that
`ec2:ManagedResourceOperator` equals `scaler.lambda.amazonaws.com`
* Action: `ec2:DescribeInstanceStatus` and `ec2:DescribeInstances`
on `\*`
You must configure permissions to allow your users, groups, or roles to create, edit, or
delete a service-linked role. For more information, see [Service-linked role permissions](https://docs.aws.amazon.com/IAM/latest/UserGuide/using-service-linked-roles.html#service-linked-role-permissions) in the
*IAM User Guide*.
For managed policy updates, see [Lambda managed policies](./security-iam-awsmanpol.html#lambda-security-iam-awsmanpol-updates).
## Creating a service-linked role for Lambda
You don't need to manually create a service-linked role. When you
create a Lambda capacity provider in the AWS Management Console, the AWS CLI, or the AWS API, Lambda creates
the service-linked role for you.
If you delete this service-linked role, and then need to create it again, you can use the
same process to recreate the role in your account. When you create a Lambda capacity provider,
Lambda creates the service-linked role for you again.
You can also use the IAM console to create a service-linked role with the
**AWSServiceRoleForLambda** use case. In the AWS CLI or the AWS API, create a
service-linked role with the `lambda.amazonaws.com` service name. For more
information, see [Creating a service-linked role](https://docs.aws.amazon.com/IAM/latest/UserGuide/using-service-linked-roles.html#create-service-linked-role) in the *IAM User Guide*. If you
delete this service-linked role, you can use this same process to create the role
again.
## Editing a service-linked role for Lambda
Lambda does not allow you to edit the AWSServiceRoleForLambda service-linked role. After you
create a service-linked role, you cannot change the name of the role because various entities
might reference the role. However, you can edit the description of the role using IAM. For
more information, see [Editing
a service-linked role](https://docs.aws.amazon.com/IAM/latest/UserGuide/using-service-linked-roles.html#edit-service-linked-role) in the *IAM User Guide*.
## Deleting a service-linked role for Lambda
If you no longer need to use a feature or service that requires a service-linked role, we
recommend that you delete that role. That way you don’t have an unused entity that is not
actively monitored or maintained. However, you must clean up the resources for your
service-linked role before you can manually delete it.
###### Note
If the Lambda service is using the role when you try to delete the resources,
then the deletion might fail. If that happens, wait for a few minutes and try the operation
again.
###### To delete Lambda resources used by the AWSServiceRoleForLambda
1. Remove all Lambda capacity providers from your account. You can do this using the Lambda console, CLI, or API.
2. Verify that no Lambda capacity providers remain in your account before attempting to delete the service-linked role.
**To manually delete the service-linked role using
IAM**
Use the IAM console, the AWS CLI, or the AWS API to delete the AWSServiceRoleForLambda service-linked
role. For more information, see [Deleting a service-linked role](https://docs.aws.amazon.com/IAM/latest/UserGuide/using-service-linked-roles.html#delete-service-linked-role) in the *IAM User Guide*.
## Supported Regions for Lambda service-linked roles
Lambda does not support using service-linked roles in every Region where the
service is available. AWSServiceRoleForLambda is supported in the following Regions.
|Region name|Region identity|Support in Lambda|
|US East (N. Virginia)|us-east-1|Yes|
|US East (Ohio)|us-east-2|Yes|
|US West (N. California)|us-west-1|Yes|
|US West (Oregon)|us-west-2|Yes|
|Africa (Cape Town)|af-south-1|No|
|Asia Pacific (Hong Kong)|ap-east-1|Yes|
|Asia Pacific (Jakarta)|ap-southeast-3|Yes|
|Asia Pacific (Bangkok)|ap-southeast-7|Yes|
|Asia Pacific (Mumbai)|ap-south-1|Yes|
|Asia Pacific (Osaka)|ap-northeast-3|No|
|Asia Pacific (Seoul)|ap-northeast-2|No|
|Asia Pacific (Singapore)|ap-southeast-1|Yes|
|Asia Pacific (Sydney)|ap-southeast-2|Yes|
|Asia Pacific (Tokyo)|ap-northeast-1|Yes|
|Canada (Central)|ca-central-1|No|
|Europe (Frankfurt)|eu-central-1|Yes|
|Europe (Ireland)|eu-west-1|Yes|
|Europe (London)|eu-west-2|Yes|
|Europe (Milan)|eu-south-1|No|
|Europe (Paris)|eu-west-3|No|
|Europe (Stockholm)|eu-north-1|No|
|Middle East (Bahrain)|me-south-1|No|
|Middle East (UAE)|me-central-1|No|
|South America (São Paulo)|sa-east-1|No|
|AWS GovCloud (US-East)|us-gov-east-1|No|
|AWS GovCloud (US-West)|us-gov-west-1|No|
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Encryption at rest
Identity and Access Management
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.