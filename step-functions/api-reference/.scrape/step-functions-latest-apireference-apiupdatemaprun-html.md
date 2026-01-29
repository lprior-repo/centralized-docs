---
url: https://docs.aws.amazon.com/step-functions/latest/apireference/API_UpdateMapRun.html
title: UpdateMapRun
word_count: 293
filtered: true
elements_removed: 0
density_score: 0.88
---

UpdateMapRun - AWS Step Functions
UpdateMapRun - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/apireference/step-functions-api.pdf#API_UpdateMapRun)
[Request Syntax](#API_UpdateMapRun_RequestSyntax)[Request Parameters](#API_UpdateMapRun_RequestParameters)[Response Elements](#API_UpdateMapRun_ResponseElements)[Errors](#API_UpdateMapRun_Errors)[See Also](#API_UpdateMapRun_SeeAlso)
# UpdateMapRun
Updates an in-progress Map Run's configuration to include changes to the settings that control maximum concurrency and Map Run failure.
## Request Syntax
```
`{
"[mapRunArn](#StepFunctions-UpdateMapRun-request-mapRunArn)": "`string`",
"[maxConcurrency](#StepFunctions-UpdateMapRun-request-maxConcurrency)": `number`,
"[toleratedFailureCount](#StepFunctions-UpdateMapRun-request-toleratedFailureCount)": `number`,
"[toleratedFailurePercentage](#StepFunctions-UpdateMapRun-request-toleratedFailurePercentage)": `number`
}`
```
## Request Parameters
For information about the parameters that are common to all actions, see [Common Parameters](./CommonParameters.html).
The request accepts the following data in JSON format.
**
[mapRunArn](#API_UpdateMapRun_RequestSyntax)
**
The Amazon Resource Name (ARN) of a Map Run.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 2000.
Required: Yes
**
[maxConcurrency](#API_UpdateMapRun_RequestSyntax)
**
The maximum number of child workflow executions that can be specified to run in parallel for the Map Run at the same time.
Type: Integer
Valid Range: Minimum value of 0.
Required: No
**
[toleratedFailureCount](#API_UpdateMapRun_RequestSyntax)
**
The maximum number of failed items before the Map Run fails.
Type: Long
Valid Range: Minimum value of 0.
Required: No
**
[toleratedFailurePercentage](#API_UpdateMapRun_RequestSyntax)
**
The maximum percentage of failed items before the Map Run fails.
Type: Float
Valid Range: Minimum value of 0. Maximum value of 100.
Required: No
## Response Elements
If the action is successful, the service sends back an HTTP 200 response with an empty HTTP body.
## Errors
For information about the errors that are common to all actions, see [Common Errors](./CommonErrors.html).
**
InvalidArn
**
The provided Amazon Resource Name (ARN) is not valid.
HTTP Status Code: 400
**
ResourceNotFound
**
Could not find the referenced resource.
HTTP Status Code: 400
**
ValidationException
**
The input does not satisfy the constraints specified by an AWS service.
**
reason
**
The input does not satisfy the constraints specified by an AWS service.
HTTP Status Code: 400