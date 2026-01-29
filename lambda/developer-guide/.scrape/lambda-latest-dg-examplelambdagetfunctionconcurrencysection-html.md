---
url: https://docs.aws.amazon.com/lambda/latest/dg/example_lambda_GetFunctionConcurrency_section.html
title: Use `GetFunctionConcurrency` with a CLI
word_count: 283
filtered: true
elements_removed: 0
density_score: 0.94
---

Use GetFunctionConcurrency with a CLI - AWS Lambda
Use GetFunctionConcurrency with a CLI - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#example_lambda_GetFunctionConcurrency_section)
# Use `GetFunctionConcurrency` with a CLI
The following code examples show how to use `GetFunctionConcurrency`.
CLI
**AWS CLI**
**To view the reserved concurrency setting for a function**
The following `get-function-concurrency` example retrieves the reserved concurrency setting for the specified function.
```
``aws lambda get-function-concurrency \\
--function-name `my-function``
`
```
Output:
```
`{
"ReservedConcurrentExecutions": 250
}`
```
*
For API details, see
[GetFunctionConcurrency](https://awscli.amazonaws.com/v2/documentation/api/latest/reference/lambda/get-function-concurrency.html)
in *AWS CLI Command Reference*.
PowerShell
**Tools for PowerShell V4**
**Example 1: This examples gets the Reserved concurrency for the Lambda Function**
```
`Get-LMFunctionConcurrency -FunctionName "MylambdaFunction123" -Select \*
`
```
**Output:**
```
`ReservedConcurrentExecutions
----------------------------
100`
```
*
For API details, see
[GetFunctionConcurrency](https://docs.aws.amazon.com/powershell/v4/reference)
in *AWS Tools for PowerShell Cmdlet Reference (V4)*.
**Tools for PowerShell V5**
**Example 1: This examples gets the Reserved concurrency for the Lambda Function**
```
`Get-LMFunctionConcurrency -FunctionName "MylambdaFunction123" -Select \*
`
```
**Output:**
```
`ReservedConcurrentExecutions
----------------------------
100`
```
*
For API details, see
[GetFunctionConcurrency](https://docs.aws.amazon.com/powershell/v5/reference)
in *AWS Tools for PowerShell Cmdlet Reference (V5)*.
For a complete list of AWS SDK developer guides and code examples, see
[Using Lambda with an AWS SDK](./sdk-general-information-section.html).
This topic also includes information about getting started and details about previous SDK versions.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
GetFunction
GetFunctionConfiguration
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.