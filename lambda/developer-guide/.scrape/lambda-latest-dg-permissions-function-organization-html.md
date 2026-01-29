---
url: https://docs.aws.amazon.com/lambda/latest/dg/permissions-function-organization.html
title: Granting function access to an organization
word_count: 264
filtered: true
elements_removed: 0
density_score: 0.80
---

Granting function access to an organization - AWS Lambda
Granting function access to an organization - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#permissions-function-organization)
# Granting function access to an organization
To grant permissions to an organization in [AWS Organizations](https://docs.aws.amazon.com/organizations/latest/userguide/orgs_introduction.html), specify the organization ID as the `principal-org-id`.
The following [add-permission](https://awscli.amazonaws.com/v2/documentation/api/latest/reference/lambda/add-permission.html) command grants invocation access to all users in organization `o-a1b2c3d4e5f`.
```
``aws lambda add-permission \\
--function-name example \\
--statement-id PrincipalOrgIDExample \\
--action lambda:InvokeFunction \\
--principal \* \\
--principal-org-id o-a1b2c3d4e5f``
```
###### Note
In this command, `Principal` is `\*`. This means that all users in the organization
`o-a1b2c3d4e5f` get function invocation permissions. If you specify an AWS account or role as the
`Principal`, then only that principal gets function invocation permissions, but only if they are
also part of the `o-a1b2c3d4e5f` organization.
This command creates a resource-based policy that looks like the following:
JSON
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Sid": "PrincipalOrgIDExample",
"Effect": "Allow",
"Principal": "\*",
"Action": "lambda:InvokeFunction",
"Resource": "arn:aws:lambda:us-east-2:123456789012:function:example",
`"Condition": {
"StringEquals": {
"aws:PrincipalOrgID": "o-a1b2c3d4e5f"
}
}`
}
]
}`
`
```
For more information, see [
aws:PrincipalOrgID](https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_condition-keys.html#condition-keys-principalorgid) in the *IAM user guide*.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Function access for AWS services
Function access for other accounts
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.