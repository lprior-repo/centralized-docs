---
url: https://docs.aws.amazon.com/step-functions/latest/apireference/API_DescribeMapRun.html
title: DescribeMapRun
word_count: 582
filtered: true
elements_removed: 0
density_score: 0.93
---

DescribeMapRun - AWS Step Functions
DescribeMapRun - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/apireference/step-functions-api.pdf#API_DescribeMapRun)
[Request Syntax](#API_DescribeMapRun_RequestSyntax)[Request Parameters](#API_DescribeMapRun_RequestParameters)[Response Syntax](#API_DescribeMapRun_ResponseSyntax)[Response Elements](#API_DescribeMapRun_ResponseElements)[Errors](#API_DescribeMapRun_Errors)[See Also](#API_DescribeMapRun_SeeAlso)
# DescribeMapRun
Provides information about a Map Run's configuration, progress, and results. If you've [redriven](https://docs.aws.amazon.com/step-functions/latest/dg/redrive-map-run.html) a Map Run, this API action also returns information about the redrives of that Map Run. For more information, see [Examining Map Run](https://docs.aws.amazon.com/step-functions/latest/dg/concepts-examine-map-run.html) in the *
AWS Step Functions Developer Guide*.
## Request Parameters
For information about the parameters that are common to all actions, see [Common Parameters](./CommonParameters.html).
The request accepts the following data in JSON format.
**
[mapRunArn](#API_DescribeMapRun_RequestSyntax)
**
The Amazon Resource Name (ARN) that identifies a Map Run.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 2000.
Required: Yes
## Response Syntax
```
`{
"[executionArn](#StepFunctions-DescribeMapRun-response-executionArn)": "***string***",
"[executionCounts](#StepFunctions-DescribeMapRun-response-executionCounts)": {
"[aborted](./API_MapRunExecutionCounts.html#StepFunctions-Type-MapRunExecutionCounts-aborted)": ***number***,
"[failed](./API_MapRunExecutionCounts.html#StepFunctions-Type-MapRunExecutionCounts-failed)": ***number***,
"[failuresNotRedrivable](./API_MapRunExecutionCounts.html#StepFunctions-Type-MapRunExecutionCounts-failuresNotRedrivable)": ***number***,
"[pending](./API_MapRunExecutionCounts.html#StepFunctions-Type-MapRunExecutionCounts-pending)": ***number***,
"[pendingRedrive](./API_MapRunExecutionCounts.html#StepFunctions-Type-MapRunExecutionCounts-pendingRedrive)": ***number***,
"[resultsWritten](./API_MapRunExecutionCounts.html#StepFunctions-Type-MapRunExecutionCounts-resultsWritten)": ***number***,
"[running](./API_MapRunExecutionCounts.html#StepFunctions-Type-MapRunExecutionCounts-running)": ***number***,
"[succeeded](./API_MapRunExecutionCounts.html#StepFunctions-Type-MapRunExecutionCounts-succeeded)": ***number***,
"[timedOut](./API_MapRunExecutionCounts.html#StepFunctions-Type-MapRunExecutionCounts-timedOut)": ***number***,
"[total](./API_MapRunExecutionCounts.html#StepFunctions-Type-MapRunExecutionCounts-total)": ***number***
},
"[itemCounts](#StepFunctions-DescribeMapRun-response-itemCounts)": {
"[aborted](./API_MapRunItemCounts.html#StepFunctions-Type-MapRunItemCounts-aborted)": ***number***,
"[failed](./API_MapRunItemCounts.html#StepFunctions-Type-MapRunItemCounts-failed)": ***number***,
"[failuresNotRedrivable](./API_MapRunItemCounts.html#StepFunctions-Type-MapRunItemCounts-failuresNotRedrivable)": ***number***,
"[pending](./API_MapRunItemCounts.html#StepFunctions-Type-MapRunItemCounts-pending)": ***number***,
"[pendingRedrive](./API_MapRunItemCounts.html#StepFunctions-Type-MapRunItemCounts-pendingRedrive)": ***number***,
"[resultsWritten](./API_MapRunItemCounts.html#StepFunctions-Type-MapRunItemCounts-resultsWritten)": ***number***,
"[running](./API_MapRunItemCounts.html#StepFunctions-Type-MapRunItemCounts-running)": ***number***,
"[succeeded](./API_MapRunItemCounts.html#StepFunctions-Type-MapRunItemCounts-succeeded)": ***number***,
"[timedOut](./API_MapRunItemCounts.html#StepFunctions-Type-MapRunItemCounts-timedOut)": ***number***,
"[total](./API_MapRunItemCounts.html#StepFunctions-Type-MapRunItemCounts-total)": ***number***
},
"[mapRunArn](#StepFunctions-DescribeMapRun-response-mapRunArn)": "***string***",
"[maxConcurrency](#StepFunctions-DescribeMapRun-response-maxConcurrency)": ***number***,
"[redriveCount](#StepFunctions-DescribeMapRun-response-redriveCount)": ***number***,
"[redriveDate](#StepFunctions-DescribeMapRun-response-redriveDate)": ***number***,
"[startDate](#StepFunctions-DescribeMapRun-response-startDate)": ***number***,
"[status](#StepFunctions-DescribeMapRun-response-status)": "***string***",
"[stopDate](#StepFunctions-DescribeMapRun-response-stopDate)": ***number***,
"[toleratedFailureCount](#StepFunctions-DescribeMapRun-response-toleratedFailureCount)": ***number***,
"[toleratedFailurePercentage](#StepFunctions-DescribeMapRun-response-toleratedFailurePercentage)": ***number***
}`
```
## Response Elements
If the action is successful, the service sends back an HTTP 200 response.
The following data is returned in JSON format by the service.
**
[executionArn](#API_DescribeMapRun_ResponseSyntax)
**
The Amazon Resource Name (ARN) that identifies the execution in which the Map Run was started.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 256.
**
[executionCounts](#API_DescribeMapRun_ResponseSyntax)
**
A JSON object that contains information about the total number of child workflow executions for the Map Run, and the count of child workflow executions for each status, such as `failed` and `succeeded`.
Type: [MapRunExecutionCounts](./API_MapRunExecutionCounts.html) object
**
[itemCounts](#API_DescribeMapRun_ResponseSyntax)
**
A JSON object that contains information about the total number of items, and the item count for each processing status, such as `pending` and `failed`.
Type: [MapRunItemCounts](./API_MapRunItemCounts.html) object
**
[mapRunArn](#API_DescribeMapRun_ResponseSyntax)
**
The Amazon Resource Name (ARN) that identifies a Map Run.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 2000.
**
[maxConcurrency](#API_DescribeMapRun_ResponseSyntax)
**
The maximum number of child workflow executions configured to run in parallel for the Map Run at the same time.
Type: Integer
Valid Range: Minimum value of 0.
**
[redriveCount](#API_DescribeMapRun_ResponseSyntax)
**
The number of times you've redriven a Map Run. If you have not yet redriven a Map Run, the `redriveCount` is 0. This count is only updated if you successfully redrive a Map Run.
Type: Integer
**
[redriveDate](#API_DescribeMapRun_ResponseSyntax)
**
The date a Map Run was last redriven. If you have not yet redriven a Map Run, the `redriveDate` is null.
Type: Timestamp
**
[startDate](#API_DescribeMapRun_ResponseSyntax)
**
The date when the Map Run was started.
Type: Timestamp
**
[status](#API_DescribeMapRun_ResponseSyntax)
**
The current status of the Map Run.
Type: String
Valid Values: `RUNNING | SUCCEEDED | FAILED | ABORTED`
**
[stopDate](#API_DescribeMapRun_ResponseSyntax)
**
The date when the Map Run was stopped.
Type: Timestamp
**
[toleratedFailureCount](#API_DescribeMapRun_ResponseSyntax)
**
The maximum number of failed child workflow executions before the Map Run fails.
Type: Long
Valid Range: Minimum value of 0.
**
[toleratedFailurePercentage](#API_DescribeMapRun_ResponseSyntax)
**
The maximum percentage of failed child workflow executions before the Map Run fails.
Type: Float
Valid Range: Minimum value of 0. Maximum value of 100.
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