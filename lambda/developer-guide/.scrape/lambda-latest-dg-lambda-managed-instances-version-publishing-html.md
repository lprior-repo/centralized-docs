---
url: https://docs.aws.amazon.com/lambda/latest/dg/lambda-managed-instances-version-publishing.html
title: $LATEST.PUBLISHED version in Lambda Managed Instances
word_count: 252
filtered: true
elements_removed: 0
density_score: 0.91
---

$LATEST.PUBLISHED version in Lambda Managed Instances - AWS Lambda
$LATEST.PUBLISHED version in Lambda Managed Instances - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#lambda-managed-instances-version-publishing)
# $LATEST.PUBLISHED version in Lambda Managed Instances
Lambda Managed Instances functions support the same numbered versioning workflow as Lambda (default). If you prefer not to maintain numbered versions, Lambda Managed Instances introduces a new version type: `$LATEST.PUBLISHED`. This version allows you to create or republish a latest published version as needed with updated code or configuration, without managing numbered versions.
**Key difference from $LATEST:** When you invoke a Lambda Managed Instances function using an unqualified ARN, Lambda implicitly invokes the `$LATEST.PUBLISHED` version rather than the unpublished $LATEST version.
The following AWS CLI command creates or republishes the `$LATEST.PUBLISHED` version.
```
`aws lambda publish-version --function-name my-function --publish-to LATEST\_PUBLISHED`
```
You should see the following output:
```
`{
"FunctionName": "my-function",
"FunctionArn": "arn:aws:lambda:us-east-2:123456789012:function:my-function:$LATEST.PUBLISHED",
"Version": "$LATEST.PUBLISHED",
"Role": "arn:aws:iam::123456789012:role/lambda-role",
"Handler": "function.handler",
"Runtime": "nodejs24.x",
...
}`
```
###### Note
If you use AWS CloudFormation or the Lambda console to create a Lambda Managed Instances function, Lambda automatically creates the `$LATEST.PUBLISHED` version.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Execution environment
Runtimes
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.