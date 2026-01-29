---
url: https://docs.aws.amazon.com/step-functions/latest/apireference/API_GetExecutionHistory.html
title: GetExecutionHistory
word_count: 1149
filtered: true
elements_removed: 0
density_score: 0.88
---

GetExecutionHistory - AWS Step Functions
GetExecutionHistory - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/apireference/step-functions-api.pdf#API_GetExecutionHistory)
[Request Syntax](#API_GetExecutionHistory_RequestSyntax)[Request Parameters](#API_GetExecutionHistory_RequestParameters)[Response Syntax](#API_GetExecutionHistory_ResponseSyntax)[Response Elements](#API_GetExecutionHistory_ResponseElements)[Errors](#API_GetExecutionHistory_Errors)[Examples](#API_GetExecutionHistory_Examples)[See Also](#API_GetExecutionHistory_SeeAlso)
# GetExecutionHistory
Returns the history of the specified execution as a list of events. By default, the
results are returned in ascending order of the `timeStamp` of the events. Use the
`reverseOrder` parameter to get the latest events first.
If `nextToken` is returned, there are more results available. The value of `nextToken` is a unique pagination token for each page.
Make the call again using the returned token to retrieve the next page. Keep all other arguments unchanged. Each pagination token expires after 24 hours. Using an expired pagination token will return an *HTTP 400 InvalidToken* error.
This API action is not supported by `EXPRESS` state machines.
## Request Syntax
```
`{
"[executionArn](#StepFunctions-GetExecutionHistory-request-executionArn)": "`string`",
"[includeExecutionData](#StepFunctions-GetExecutionHistory-request-includeExecutionData)": `boolean`,
"[maxResults](#StepFunctions-GetExecutionHistory-request-maxResults)": `number`,
"[nextToken](#StepFunctions-GetExecutionHistory-request-nextToken)": "`string`",
"[reverseOrder](#StepFunctions-GetExecutionHistory-request-reverseOrder)": `boolean`
}`
```
## Request Parameters
For information about the parameters that are common to all actions, see [Common Parameters](./CommonParameters.html).
The request accepts the following data in JSON format.
**
[executionArn](#API_GetExecutionHistory_RequestSyntax)
**
The Amazon Resource Name (ARN) of the execution.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 256.
Required: Yes
**
[includeExecutionData](#API_GetExecutionHistory_RequestSyntax)
**
You can select whether execution data (input or output of a history event) is returned.
The default is `true`.
Type: Boolean
Required: No
**
[maxResults](#API_GetExecutionHistory_RequestSyntax)
**
The maximum number of results that are returned per call. You can use `nextToken` to obtain further pages of results.
The default is 100 and the maximum allowed page size is 1000. A value of 0 uses the default.
This is only an upper limit. The actual number of results returned per call might be fewer than the specified maximum.
Type: Integer
Valid Range: Minimum value of 0. Maximum value of 1000.
Required: No
**
[nextToken](#API_GetExecutionHistory_RequestSyntax)
**
If `nextToken` is returned, there are more results available. The value of `nextToken` is a unique pagination token for each page.
Make the call again using the returned token to retrieve the next page. Keep all other arguments unchanged. Each pagination token expires after 24 hours. Using an expired pagination token will return an *HTTP 400 InvalidToken* error.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 1024.
Required: No
**
[reverseOrder](#API_GetExecutionHistory_RequestSyntax)
**
Lists events in descending order of their `timeStamp`.
Type: Boolean
Required: No
## Response Syntax
```
`{
"[events](#StepFunctions-GetExecutionHistory-response-events)": [
{
"[activityFailedEventDetails](./API_HistoryEvent.html#StepFunctions-Type-HistoryEvent-activityFailedEventDetails)": {
"[cause](./API_ActivityFailedEventDetails.html#StepFunctions-Type-ActivityFailedEventDetails-cause)": "***string***",
"[error](./API_ActivityFailedEventDetails.html#StepFunctions-Type-ActivityFailedEventDetails-error)": "***string***"
},
"[activityScheduledEventDetails](./API_HistoryEvent.html#StepFunctions-Type-HistoryEvent-activityScheduledEventDetails)": {
"[heartbeatInSeconds](./API_ActivityScheduledEventDetails.html#StepFunctions-Type-ActivityScheduledEventDetails-heartbeatInSeconds)": ***number***,
"[input](./API_ActivityScheduledEventDetails.html#StepFunctions-Type-ActivityScheduledEventDetails-input)": "***string***",
"[inputDetails](./API_ActivityScheduledEventDetails.html#StepFunctions-Type-ActivityScheduledEventDetails-inputDetails)": {
"[truncated](./API_HistoryEventExecutionDataDetails.html#StepFunctions-Type-HistoryEventExecutionDataDetails-truncated)": ***boolean***
},
"[resource](./API_ActivityScheduledEventDetails.html#StepFunctions-Type-ActivityScheduledEventDetails-resource)": "***string***",
"[timeoutInSeconds](./API_ActivityScheduledEventDetails.html#StepFunctions-Type-ActivityScheduledEventDetails-timeoutInSeconds)": ***number***
},
"[activityScheduleFailedEventDetails](./API_HistoryEvent.html#StepFunctions-Type-HistoryEvent-activityScheduleFailedEventDetails)": {
"[cause](./API_ActivityScheduleFailedEventDetails.html#StepFunctions-Type-ActivityScheduleFailedEventDetails-cause)": "***string***",
"[error](./API_ActivityScheduleFailedEventDetails.html#StepFunctions-Type-ActivityScheduleFailedEventDetails-error)": "***string***"
},
"[activityStartedEventDetails](./API_HistoryEvent.html#StepFunctions-Type-HistoryEvent-activityStartedEventDetails)": {
"[workerName](./API_ActivityStartedEventDetails.html#StepFunctions-Type-ActivityStartedEventDetails-workerName)": "***string***"
},
"[activitySucceededEventDetails](./API_HistoryEvent.html#StepFunctions-Type-HistoryEvent-activitySucceededEventDetails)": {
"[output](./API_ActivitySucceededEventDetails.html#StepFunctions-Type-ActivitySucceededEventDetails-output)": "***string***",
"[outputDetails](./API_ActivitySucceededEventDetails.html#StepFunctions-Type-ActivitySucceededEventDetails-outputDetails)": {
"[truncated](./API_HistoryEventExecutionDataDetails.html#StepFunctions-Type-HistoryEventExecutionDataDetails-truncated)": ***boolean***
}
},
"[activityTimedOutEventDetails](./API_HistoryEvent.html#StepFunctions-Type-HistoryEvent-activityTimedOutEventDetails)": {
"[cause](./API_ActivityTimedOutEventDetails.html#StepFunctions-Type-ActivityTimedOutEventDetails-cause)": "***string***",
"[error](./API_ActivityTimedOutEventDetails.html#StepFunctions-Type-ActivityTimedOutEventDetails-error)": "***string***"
},
"[evaluationFailedEventDetails](./API_HistoryEvent.html#StepFunctions-Type-HistoryEvent-evaluationFailedEventDetails)": {
"[cause](./API_EvaluationFailedEventDetails.html#StepFunctions-Type-EvaluationFailedEventDetails-cause)": "***string***",
"[error](./API_EvaluationFailedEventDetails.html#StepFunctions-Type-EvaluationFailedEventDetails-error)": "***string***",
"[location](./API_EvaluationFailedEventDetails.html#StepFunctions-Type-EvaluationFailedEventDetails-location)": "***string***",
"[state](./API_EvaluationFailedEventDetails.html#StepFunctions-Type-EvaluationFailedEventDetails-state)": "***string***"
},
"[executionAbortedEventDetails](./API_HistoryEvent.html#StepFunctions-Type-HistoryEvent-executionAbortedEventDetails)": {
"[cause](./API_ExecutionAbortedEventDetails.html#StepFunctions-Type-ExecutionAbortedEventDetails-cause)": "***string***",
"[error](./API_ExecutionAbortedEventDetails.html#StepFunctions-Type-ExecutionAbortedEventDetails-error)": "***string***"
},
"[executionFailedEventDetails](./API_HistoryEvent.html#StepFunctions-Type-HistoryEvent-executionFailedEventDetails)": {
"[cause](./API_ExecutionFailedEventDetails.html#StepFunctions-Type-ExecutionFailedEventDetails-cause)": "***string***",
"[error](./API_ExecutionFailedEventDetails.html#StepFunctions-Type-ExecutionFailedEventDetails-error)": "***string***"
},
"[executionRedrivenEventDetails](./API_HistoryEvent.html#StepFunctions-Type-HistoryEvent-executionRedrivenEventDetails)": {
"[redriveCount](./API_ExecutionRedrivenEventDetails.html#StepFunctions-Type-ExecutionRedrivenEventDetails-redriveCount)": ***number***
},
"[executionStartedEventDetails](./API_HistoryEvent.html#StepFunctions-Type-HistoryEvent-executionStartedEventDetails)": {
"[input](./API_ExecutionStartedEventDetails.html#StepFunctions-Type-ExecutionStartedEventDetails-input)": "***string***",
"[inputDetails](./API_ExecutionStartedEventDetails.html#StepFunctions-Type-ExecutionStartedEventDetails-inputDetails)": {
"[truncated](./API_HistoryEventExecutionDataDetails.html#StepFunctions-Type-HistoryEventExecutionDataDetails-truncated)": ***boolean***
},
"[roleArn](./API_ExecutionStartedEventDetails.html#StepFunctions-Type-ExecutionStartedEventDetails-roleArn)": "***string***",
"[stateMachineAliasArn](./API_ExecutionStartedEventDetails.html#StepFunctions-Type-ExecutionStartedEventDetails-stateMachineAliasArn)": "***string***",
"[stateMachineVersionArn](./API_ExecutionStartedEventDetails.html#StepFunctions-Type-ExecutionStartedEventDetails-stateMachineVersionArn)": "***string***"
},
"[executionSucceededEventDetails](./API_HistoryEvent.html#StepFunctions-Type-HistoryEvent-executionSucceededEventDetails)": {
"[output](./API_ExecutionSucceededEventDetails.html#StepFunctions-Type-ExecutionSucceededEventDetails-output)": "***string***",
"[outputDetails](./API_ExecutionSucceededEventDetails.html#StepFunctions-Type-ExecutionSucceededEventDetails-outputDetails)": {
"[truncated](./API_HistoryEventExecutionDataDetails.html#StepFunctions-Type-HistoryEventExecutionDataDetails-truncated)": ***boolean***
}
},
"[executionTimedOutEventDetails](./API_HistoryEvent.html#StepFunctions-Type-HistoryEvent-executionTimedOutEventDetails)": {
"[cause](./API_ExecutionTimedOutEventDetails.html#StepFunctions-Type-ExecutionTimedOutEventDetails-cause)": "***string***",
"[error](./API_ExecutionTimedOutEventDetails.html#StepFunctions-Type-ExecutionTimedOutEventDetails-error)": "***string***"
},
"[id](./API_HistoryEvent.html#StepFunctions-Type-HistoryEvent-id)": ***number***,
"[lambdaFunctionFailedEventDetails](./API_HistoryEvent.html#StepFunctions-Type-HistoryEvent-lambdaFunctionFailedEventDetails)": {
"[cause](./API_LambdaFunctionFailedEventDetails.html#StepFunctions-Type-LambdaFunctionFailedEventDetails-cause)": "***string***",
"[error](./API_LambdaFunctionFailedEventDetails.html#StepFunctions-Type-LambdaFunctionFailedEventDetails-error)": "***string***"
},
"[lambdaFunctionScheduledEventDetails](./API_HistoryEvent.html#StepFunctions-Type-HistoryEvent-lambdaFunctionScheduledEventDetails)": {
"[input](./API_LambdaFunctionScheduledEventDetails.html#StepFunctions-Type-LambdaFunctionScheduledEventDetails-input)": "***string***",
"[inputDetails](./API_LambdaFunctionScheduledEventDetails.html#StepFunctions-Type-LambdaFunctionScheduledEventDetails-inputDetails)": {
"[truncated](./API_HistoryEventExecutionDataDetails.html#StepFunctions-Type-HistoryEventExecutionDataDetails-truncated)": ***boolean***
},
"[resource](./API_LambdaFunctionScheduledEventDetails.html#StepFunctions-Type-LambdaFunctionScheduledEventDetails-resource)": "***string***",
"[taskCredentials](./API_LambdaFunctionScheduledEventDetails.html#StepFunctions-Type-LambdaFunctionScheduledEventDetails-taskCredentials)": {
"[roleArn](./API_TaskCredentials.html#StepFunctions-Type-TaskCredentials-roleArn)": "***string***"
},
"[timeoutInSeconds](./API_LambdaFunctionScheduledEventDetails.html#StepFunctions-Type-LambdaFunctionScheduledEventDetails-timeoutInSeconds)": ***number***
},
"[lambdaFunctionScheduleFailedEventDetails](./API_HistoryEvent.html#StepFunctions-Type-HistoryEvent-lambdaFunctionScheduleFailedEventDetails)": {
"[cause](./API_LambdaFunctionScheduleFailedEventDetails.html#StepFunctions-Type-LambdaFunctionScheduleFailedEventDetails-cause)": "***string***",
"[error](./API_LambdaFunctionScheduleFailedEventDetails.html#StepFunctions-Type-LambdaFunctionScheduleFailedEventDetails-error)": "***string***"
},
"[lambdaFunctionStartFailedEventDetails](./API_HistoryEvent.html#StepFunctions-Type-HistoryEvent-lambdaFunctionStartFailedEventDetails)": {
"[cause](./API_LambdaFunctionStartFailedEventDetails.html#StepFunctions-Type-LambdaFunctionStartFailedEventDetails-cause)": "***string***",
"[error](./API_LambdaFunctionStartFailedEventDetails.html#StepFunctions-Type-LambdaFunctionStartFailedEventDetails-error)": "***string***"
},
"[lambdaFunctionSucceededEventDetails](./API_HistoryEvent.html#StepFunctions-Type-HistoryEvent-lambdaFunctionSucceededEventDetails)": {
"[output](./API_LambdaFunctionSucceededEventDetails.html#StepFunctions-Type-LambdaFunctionSucceededEventDetails-output)": "***string***",
"[outputDetails](./API_LambdaFunctionSucceededEventDetails.html#StepFunctions-Type-LambdaFunctionSucceededEventDetails-outputDetails)": {
"[truncated](./API_HistoryEventExecutionDataDetails.html#StepFunctions-Type-HistoryEventExecutionDataDetails-truncated)": ***boolean***
}
},
"[lambdaFunctionTimedOutEventDetails](./API_HistoryEvent.html#StepFunctions-Type-HistoryEvent-lambdaFunctionTimedOutEventDetails)": {
"[cause](./API_LambdaFunctionTimedOutEventDetails.html#StepFunctions-Type-LambdaFunctionTimedOutEventDetails-cause)": "***string***",
"[error](./API_LambdaFunctionTimedOutEventDetails.html#StepFunctions-Type-LambdaFunctionTimedOutEventDetails-error)": "***string***"
},
"[mapIterationAbortedEventDetails](./API_HistoryEvent.html#StepFunctions-Type-HistoryEvent-mapIterationAbortedEventDetails)": {
"[index](./API_MapIterationEventDetails.html#StepFunctions-Type-MapIterationEventDetails-index)": ***number***,
"[name](./API_MapIterationEventDetails.html#StepFunctions-Type-MapIterationEventDetails-name)": "***string***"
},
"[mapIterationFailedEventDetails](./API_HistoryEvent.html#StepFunctions-Type-HistoryEvent-mapIterationFailedEventDetails)": {
"[index](./API_MapIterationEventDetails.html#StepFunctions-Type-MapIterationEventDetails-index)": ***number***,
"[name](./API_MapIterationEventDetails.html#StepFunctions-Type-MapIterationEventDetails-name)": "***string***"
},
"[mapIterationStartedEventDetails](./API_HistoryEvent.html#StepFunctions-Type-HistoryEvent-mapIterationStartedEventDetails)": {
"[index](./API_MapIterationEventDetails.html#StepFunctions-Type-MapIterationEventDetails-index)": ***number***,
"[name](./API_MapIterationEventDetails.html#StepFunctions-Type-MapIterationEventDetails-name)": "***string***"
},
"[mapIterationSucceededEventDetails](./API_HistoryEvent.html#StepFunctions-Type-HistoryEvent-mapIterationSucceededEventDetails)": {
"[index](./API_MapIterationEventDetails.html#StepFunctions-Type-MapIterationEventDetails-index)": ***number***,
"[name](./API_MapIterationEventDetails.html#StepFunctions-Type-MapIterationEventDetails-name)": "***string***"
},
"[mapRunFailedEventDetails](./API_HistoryEvent.html#StepFunctions-Type-HistoryEvent-mapRunFailedEventDetails)": {
"[cause](./API_MapRunFailedEventDetails.html#StepFunctions-Type-MapRunFailedEventDetails-cause)": "***string***",
"[error](./API_MapRunFailedEventDetails.html#StepFunctions-Type-MapRunFailedEventDetails-error)": "***string***"
},
"[mapRunRedrivenEventDetails](./API_HistoryEvent.html#StepFunctions-Type-HistoryEvent-mapRunRedrivenEventDetails)": {
"[mapRunArn](./API_MapRunRedrivenEventDetails.html#StepFunctions-Type-MapRunRedrivenEventDetails-mapRunArn)": "***string***",
"[redriveCount](./API_MapRunRedrivenEventDetails.html#StepFunctions-Type-MapRunRedrivenEventDetails-redriveCount)": ***number***
},
"[mapRunStartedEventDetails](./API_HistoryEvent.html#StepFunctions-Type-HistoryEvent-mapRunStartedEventDetails)": {
"[mapRunArn](./API_MapRunStartedEventDetails.html#StepFunctions-Type-MapRunStartedEventDetails-mapRunArn)": "***string***"
},
"[mapStateStartedEventDetails](./API_HistoryEvent.html#StepFunctions-Type-HistoryEvent-mapStateStartedEventDetails)": {
"[length](./API_MapStateStartedEventDetails.html#StepFunctions-Type-MapStateStartedEventDetails-length)": ***number***
},
"[previousEventId](./API_HistoryEvent.html#StepFunctions-Type-HistoryEvent-previousEventId)": ***number***,
"[stateEnteredEventDetails](./API_HistoryEvent.html#StepFunctions-Type-HistoryEvent-stateEnteredEventDetails)": {
"[input](./API_StateEnteredEventDetails.html#StepFunctions-Type-StateEnteredEventDetails-input)": "***string***",
"[inputDetails](./API_StateEnteredEventDetails.html#StepFunctions-Type-StateEnteredEventDetails-inputDetails)": {
"[truncated](./API_HistoryEventExecutionDataDetails.html#StepFunctions-Type-HistoryEventExecutionDataDetails-truncated)": ***boolean***
},
"[name](./API_StateEnteredEventDetails.html#StepFunctions-Type-StateEnteredEventDetails-name)": "***string***"
},
"[stateExitedEventDetails](./API_HistoryEvent.html#StepFunctions-Type-HistoryEvent-stateExitedEventDetails)": {
"[assignedVariables](./API_StateExitedEventDetails.html#StepFunctions-Type-StateExitedEventDetails-assignedVariables)": {
"***string***" : "***string***"
},
"[assignedVariablesDetails](./API_StateExitedEventDetails.html#StepFunctions-Type-StateExitedEventDetails-assignedVariablesDetails)": {
"[truncated](./API_AssignedVariablesDetails.html#StepFunctions-Type-AssignedVariablesDetails-truncated)": ***boolean***
},
"[name](./API_StateExitedEventDetails.html#StepFunctions-Type-StateExitedEventDetails-name)": "***string***",
"[output](./API_StateExitedEventDetails.html#StepFunctions-Type-StateExitedEventDetails-output)": "***string***",
"[outputDetails](./API_StateExitedEventDetails.html#StepFunctions-Type-StateExitedEventDetails-outputDetails)": {
"[truncated](./API_HistoryEventExecutionDataDetails.html#StepFunctions-Type-HistoryEventExecutionDataDetails-truncated)": ***boolean***
}
},
"[taskFailedEventDetails](./API_HistoryEvent.html#StepFunctions-Type-HistoryEvent-taskFailedEventDetails)": {
"[cause](./API_TaskFailedEventDetails.html#StepFunctions-Type-TaskFailedEventDetails-cause)": "***string***",
"[error](./API_TaskFailedEventDetails.html#StepFunctions-Type-TaskFailedEventDetails-error)": "***string***",
"[resource](./API_TaskFailedEventDetails.html#StepFunctions-Type-TaskFailedEventDetails-resource)": "***string***",
"[resourceType](./API_TaskFailedEventDetails.html#StepFunctions-Type-TaskFailedEventDetails-resourceType)": "***string***"
},
"[taskScheduledEventDetails](./API_HistoryEvent.html#StepFunctions-Type-HistoryEvent-taskScheduledEventDetails)": {
"[heartbeatInSeconds](./API_TaskScheduledEventDetails.html#StepFunctions-Type-TaskScheduledEventDetails-heartbeatInSeconds)": ***number***,
"[parameters](./API_TaskScheduledEventDetails.html#StepFunctions-Type-TaskScheduledEventDetails-parameters)": "***string***",
"[region](./API_TaskScheduledEventDetails.html#StepFunctions-Type-TaskScheduledEventDetails-region)": "***string***",
"[resource](./API_TaskScheduledEventDetails.html#StepFunctions-Type-TaskScheduledEventDetails-resource)": "***string***",
"[resourceType](./API_TaskScheduledEventDetails.html#StepFunctions-Type-TaskScheduledEventDetails-resourceType)": "***string***",
"[taskCredentials](./API_TaskScheduledEventDetails.html#StepFunctions-Type-TaskScheduledEventDetails-taskCredentials)": {
"[roleArn](./API_TaskCredentials.html#StepFunctions-Type-TaskCredentials-roleArn)": "***string***"
},
"[timeoutInSeconds](./API_TaskScheduledEventDetails.html#StepFunctions-Type-TaskScheduledEventDetails-timeoutInSeconds)": ***number***
},
"[taskStartedEventDetails](./API_HistoryEvent.html#StepFunctions-Type-HistoryEvent-taskStartedEventDetails)": {
"[resource](./API_TaskStartedEventDetails.html#StepFunctions-Type-TaskStartedEventDetails-resource)": "***string***",
"[resourceType](./API_TaskStartedEventDetails.html#StepFunctions-Type-TaskStartedEventDetails-resourceType)": "***string***"
},
"[taskStartFailedEventDetails](./API_HistoryEvent.html#StepFunctions-Type-HistoryEvent-taskStartFailedEventDetails)": {
"[cause](./API_TaskStartFailedEventDetails.html#StepFunctions-Type-TaskStartFailedEventDetails-cause)": "***string***",
"[error](./API_TaskStartFailedEventDetails.html#StepFunctions-Type-TaskStartFailedEventDetails-error)": "***string***",
"[resource](./API_TaskStartFailedEventDetails.html#StepFunctions-Type-TaskStartFailedEventDetails-resource)": "***string***",
"[resourceType](./API_TaskStartFailedEventDetails.html#StepFunctions-Type-TaskStartFailedEventDetails-resourceType)": "***string***"
},
"[taskSubmitFailedEventDetails](./API_HistoryEvent.html#StepFunctions-Type-HistoryEvent-taskSubmitFailedEventDetails)": {
"[cause](./API_TaskSubmitFailedEventDetails.html#StepFunctions-Type-TaskSubmitFailedEventDetails-cause)": "***string***",
"[error](./API_TaskSubmitFailedEventDetails.html#StepFunctions-Type-TaskSubmitFailedEventDetails-error)": "***string***",
"[resource](./API_TaskSubmitFailedEventDetails.html#StepFunctions-Type-TaskSubmitFailedEventDetails-resource)": "***string***",
"[resourceType](./API_TaskSubmitFailedEventDetails.html#StepFunctions-Type-TaskSubmitFailedEventDetails-resourceType)": "***string***"
},
"[taskSubmittedEventDetails](./API_HistoryEvent.html#StepFunctions-Type-HistoryEvent-taskSubmittedEventDetails)": {
"[output](./API_TaskSubmittedEventDetails.html#StepFunctions-Type-TaskSubmittedEventDetails-output)": "***string***",
"[outputDetails](./API_TaskSubmittedEventDetails.html#StepFunctions-Type-TaskSubmittedEventDetails-outputDetails)": {
"[truncated](./API_HistoryEventExecutionDataDetails.html#StepFunctions-Type-HistoryEventExecutionDataDetails-truncated)": ***boolean***
},
"[resource](./API_TaskSubmittedEventDetails.html#StepFunctions-Type-TaskSubmittedEventDetails-resource)": "***string***",
"[resourceType](./API_TaskSubmittedEventDetails.html#StepFunctions-Type-TaskSubmittedEventDetails-resourceType)": "***string***"
},
"[taskSucceededEventDetails](./API_HistoryEvent.html#StepFunctions-Type-HistoryEvent-taskSucceededEventDetails)": {
"[output](./API_TaskSucceededEventDetails.html#StepFunctions-Type-TaskSucceededEventDetails-output)": "***string***",
"[outputDetails](./API_TaskSucceededEventDetails.html#StepFunctions-Type-TaskSucceededEventDetails-outputDetails)": {
"[truncated](./API_HistoryEventExecutionDataDetails.html#StepFunctions-Type-HistoryEventExecutionDataDetails-truncated)": ***boolean***
},
"[resource](./API_TaskSucceededEventDetails.html#StepFunctions-Type-TaskSucceededEventDetails-resource)": "***string***",
"[resourceType](./API_TaskSucceededEventDetails.html#StepFunctions-Type-TaskSucceededEventDetails-resourceType)": "***string***"
},
"[taskTimedOutEventDetails](./API_HistoryEvent.html#StepFunctions-Type-HistoryEvent-taskTimedOutEventDetails)": {
"[cause](./API_TaskTimedOutEventDetails.html#StepFunctions-Type-TaskTimedOutEventDetails-cause)": "***string***",
"[error](./API_TaskTimedOutEventDetails.html#StepFunctions-Type-TaskTimedOutEventDetails-error)": "***string***",
"[resource](./API_TaskTimedOutEventDetails.html#StepFunctions-Type-TaskTimedOutEventDetails-resource)": "***string***",
"[resourceType](./API_TaskTimedOutEventDetails.html#StepFunctions-Type-TaskTimedOutEventDetails-resourceType)": "***string***"
},
"[timestamp](./API_HistoryEvent.html#StepFunctions-Type-HistoryEvent-timestamp)": ***number***,
"[type](./API_HistoryEvent.html#StepFunctions-Type-HistoryEvent-type)": "***string***"
}
],
"[nextToken](#StepFunctions-GetExecutionHistory-response-nextToken)": "***string***"
}`
```
## Response Elements
If the action is successful, the service sends back an HTTP 200 response.
The following data is returned in JSON format by the service.
**
[events](#API_GetExecutionHistory_ResponseSyntax)
**
The list of events that occurred in the execution.
Type: Array of [HistoryEvent](./API_HistoryEvent.html) objects
**
[nextToken](#API_GetExecutionHistory_ResponseSyntax)
**
If `nextToken` is returned, there are more results available. The value of `nextToken` is a unique pagination token for each page.
Make the call again using the returned token to retrieve the next page. Keep all other arguments unchanged. Each pagination token expires after 24 hours. Using an expired pagination token will return an *HTTP 400 InvalidToken* error.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 1024.
## Errors
For information about the errors that are common to all actions, see [Common Errors](./CommonErrors.html).
**
ExecutionDoesNotExist
**
The specified execution does not exist.
HTTP Status Code: 400
**
InvalidArn
**
The provided Amazon Resource Name (ARN) is not valid.
HTTP Status Code: 400
**
InvalidToken
**
The provided token is not valid.
HTTP Status Code: 400
**
KmsAccessDeniedException
**
Either your AWS KMS key policy or API caller does not have the required permissions.
HTTP Status Code: 400
**
KmsInvalidStateException
**
The AWS KMS key is not in valid state, for example: Disabled or Deleted.
**
kmsKeyState
**
Current status of the AWS KMS; key. For example: `DISABLED`, `PENDING\_DELETION`, `PENDING\_IMPORT`, `UNAVAILABLE`, `CREATING`.
HTTP Status Code: 400
**
KmsThrottlingException
**
Received when AWS KMS returns `ThrottlingException` for a AWS KMS call that Step Functions makes on behalf of the caller.
HTTP Status Code: 400
### HelloWorld Execution History
The following shows example output from `GetExecutionHistory` for a simple
`HelloWorld` state machine, comprised of a single `Pass`
state.
#### Sample Response
```
`{
"events": [
{
"timestamp": 1525283875.58,
"executionStartedEventDetails": {
"input": "{}",
"inputDetails": {
"truncated": false
},
"roleArn": "arn:aws:iam::123456789123:role/service-role/StatesExecutionRole-us-east-1"
},
"type": "ExecutionStarted",
"id": 1,
"previousEventId": 0
},
{
"timestamp": 1525283875.612,
"type": "PassStateEntered",
"id": 2,
"stateEnteredEventDetails": {
"input": "{}",
"inputDetails": {
"truncated": false
},
"name": "HelloWorld"
},
"previousEventId": 0
},
{
"timestamp": 1525283875.612,
"stateExitedEventDetails": {
"output": "\\"Hello World!\\"",
"outputDetails": {
"truncated": false
},
"name": "HelloWorld"
},
"type": "PassStateExited",
"id": 3,
"previousEventId": 2
},
{
"executionSucceededEventDetails": {
"output": "\\"Hello World!\\"",
"outputDetails": {
"truncated": false
}
},
"timestamp": 1525283875.612,
"type": "ExecutionSucceeded",
"id": 4,
"previousEventId": 3
}
]
}
`
```