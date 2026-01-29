---
url: https://docs.aws.amazon.com/lambda/latest/dg/permissions-function-cross-account.html
title: Granting Lambda function access to other accounts
word_count: 438
filtered: true
elements_removed: 0
density_score: 0.87
---

Granting Lambda function access to other accounts - AWS Lambda
Granting Lambda function access to other accounts - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#permissions-function-cross-account)
# Granting Lambda function access to other accounts
To share a function with another AWS account, add a cross-account permissions statement to the function's [resource-based policy](./access-control-resource-based.html). Run the [add-permission](https://awscli.amazonaws.com/v2/documentation/api/latest/reference/lambda/add-permission.html) command and specify the account ID as the `principal`. The following example grants account `111122223333` permission to invoke `my-function` with the
`prod` alias.
```
``aws lambda add-permission \\
--function-name my-function:`prod` \\
--statement-id xaccount \\
--action lambda:InvokeFunction \\
--principal `111122223333` \\
--output text``
```
You should see the following output:
```
{"Sid":"xaccount","Effect":"Allow","Principal":{"AWS":"arn:aws:iam::111122223333:root"},"Action":"lambda:InvokeFunction","Resource":"arn:aws:lambda:us-east-1:123456789012:function:my-function"}
```
The resource-based policy grants permission for the other account to access the function, but doesn't allow
users in that account to exceed their permissions. Users in the other account must have the corresponding [user permissions](./access-control-identity-based.html) to use the Lambda API.
To limit access to a user or role in another account, specify the full ARN of the identity as the principal.
For example, `arn:aws:iam::123456789012:user/developer`.
The [alias](./configuration-aliases.html) limits which version the other account can invoke. It
requires the other account to include the alias in the function ARN.
```
``aws lambda invoke \\
--function-name arn:aws:lambda:us-east-2:123456789012:function:my-function:prod out``
```
You should see the following output:
```
{
"StatusCode": 200,
"ExecutedVersion": "1"
}
```
The function owner can then update the alias to point to a new version without the caller needing to change
the way they invoke your function. This ensures that the other account doesn't need to change its code to use the
new version, and it only has permission to invoke the version of the function associated with the alias.
You can grant cross-account access for most API actions that operate on an existing function. For example, you could grant access to `lambda:ListAliases`
to let an account get a list of aliases, or `lambda:GetFunction` to let them download your function
code. Add each permission separately, or use `lambda:\*` to grant access to all actions for the
specified function.
To grant other accounts permission for multiple functions, or for actions that don't operate on a function, we
recommend that you use [IAM roles](./access-control-identity-based.html).
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Function access for AWS Organizations
Layer access for other accounts
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.