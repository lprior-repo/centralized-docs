---
url: https://docs.aws.amazon.com/lambda/latest/dg/example_lambda_DeleteFunctionConcurrency_section.html
title: Use `DeleteFunctionConcurrency` with a CLI
word_count: 283
filtered: true
elements_removed: 0
density_score: 0.93
---

Use DeleteFunctionConcurrency with a CLI - AWS Lambda
Use DeleteFunctionConcurrency with a CLI - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#example_lambda_DeleteFunctionConcurrency_section)
# Use `DeleteFunctionConcurrency` with a CLI
The following code examples show how to use `DeleteFunctionConcurrency`.
CLI
**AWS CLI**
**To remove the reserved concurrent execution limit from a function**
The following `delete-function-concurrency` example deletes the reserved concurrent execution limit from the `my-function` function.
```
``aws lambda delete-function-concurrency \\
--function-name `my-function``
`
```
This command produces no output.
For more information, see [Reserving Concurrency for a Lambda Function](https://docs.aws.amazon.com/lambda/latest/dg/per-function-concurrency.html) in the *AWS Lambda Developer Guide*.
*
For API details, see
[DeleteFunctionConcurrency](https://awscli.amazonaws.com/v2/documentation/api/latest/reference/lambda/delete-function-concurrency.html)
in *AWS CLI Command Reference*.
PowerShell
**Tools for PowerShell V4**
**Example 1: This examples removes the Function Concurrency of the Lambda Function.**
```
`Remove-LMFunctionConcurrency -FunctionName "MylambdaFunction123"
`
```
*
For API details, see
[DeleteFunctionConcurrency](https://docs.aws.amazon.com/powershell/v4/reference)
in *AWS Tools for PowerShell Cmdlet Reference (V4)*.
**Tools for PowerShell V5**
**Example 1: This examples removes the Function Concurrency of the Lambda Function.**
```
`Remove-LMFunctionConcurrency -FunctionName "MylambdaFunction123"
`
```
*
For API details, see
[DeleteFunctionConcurrency](https://docs.aws.amazon.com/powershell/v5/reference)
in *AWS Tools for PowerShell Cmdlet Reference (V5)*.
For a complete list of AWS SDK developer guides and code examples, see
[Using Lambda with an AWS SDK](./sdk-general-information-section.html).
This topic also includes information about getting started and details about previous SDK versions.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
DeleteFunction
DeleteProvisionedConcurrencyConfig
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.