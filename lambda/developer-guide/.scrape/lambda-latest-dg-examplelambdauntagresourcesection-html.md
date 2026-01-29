---
url: https://docs.aws.amazon.com/lambda/latest/dg/example_lambda_UntagResource_section.html
title: Use `UntagResource` with a CLI
word_count: 417
filtered: true
elements_removed: 0
density_score: 0.88
---

Use UntagResource with a CLI - AWS Lambda
Use UntagResource with a CLI - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#example_lambda_UntagResource_section)
# Use `UntagResource` with a CLI
The following code examples show how to use `UntagResource`.
CLI
**AWS CLI**
**To remove tags from an existing Lambda function**
The following `untag-resource` example removes the tag with the key name `DEPARTMENT` tag from the `my-function` Lambda function.
```
``aws lambda untag-resource \\
--resource `arn:aws:lambda:us-west-2:123456789012:function:my-function` \\
--tag-keys `DEPARTMENT``
`
```
This command produces no output.
For more information, see [Tagging Lambda Functions](https://docs.aws.amazon.com/lambda/latest/dg/tagging.html) in the *AWS Lambda Developer Guide*.
*
For API details, see
[UntagResource](https://awscli.amazonaws.com/v2/documentation/api/latest/reference/lambda/untag-resource.html)
in *AWS CLI Command Reference*.
PowerShell
**Tools for PowerShell V4**
**Example 1: Removes the supplied tags from a function. The cmdlet will prompt for confirmation before proceeding unless the -Force switch is specified. A single call is made to the service to remove the tags.**
```
`Remove-LMResourceTag -Resource "arn:aws:lambda:us-west-2:123456789012:function:MyFunction" -TagKey "Washington","Oregon","California"
`
```
**Example 2: Removes the supplied tags from a function. The cmdlet will prompt for confirmation before proceeding unless the -Force switch is specified. Once call to the service is made per supplied tag.**
```
`"Washington","Oregon","California" | Remove-LMResourceTag -Resource "arn:aws:lambda:us-west-2:123456789012:function:MyFunction"
`
```
*
For API details, see
[UntagResource](https://docs.aws.amazon.com/powershell/v4/reference)
in *AWS Tools for PowerShell Cmdlet Reference (V4)*.
**Tools for PowerShell V5**
**Example 1: Removes the supplied tags from a function. The cmdlet will prompt for confirmation before proceeding unless the -Force switch is specified. A single call is made to the service to remove the tags.**
```
`Remove-LMResourceTag -Resource "arn:aws:lambda:us-west-2:123456789012:function:MyFunction" -TagKey "Washington","Oregon","California"
`
```
**Example 2: Removes the supplied tags from a function. The cmdlet will prompt for confirmation before proceeding unless the -Force switch is specified. Once call to the service is made per supplied tag.**
```
`"Washington","Oregon","California" | Remove-LMResourceTag -Resource "arn:aws:lambda:us-west-2:123456789012:function:MyFunction"
`
```
*
For API details, see
[UntagResource](https://docs.aws.amazon.com/powershell/v5/reference)
in *AWS Tools for PowerShell Cmdlet Reference (V5)*.
For a complete list of AWS SDK developer guides and code examples, see
[Using Lambda with an AWS SDK](./sdk-general-information-section.html).
This topic also includes information about getting started and details about previous SDK versions.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
TagResource
UpdateAlias
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.