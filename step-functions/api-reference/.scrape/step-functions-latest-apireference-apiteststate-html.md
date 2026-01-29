---
url: https://docs.aws.amazon.com/step-functions/latest/apireference/API_TestState.html
title: TestState
word_count: 1177
filtered: true
elements_removed: 0
density_score: 0.91
---

TestState - AWS Step Functions
TestState - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/apireference/step-functions-api.pdf#API_TestState)
[Request Syntax](#API_TestState_RequestSyntax)[Request Parameters](#API_TestState_RequestParameters)[Response Syntax](#API_TestState_ResponseSyntax)[Response Elements](#API_TestState_ResponseElements)[Errors](#API_TestState_Errors)[See Also](#API_TestState_SeeAlso)
# TestState
Accepts the definition of a single state and executes it. You can test a state without creating a state machine or updating an existing state machine. Using this API, you can test the following:
* A state's [input and output processing](https://docs.aws.amazon.com/step-functions/latest/dg/test-state-isolation.html#test-state-input-output-dataflow) data flow
* An [AWS service integration](https://docs.aws.amazon.com/step-functions/latest/dg/connect-to-services.html) request and response
* An [HTTP Task](https://docs.aws.amazon.com/step-functions/latest/dg/call-https-apis.html) request and response
You can call this API on only one state at a time. The states that you can test include the following:
* [All Task types](https://docs.aws.amazon.com/step-functions/latest/dg/amazon-states-language-task-state.html#task-types) except [Activity](https://docs.aws.amazon.com/step-functions/latest/dg/concepts-activities.html)
* [Pass](https://docs.aws.amazon.com/step-functions/latest/dg/amazon-states-language-pass-state.html)
* [Wait](https://docs.aws.amazon.com/step-functions/latest/dg/amazon-states-language-wait-state.html)
* [Choice](https://docs.aws.amazon.com/step-functions/latest/dg/amazon-states-language-choice-state.html)
* [Succeed](https://docs.aws.amazon.com/step-functions/latest/dg/amazon-states-language-succeed-state.html)
* [Fail](https://docs.aws.amazon.com/step-functions/latest/dg/amazon-states-language-fail-state.html)
The `TestState` API assumes an IAM role which must contain the required IAM permissions for the resources your state is accessing. For information about the permissions a state might need, see [IAM permissions to test a state](https://docs.aws.amazon.com/step-functions/latest/dg/test-state-isolation.html#test-state-permissions).
The `TestState` API can run for up to five minutes. If the execution of a state exceeds this duration, it fails with the `States.Timeout` error.
`TestState` only supports the following when a mock is specified: [Activity tasks](https://docs.aws.amazon.com/step-functions/latest/dg/concepts-activities.html), `.sync` or `.waitForTaskToken`
[service integration patterns](https://docs.aws.amazon.com/step-functions/latest/dg/connect-to-resource.html), [Parallel](https://docs.aws.amazon.com/step-functions/latest/dg/amazon-states-language-parallel-state.html), or [Map](https://docs.aws.amazon.com/step-functions/latest/dg/amazon-states-language-map-state.html) states.
## Request Syntax
```
`{
"[context](#StepFunctions-TestState-request-context)": "`string`",
"[definition](#StepFunctions-TestState-request-definition)": "`string`",
"[input](#StepFunctions-TestState-request-input)": "`string`",
"[inspectionLevel](#StepFunctions-TestState-request-inspectionLevel)": "`string`",
"[mock](#StepFunctions-TestState-request-mock)": {
"[errorOutput](./API_MockInput.html#StepFunctions-Type-MockInput-errorOutput)": {
"[cause](./API_MockErrorOutput.html#StepFunctions-Type-MockErrorOutput-cause)": "`string`",
"[error](./API_MockErrorOutput.html#StepFunctions-Type-MockErrorOutput-error)": "`string`"
},
"[fieldValidationMode](./API_MockInput.html#StepFunctions-Type-MockInput-fieldValidationMode)": "`string`",
"[result](./API_MockInput.html#StepFunctions-Type-MockInput-result)": "`string`"
},
"[revealSecrets](#StepFunctions-TestState-request-revealSecrets)": `boolean`,
"[roleArn](#StepFunctions-TestState-request-roleArn)": "`string`",
"[stateConfiguration](#StepFunctions-TestState-request-stateConfiguration)": {
"[errorCausedByState](./API_TestStateConfiguration.html#StepFunctions-Type-TestStateConfiguration-errorCausedByState)": "`string`",
"[mapItemReaderData](./API_TestStateConfiguration.html#StepFunctions-Type-TestStateConfiguration-mapItemReaderData)": "`string`",
"[mapIterationFailureCount](./API_TestStateConfiguration.html#StepFunctions-Type-TestStateConfiguration-mapIterationFailureCount)": `number`,
"[retrierRetryCount](./API_TestStateConfiguration.html#StepFunctions-Type-TestStateConfiguration-retrierRetryCount)": `number`
},
"[stateName](#StepFunctions-TestState-request-stateName)": "`string`",
"[variables](#StepFunctions-TestState-request-variables)": "`string`"
}`
```
## Request Parameters
For information about the parameters that are common to all actions, see [Common Parameters](./CommonParameters.html).
The request accepts the following data in JSON format.
**
[context](#API_TestState_RequestSyntax)
**
A JSON string representing a valid Context object for the state under test. This field may only be specified if a mock is specified in the same request.
Type: String
Length Constraints: Maximum length of 262144.
Required: No
**
[definition](#API_TestState_RequestSyntax)
**
The [Amazon States Language](https://docs.aws.amazon.com/step-functions/latest/dg/concepts-amazon-states-language.html) (ASL) definition of the state or state machine.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 1048576.
Required: Yes
**
[input](#API_TestState_RequestSyntax)
**
A string that contains the JSON input data for the state.
Type: String
Length Constraints: Maximum length of 262144.
Required: No
**
[inspectionLevel](#API_TestState_RequestSyntax)
**
Determines the values to return when a state is tested. You can specify one of the following types:
* `INFO`: Shows the final state output. By default, Step Functions sets `inspectionLevel` to `INFO` if you don't specify a level.
* `DEBUG`: Shows the final state output along with the input and output data processing result.
* `TRACE`: Shows the HTTP request and response for an HTTP Task. This level also shows the final state output along with the input and output data processing result.
Each of these levels also provide information about the status of the state execution and the next state to transition to.
Type: String
Valid Values: `INFO | DEBUG | TRACE`
Required: No
**
[mock](#API_TestState_RequestSyntax)
**
Defines a mocked result or error for the state under test.
A mock can only be specified for Task, Map, or Parallel states. If it is specified for another state type, an exception will be thrown.
Type: [MockInput](./API_MockInput.html) object
Required: No
**
[revealSecrets](#API_TestState_RequestSyntax)
**
Specifies whether or not to include secret information in the test result. For HTTP Tasks, a secret includes the data that an EventBridge connection adds to modify the HTTP request headers, query parameters, and body. Step Functions doesn't omit any information included in the state definition or the HTTP response.
If you set `revealSecrets` to `true`, you must make sure that the IAM user that calls the `TestState` API has permission for the `states:RevealSecrets` action. For an example of IAM policy that sets the `states:RevealSecrets` permission, see [IAM permissions to test a state](https://docs.aws.amazon.com/step-functions/latest/dg/test-state-isolation.html#test-state-permissions). Without this permission, Step Functions throws an access denied error.
By default, `revealSecrets` is set to `false`.
Type: Boolean
Required: No
**
[roleArn](#API_TestState_RequestSyntax)
**
The Amazon Resource Name (ARN) of the execution role with the required IAM permissions for the state.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 256.
Required: No
**
[stateConfiguration](#API_TestState_RequestSyntax)
**
Contains configurations for the state under test.
Type: [TestStateConfiguration](./API_TestStateConfiguration.html) object
Required: No
**
[stateName](#API_TestState_RequestSyntax)
**
Denotes the particular state within a state machine definition to be tested. If this field is specified, the `definition` must contain a fully-formed state machine definition.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 80.
Required: No
**
[variables](#API_TestState_RequestSyntax)
**
JSON object literal that sets variables used in the state under test. Object keys are the variable names and values are the variable values.
Type: String
Length Constraints: Maximum length of 262144.
Required: No
## Response Syntax
```
`{
"[cause](#StepFunctions-TestState-response-cause)": "***string***",
"[error](#StepFunctions-TestState-response-error)": "***string***",
"[inspectionData](#StepFunctions-TestState-response-inspectionData)": {
"[afterArguments](./API_InspectionData.html#StepFunctions-Type-InspectionData-afterArguments)": "***string***",
"[afterInputPath](./API_InspectionData.html#StepFunctions-Type-InspectionData-afterInputPath)": "***string***",
"[afterItemBatcher](./API_InspectionData.html#StepFunctions-Type-InspectionData-afterItemBatcher)": "***string***",
"[afterItemSelector](./API_InspectionData.html#StepFunctions-Type-InspectionData-afterItemSelector)": "***string***",
"[afterItemsPath](./API_InspectionData.html#StepFunctions-Type-InspectionData-afterItemsPath)": "***string***",
"[afterItemsPointer](./API_InspectionData.html#StepFunctions-Type-InspectionData-afterItemsPointer)": "***string***",
"[afterParameters](./API_InspectionData.html#StepFunctions-Type-InspectionData-afterParameters)": "***string***",
"[afterResultPath](./API_InspectionData.html#StepFunctions-Type-InspectionData-afterResultPath)": "***string***",
"[afterResultSelector](./API_InspectionData.html#StepFunctions-Type-InspectionData-afterResultSelector)": "***string***",
"[errorDetails](./API_InspectionData.html#StepFunctions-Type-InspectionData-errorDetails)": {
"[catchIndex](./API_InspectionErrorDetails.html#StepFunctions-Type-InspectionErrorDetails-catchIndex)": ***number***,
"[retryBackoffIntervalSeconds](./API_InspectionErrorDetails.html#StepFunctions-Type-InspectionErrorDetails-retryBackoffIntervalSeconds)": ***number***,
"[retryIndex](./API_InspectionErrorDetails.html#StepFunctions-Type-InspectionErrorDetails-retryIndex)": ***number***
},
"[input](./API_InspectionData.html#StepFunctions-Type-InspectionData-input)": "***string***",
"[maxConcurrency](./API_InspectionData.html#StepFunctions-Type-InspectionData-maxConcurrency)": ***number***,
"[request](./API_InspectionData.html#StepFunctions-Type-InspectionData-request)": {
"[body](./API_InspectionDataRequest.html#StepFunctions-Type-InspectionDataRequest-body)": "***string***",
"[headers](./API_InspectionDataRequest.html#StepFunctions-Type-InspectionDataRequest-headers)": "***string***",
"[method](./API_InspectionDataRequest.html#StepFunctions-Type-InspectionDataRequest-method)": "***string***",
"[protocol](./API_InspectionDataRequest.html#StepFunctions-Type-InspectionDataRequest-protocol)": "***string***",
"[url](./API_InspectionDataRequest.html#StepFunctions-Type-InspectionDataRequest-url)": "***string***"
},
"[response](./API_InspectionData.html#StepFunctions-Type-InspectionData-response)": {
"[body](./API_InspectionDataResponse.html#StepFunctions-Type-InspectionDataResponse-body)": "***string***",
"[headers](./API_InspectionDataResponse.html#StepFunctions-Type-InspectionDataResponse-headers)": "***string***",
"[protocol](./API_InspectionDataResponse.html#StepFunctions-Type-InspectionDataResponse-protocol)": "***string***",
"[statusCode](./API_InspectionDataResponse.html#StepFunctions-Type-InspectionDataResponse-statusCode)": "***string***",
"[statusMessage](./API_InspectionDataResponse.html#StepFunctions-Type-InspectionDataResponse-statusMessage)": "***string***"
},
"[result](./API_InspectionData.html#StepFunctions-Type-InspectionData-result)": "***string***",
"[toleratedFailureCount](./API_InspectionData.html#StepFunctions-Type-InspectionData-toleratedFailureCount)": ***number***,
"[toleratedFailurePercentage](./API_InspectionData.html#StepFunctions-Type-InspectionData-toleratedFailurePercentage)": ***number***,
"[variables](./API_InspectionData.html#StepFunctions-Type-InspectionData-variables)": "***string***"
},
"[nextState](#StepFunctions-TestState-response-nextState)": "***string***",
"[output](#StepFunctions-TestState-response-output)": "***string***",
"[status](#StepFunctions-TestState-response-status)": "***string***"
}`
```
## Response Elements
If the action is successful, the service sends back an HTTP 200 response.
The following data is returned in JSON format by the service.
**
[cause](#API_TestState_ResponseSyntax)
**
A detailed explanation of the cause for the error when the execution of a state fails.
Type: String
Length Constraints: Minimum length of 0. Maximum length of 32768.
**
[error](#API_TestState_ResponseSyntax)
**
The error returned when the execution of a state fails.
Type: String
Length Constraints: Minimum length of 0. Maximum length of 256.
**
[inspectionData](#API_TestState_ResponseSyntax)
**
Returns additional details about the state's execution, including its input and output data processing flow, and HTTP request and response information. The `inspectionLevel` request parameter specifies which details are returned.
Type: [InspectionData](./API_InspectionData.html) object
**
[nextState](#API_TestState_ResponseSyntax)
**
The name of the next state to transition to. If you haven't defined a next state in your definition or if the execution of the state fails, this field doesn't contain a value.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 80.
**
[output](#API_TestState_ResponseSyntax)
**
The JSON output data of the state. Length constraints apply to the payload size, and are expressed as bytes in UTF-8 encoding.
Type: String
Length Constraints: Maximum length of 262144.
**
[status](#API_TestState_ResponseSyntax)
**
The execution status of the state.
Type: String
Valid Values: `SUCCEEDED | FAILED | RETRIABLE | CAUGHT\_ERROR`
## Errors
For information about the errors that are common to all actions, see [Common Errors](./CommonErrors.html).
**
InvalidArn
**
The provided Amazon Resource Name (ARN) is not valid.
HTTP Status Code: 400
**
InvalidDefinition
**
The provided Amazon States Language definition is not valid.
HTTP Status Code: 400
**
InvalidExecutionInput
**
The provided JSON input data is not valid.
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