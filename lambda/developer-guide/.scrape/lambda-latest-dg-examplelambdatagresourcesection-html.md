---
url: https://docs.aws.amazon.com/lambda/latest/dg/example_lambda_TagResource_section.html
title: Use `TagResource` with a CLI
word_count: 335
filtered: true
elements_removed: 0
density_score: 0.85
---

Use TagResource with a CLI - AWS Lambda
Use TagResource with a CLI - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#example_lambda_TagResource_section)
# Use `TagResource` with a CLI
The following code examples show how to use `TagResource`.
CLI
**AWS CLI**
**To add tags to an existing Lambda function**
The following `tag-resource` example adds a tag with the key name `DEPARTMENT` and a value of `Department A` to the specified Lambda function.
```
``aws lambda tag-resource \\
--resource `arn:aws:lambda:us-west-2:123456789012:function:my-function` \\
--tags `"DEPARTMENT=Department A"``
`
```
This command produces no output.
For more information, see [Tagging Lambda Functions](https://docs.aws.amazon.com/lambda/latest/dg/tagging.html) in the *AWS Lambda Developer Guide*.
*
For API details, see
[TagResource](https://awscli.amazonaws.com/v2/documentation/api/latest/reference/lambda/tag-resource.html)
in *AWS CLI Command Reference*.
PowerShell
**Tools for PowerShell V4**
**Example 1: Adds the three tags (Washington, Oregon and California) and their associated values to the specified function identified by its ARN.**
```
`Add-LMResourceTag -Resource "arn:aws:lambda:us-west-2:123456789012:function:MyFunction" -Tag @{ "Washington" = "Olympia"; "Oregon" = "Salem"; "California" = "Sacramento" }
`
```
*
For API details, see
[TagResource](https://docs.aws.amazon.com/powershell/v4/reference)
in *AWS Tools for PowerShell Cmdlet Reference (V4)*.
**Tools for PowerShell V5**
**Example 1: Adds the three tags (Washington, Oregon and California) and their associated values to the specified function identified by its ARN.**
```
`Add-LMResourceTag -Resource "arn:aws:lambda:us-west-2:123456789012:function:MyFunction" -Tag @{ "Washington" = "Olympia"; "Oregon" = "Salem"; "California" = "Sacramento" }
`
```
*
For API details, see
[TagResource](https://docs.aws.amazon.com/powershell/v5/reference)
in *AWS Tools for PowerShell Cmdlet Reference (V5)*.
For a complete list of AWS SDK developer guides and code examples, see
[Using Lambda with an AWS SDK](./sdk-general-information-section.html).
This topic also includes information about getting started and details about previous SDK versions.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
RemovePermission
UntagResource
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.