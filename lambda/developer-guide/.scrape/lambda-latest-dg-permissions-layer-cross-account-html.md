---
url: https://docs.aws.amazon.com/lambda/latest/dg/permissions-layer-cross-account.html
title: Granting Lambda layer access to other accounts
word_count: 308
filtered: true
elements_removed: 0
density_score: 0.92
---

Granting Lambda layer access to other accounts - AWS Lambda
Granting Lambda layer access to other accounts - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#permissions-layer-cross-account)
# Granting Lambda layer access to other accounts
To share a layer with another AWS account, add a cross-account permissions statement to the layer's [resource-based policy](./access-control-resource-based.html). Run the [add-layer-version-permission](https://awscli.amazonaws.com/v2/documentation/api/latest/reference/lambda/add-layer-version-permission.html) command and specify the account ID as the `principal`. In each statement, you can grant permission to a
single account, all accounts, or an organization in [AWS Organizations](https://docs.aws.amazon.com/organizations/latest/userguide/orgs_introduction.html).
The following example grants account 111122223333 access to version 2 of the `bash-runtime` layer.
```
``aws lambda add-layer-version-permission \\
--layer-name bash-runtime \\
--version-number 2 \\
--statement-id xaccount \\
--action lambda:GetLayerVersion \\
--principal 111122223333 \\
--output text``
```
You should see output similar to the following:
```
`{"Sid":"xaccount","Effect":"Allow","Principal":{"AWS":"arn:aws:iam::111122223333:root"},"Action":"lambda:GetLayerVersion","Resource":"arn:aws:lambda:us-east-1:123456789012:layer:bash-runtime:2"}`
```
Permissions apply only to a single layer version. Repeat the process each time that you create a new layer
version.
To grant permission to all accounts in an [AWS Organizations](https://docs.aws.amazon.com/organizations/latest/userguide/orgs_introduction.html) organization, use the `organization-id` option. The
following example grants all accounts in organization `o-t194hfs8cz` permission to use version 3 of `my-layer`.
```
``aws lambda add-layer-version-permission \\
--layer-name my-layer \\
--version-number 3 \\
--statement-id engineering-org \\
--principal '\*' \\
--action lambda:GetLayerVersion \\
--organization-id o-t194hfs8cz \\
--output text``
```
You should see the following output:
```
{"Sid":"engineering-org","Effect":"Allow","Principal":"\*","Action":"lambda:GetLayerVersion","Resource":"arn:aws:lambda:us-east-2:123456789012:layer:my-layer:3","Condition":{"StringEquals":{"aws:PrincipalOrgID":"o-t194hfs8cz"}}}"
```
To grant permission to multiple accounts or organizations, you must add multiple statements.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Function access for other accounts
Attribute-based access control
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.