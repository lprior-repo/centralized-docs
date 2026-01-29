---
url: https://docs.aws.amazon.com/lambda/latest/dg/access-control-identity-based.html
title: Identity-based IAM policies for Lambda
word_count: 301
filtered: true
elements_removed: 0
density_score: 0.85
---

Identity-based IAM policies for Lambda - AWS Lambda
Identity-based IAM policies for Lambda - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#access-control-identity-based)
# Identity-based IAM policies for Lambda
You can use identity-based policies in AWS Identity and Access Management (IAM) to grant users in your account access to Lambda.
Identity-based policies can apply to users, user groups, or roles. You can
also grant users in another account permission to assume a role in your account and access your Lambda
resources.
Lambda provides AWS managed policies that grant access to Lambda API actions and, in some cases, access to other
AWS services used to develop and manage Lambda resources. Lambda updates these managed policies as needed to ensure
that your users have access to new features when they're released.
* [AWSLambda\_FullAccess](https://docs.aws.amazon.com/aws-managed-policy/latest/reference/AWSLambda_FullAccess.html)– Grants full access to Lambda actions and other
AWS services used to develop and maintain Lambda resources.
* [AWSLambda\_ReadOnlyAccess](https://docs.aws.amazon.com/aws-managed-policy/latest/reference/AWSLambda_ReadOnlyAccess.html)– Grants read-only access to Lambda
resources.
* [AWSLambdaRole](https://docs.aws.amazon.com/aws-managed-policy/latest/reference/AWSLambdaRole.html) – Grants permissions to invoke Lambda functions.
AWS managed policies grant permission to API actions without restricting the Lambda functions or layers that a
user can modify. For finer-grained control, you can create your own policies that limit the scope of a user's
permissions.
###### Topics
* [Granting users access to a Lambda function](./permissions-user-function.html)
* [Granting users access to a Lambda layer](./permissions-user-layer.html)
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Access permissions (permissions for other entities to access your functions)
Function access
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.