---
url: https://docs.aws.amazon.com/step-functions/latest/dg/security_iam_troubleshoot.html
title: Troubleshooting identity and
word_count: 709
filtered: true
elements_removed: 0
density_score: 0.82
---

Troubleshooting identity and access issues in Step Functions - AWS Step Functions
Troubleshooting identity and access issues in Step Functions - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/dg/step-functions-dg.pdf#security_iam_troubleshoot)
[I am not authorized to
perform an action in Step Functions](#security_iam_troubleshoot-no-permissions)[I am not authorized to perform
iam:PassRole](#security_iam_troubleshoot-passrole)[I want to allow
people outside of my AWS account to access my Step Functions resources](#security_iam_troubleshoot-cross-account-access)
# Troubleshooting identity and
access issues in Step Functions
Use the following information to help you diagnose and fix common issues that you might
encounter when working with Step Functions and IAM.
###### Topics
* [I am not authorized to
perform an action in Step Functions](#security_iam_troubleshoot-no-permissions)
* [I am not authorized to perform
iam:PassRole](#security_iam_troubleshoot-passrole)
* [I want to allow
people outside of my AWS account to access my Step Functions resources](#security_iam_troubleshoot-cross-account-access)
## I am not authorized to
perform an action in Step Functions
If you receive an error that you're not authorized to perform an action, your
policies must be updated to allow you to perform the action.
The following example error occurs when the `mateojackson` user tries
to use the console to view details about a fictional
``my-example-widget`` resource but does
not have the fictional
`states:`GetWidget``
permissions.
```
User: arn:aws:iam::123456789012:user/mateojackson is not authorized to perform: states:`GetWidget` on resource: `my-example-widget`
```
In this case, Mateo's policy must be updated to allow him to access the
``my-example-widget`` resource using the
`states:`GetWidget``
action.
If you need help, contact your AWS administrator. Your administrator is the
person who provided you with your sign-in credentials.
## I am not authorized to perform
iam:PassRole
If you receive an error that you're not authorized to perform the `iam:PassRole` action, your policies must be updated to allow you to pass a role to Step Functions.
Some AWS services allow you to pass an existing role to that service instead of creating a new service role or service-linked role. To do
this, you must have permissions to pass the role to the service.
The following example error occurs when an IAM user named `marymajor` tries to use the console to perform an action in
Step Functions. However, the action requires the service to have permissions that are granted by a service role. Mary does not have permissions to pass the
role to the service.
```
`User: arn:aws:iam::123456789012:user/`marymajor` is not authorized to perform: iam:PassRole`
```
In this case, Mary's policies must be updated to allow her to perform the `iam:PassRole` action.
If you need help, contact your AWS administrator. Your administrator is the person who provided you with your sign-in credentials.
## I want to allow
people outside of my AWS account to access my Step Functions resources
You can create a role that users in other accounts or people outside of your organization can use to access your resources. You can specify who
is trusted to assume the role. For services that support resource-based policies or access control lists (ACLs), you can use those policies to grant
people access to your resources.
To learn more, consult the following:
* To learn whether Step Functions supports these features, see [How AWS Step Functions works with IAM](./security_iam_service-with-iam.html).
* To learn how to provide access to your resources across AWS accounts that you own, see [Providing access to an IAM user in another AWS account that you
own](https://docs.aws.amazon.com/IAM/latest/UserGuide/id_roles_common-scenarios_aws-accounts.html) in the *IAM User Guide*.
* To learn how to provide access to your resources to third-party AWS accounts, see [Providing access to AWS accounts owned by third parties](https://docs.aws.amazon.com/IAM/latest/UserGuide/id_roles_common-scenarios_third-party.html) in the
*IAM User Guide*.
* To learn how to provide access through identity federation, see [Providing access to externally authenticated users (identity federation)](https://docs.aws.amazon.com/IAM/latest/UserGuide/id_roles_common-scenarios_federated-users.html) in the *IAM User Guide*.
* To learn the difference between using roles and resource-based policies for cross-account access, see [Cross account resource access in IAM](https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies-cross-account-resource-access.html) in the
*IAM User Guide*.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Creating tag-based policies
Logging and monitoring
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.