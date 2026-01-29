---
url: https://docs.aws.amazon.com/step-functions/latest/dg/security-iam-awsmanpol.html
title: AWS managed policies for AWS Step Functions
word_count: 581
filtered: true
elements_removed: 0
density_score: 0.88
---

AWS managed policies for AWS Step Functions - AWS Step Functions
AWS managed policies for AWS Step Functions - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/dg/step-functions-dg.pdf#security-iam-awsmanpol)
[AWSStepFunctionsConsoleFullAccess](#security-iam-awsmanpol-AWSStepFunctionsConsoleFullAccess)[AWSStepFunctionsReadOnlyAccess](#security-iam-awsmanpol-AWSStepFunctionsReadOnlyAccess)[AWSStepFunctionsFullAccess](#security-iam-awsmanpol-AWSStepFunctionsFullAccess)[Policy updates](#security-iam-awsmanpol-updates)
# AWS managed policies for AWS Step Functions
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
## AWS managed policy: AWSStepFunctionsConsoleFullAccess
You can attach the
[`AWSStepFunctionsConsoleFullAccess`](https://docs.aws.amazon.com/aws-managed-policy/latest/reference/AWSStepFunctionsConsoleFullAccess.html) policy to your IAM identities.
This policy grants `administrator`permissions that
allow a user access to use the Step Functions console. For a full console experience, a user may also need iam:PassRole permission on other IAM roles that can be assumed by the service.
## AWS managed policy: AWSStepFunctionsReadOnlyAccess
You can attach the [`AWSStepFunctionsReadOnlyAccess`](https://docs.aws.amazon.com/aws-managed-policy/latest/reference/AWSStepFunctionsReadOnlyAccess.html) policy to your IAM identities.
This policy grants `read-only` permissions that allow a user or role to list and describe state machines, activities, executions, activities, tags, MapRuns, and state machine alias and versions. This policy also grants permission to check the syntax of state machine definitions that you provide.
## AWS managed policy: AWSStepFunctionsFullAccess
You can attach the
[`AWSStepFunctionsFullAccess`](https://docs.aws.amazon.com/aws-managed-policy/latest/reference/AWSStepFunctionsFullAccess.html) policy to your IAM identities.
This policy grants `full` permissions to a user or role to use the Step Functions API. For full access, a user must have `iam:PassRole` permission on at least one IAM role that can be assumed by the service.
## Step Functions updates to AWS managed
policies
View details about updates to AWS managed policies for Step Functions since this service
began tracking these changes. For automatic alerts about changes to this page, subscribe to
the RSS feed on the Step Functions [Document history](./document-history.html) page.
|Change|Description|Date|
|
[AWSStepFunctionsReadOnlyAccess](#security-iam-awsmanpol-AWSStepFunctionsReadOnlyAccess) –
Update to an existing policy
|
Step Functions added new permissions to allow calling `states:ValidateStateMachineDefinition` API action to check the syntax of state machine definitions that you provide.
|April 25, 2024|
|
[AWSStepFunctionsReadOnlyAccess](https://docs.aws.amazon.com/aws-managed-policy/latest/reference/AWSStepFunctionsReadOnlyAccess.html) –
Update to an existing policy
|
Step Functions added new permissions to allow listing and reading data related to: Tags (ListTagsForResource), Distributed Map (ListMapRuns, DescribeMapRun), Versions and Aliases (DescribeStateMachineAlias, ListStateMachineAliases, ListStateMachineVersions).
|April 02, 2024|
|
Step Functions started tracking changes
|
Step Functions started tracking changes for its AWS managed policies.
|April 02, 2024|
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Identity-based policy examples
Creating a state machine IAM role
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.