---
url: https://docs.aws.amazon.com/lambda/latest/dg/permissions-granting-access.html
title: Granting other AWS entities access to your Lambda functions
word_count: 320
filtered: true
elements_removed: 0
density_score: 0.85
---

Granting other AWS entities access to your Lambda functions - AWS Lambda
Granting other AWS entities access to your Lambda functions - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#permissions-granting-access)
# Granting other AWS entities access to your Lambda functions
To give other AWS accounts, organizations, and services permissions to access
your Lambda resources, you have a few options:
* You can use [identity-based policies](./access-control-identity-based.html)
to grant other users access to your Lambda resources. Identity-based policies can
apply to users directly, or to groups and roles that are associated with a user.
* You can use [resource-based policies](./access-control-resource-based.html)
to give other accounts and AWS services permissions to access your Lambda resources.
When a user tries to access a Lambda resource, Lambda considers both the user's
identity-based policies and the resource's resource-based policy. When an AWS service
such as Amazon Simple Storage Service (Amazon S3) calls your Lambda function, Lambda considers only the
resource-based policy.
* You can use an [attribute-based
access control (ABAC)](./attribute-based-access-control.html) model to control access to your Lambda functions. With
ABAC, you can attach tags to a Lambda function, pass them in certain API requests,
or attach them to the IAM principal making the request. Specify the same tags
in the condition element of an IAM policy to control function access.
To help you fine-tune your permissions for least-privilege
access, Lambda provides some additional conditions you can include in your policies.
For more information, see [Fine-tuning the Resources and Conditions sections of policies](./lambda-api-permissions-ref.html).
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Source function ARN
Identity-based policies
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.