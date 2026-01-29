---
url: https://docs.aws.amazon.com/step-functions/latest/dg/sfn-local-test-sm-exec.html
title: Using mocked service integrations for testing in Step Functions Local
word_count: 3082
filtered: true
elements_removed: 0
density_score: 0.80
---

Using mocked service integrations for testing in Step Functions Local - AWS Step Functions
Using mocked service integrations for testing in Step Functions Local - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/dg/step-functions-dg.pdf#sfn-local-test-sm-exec)
[Configuring mocked service integrations](#mock-resp-struct-req)[Step 1: Specify Mocked Service Integrations in a Mock Configuration File](#create-mock-config-file)[Step 2: Provide the Mock Configuration File to Step Functions
Local](#supply-mock-config-file)[Step 3: Run Mocked Service Integration Tests](#run-mocked-serv-integ-tests)[Configuration file structure](#sfn-local-mock-cfg-file)
# Using mocked service integrations for testing in Step Functions Local
###### Step Functions Local is unsupported
Step Functions Local does **not** provide feature parity and is **unsupported**.
You might consider third party solutions that emulate Step Functions for testing
purposes.
In Step Functions Local, you can test the execution paths of your state machines without actually calling integrated services by using mocked service integrations. To configure your state machines to use mocked service integrations, you create a mock configuration file. In this file, you define the desired output of your service integrations as mocked responses and the executions which use your mocked responses to simulate an execution path as test cases.
By providing the mock configuration file to
Step Functions
Local, you can test service integration calls by running state machines that use the mocked
responses specified in the test cases instead of making actual service integration calls.
###### Note
If you
don't
specify mocked service integration responses in the mock configuration file, Step Functions Local will
invoke the AWS service integration using the endpoint you configured while setting up Step Functions
Local. For information about configuring endpoints for Step Functions Local, see [Setting Configuration Options for Step Functions Local](./sfn-local.html#sfn-local-config-options).
This topic uses several concepts which are defined in the following list:
* Mocked Service Integrations - Refers to Task states configured to use mocked responses instead of performing actual service calls.
* Mocked Responses - Refers to mock data that Task states can be configured to use.
* Test Cases - Refers to state machine executions configured to use mocked service integrations.
* Mock Configuration File - Refers to mock configuration file that contains JSON, which defines
mocked service integrations, mocked responses, and test cases.
## Configuring mocked service integrations
You can mock any service integration using Step Functions Local. However, Step Functions Local doesn’t enforce the mocks to be the same as the real APIs. A mocked Task will never call the service endpoint. If you do not specify a mocked response, a Task will attempt to call the service endpoints. In addition, Step Functions Local will automatically generate a task token when you mock a Task using the `.waitForTaskToken`.
## Step 1: Specify Mocked Service Integrations in a Mock Configuration File
You can
test
Step Functions AWS SDK and optimized service integrations using Step Functions Local. The following image shows the state machine defined in the State machine definition tab:
![Mocked service integration example.](https://docs.aws.amazon.com/images/step-functions/latest/dg/images/msi-graph.png)
To do this, you must create a mock
configuration file containing sections as defined in [Mock configuration file structure](#mock-cfg-struct).
1. Create a file named `MockConfigFile.json` to configure tests with
mocked service integrations.
The following example shows a mock configuration file referencing a state machine with two defined states named `LambdaState` and `SQSState`.
Mock configuration file example
The following is an example of a mock configuration
file which
demonstrates how to mock responses from [invoking a Lambda
function](./connect-lambda.html) and [sending a message to Amazon SQS](./connect-sqs.html). In
this example, the [LambdaSQSIntegration](#mock-cfg-sm-sect) state machine contains three test cases
named `HappyPath`, `RetryPath`, and `HybridPath`
which mock the `Task`
states named `LambdaState` and `SQSState`. These
states use the `MockedLambdaSuccess`, `MockedSQSSuccess`, and
`MockedLambdaRetry` mocked service responses. These mocked service responses are defined in the `MockedResponses` section of the file.
```
`{
"StateMachines":{
"LambdaSQSIntegration":{
"TestCases":{
"HappyPath":{
"LambdaState":"MockedLambdaSuccess",
"SQSState":"MockedSQSSuccess"
},
"RetryPath":{
"LambdaState":"MockedLambdaRetry",
"SQSState":"MockedSQSSuccess"
},
"HybridPath":{
"LambdaState":"MockedLambdaSuccess"
}
}
}
},
"MockedResponses":{
"MockedLambdaSuccess":{
"0":{
"Return":{
"StatusCode":200,
"Payload":{
"StatusCode":200,
"body":"Hello from Lambda!"
}
}
}
},
"LambdaMockedResourceNotReady":{
"0":{
"Throw":{
"Error":"Lambda.ResourceNotReadyException",
"Cause":"Lambda resource is not ready."
}
}
},
"MockedSQSSuccess":{
"0":{
"Return":{
"MD5OfMessageBody":"3bcb6e8e-7h85-4375-b0bc-1a59812c6e51",
"MessageId":"3bcb6e8e-8b51-4375-b0bc-1a59812c6e51"
}
}
},
"MockedLambdaRetry":{
"0":{
"Throw":{
"Error":"Lambda.ResourceNotReadyException",
"Cause":"Lambda resource is not ready."
}
},
"1-2":{
"Throw":{
"Error":"Lambda.TimeoutException",
"Cause":"Lambda timed out."
}
},
"3":{
"Return":{
"StatusCode":200,
"Payload":{
"StatusCode":200,
"body":"Hello from Lambda!"
}
}
}
}
}
}`
```
State machine definition
The following is an example of a state machine definition called `LambdaSQSIntegration`, which defines two service integration task states named `LambdaState` and `SQSState`. `LambdaState` contains a retry policy based on `States.ALL`.
```
`{
"Comment":"This state machine is called: LambdaSQSIntegration",
"StartAt":"LambdaState",
"States":{
"LambdaState":{
"Type":"Task",
"Resource":"arn:aws:states:::lambda:invoke",
"Parameters":{
"Payload.$":"$",
"FunctionName":"HelloWorldFunction"
},
"Retry":[
{
"ErrorEquals":[
"States.ALL"
],
"IntervalSeconds":2,
"MaxAttempts":3,
"BackoffRate":2
}
],
"Next":"SQSState"
},
"SQSState":{
"Type":"Task",
"Resource":"arn:aws:states:::sqs:sendMessage",
"Parameters":{
"QueueUrl":"https://sqs.us-east-1.amazonaws.com/`account-id`/myQueue",
"MessageBody.$":"$"
},
"End": true
}
}
}`
```
You can run the `LambdaSQSIntegration` state machine definition referenced in the mock configuration file using one of the following test cases:
* `HappyPath` - This test mocks the output of `LambdaState` and `SQSState` using
`MockedLambdaSuccess` and `MockedSQSSuccess` respectively.
* The `LambdaState` will return the following value:
```
`"0":{
"Return":{
"StatusCode":200,
"Payload":{
"StatusCode":200,
"body":"Hello from Lambda!"
}
}
}`
```
* The `SQSState` will return the following value:
```
`"0":{
"Return":{
"MD5OfMessageBody":"3bcb6e8e-7h85-4375-b0bc-1a59812c6e51",
"MessageId":"3bcb6e8e-8b51-4375-b0bc-1a59812c6e51"
}
}`
```
* `RetryPath` - This test mocks the output of `LambdaState` and `SQSState` using
`MockedLambdaRetry` and `MockedSQSSuccess` respectively. In
addition, `LambdaState` is configured to perform four retry attempts. The
mocked responses for these attempts are defined and indexed in the
`MockedLambdaRetry` state.
* The initial attempt ends with a task failure containing a cause and error message as shown in the following example:
```
`"0":{
"Throw": {
"Error": "Lambda.ResourceNotReadyException",
"Cause": "Lambda resource is not ready."
}
}`
```
* The first and second retry attempts end with a task failure containing a cause and error message as shown in the following example:
```
`"1-2":{
"Throw": {
"Error": "Lambda.TimeoutException",
"Cause": "Lambda timed out."
}
}`
```
* The third retry attempt ends with a task success containing state result from Payload section in the mocked Lambda response.
```
`"3":{
"Return": {
"StatusCode": 200,
"Payload": {
"StatusCode": 200,
"body": "Hello from Lambda!"
}
}
}`
```
###### Note
* For states with a retry policy, Step Functions Local will exhaust the retry attempts set in the policy until it receives a success response. This means that you must denote mocks for retries with consecutive attempt numbers and should cover all the retry attempts before returning a success response.
* If you do not specify a mocked response for a specific retry attempt, for example, retry "3", the state machine execution will fail.
* `HybridPath` - This test mocks the output of `LambdaState`. After `LambdaState` runs successfully and receives mocked data as a response, `SQSState` performs an actual service call to the resource specified in production.
For information about how to start test executions with mocked service integrations,
see [Step 3: Run Mocked Service Integration Tests](#run-mocked-serv-integ-tests).
* Make sure
that
the mocked
responses'
structure conforms to the structure of actual service responses you
receive when you make integrated service calls. For information about the structural
requirements for mocked responses, see [Configuring mocked service integrations](#mock-resp-struct-req).
In the previous
example mock configuration file, the mocked responses defined in
`MockedLambdaSuccess` and `MockedLambdaRetry` conform to the
structure of actual responses that are returned from calling
`HelloFromLambda`.
###### Important
AWS
service responses can vary in structure between different services. Step Functions Local doesn't
validate if mocked response structures conform to actual service response structures.
You must ensure that your mocked responses conform to actual responses before
testing. To
review
the structure of service responses, you can either perform the actual
service calls using Step Functions or view the documentation
for
those services.
## Step 2: Provide the Mock Configuration File to Step Functions
Local
You can provide the mock configuration file to Step Functions Local
in
one of the following ways:
Docker
###### Note
If you're using the Docker version of Step Functions Local, you can provide the mock
configuration file using an environment variable only. In addition, you must mount the
mock configuration file onto the Step Functions Local container at the initial server
boot-up.
Mount the mock configuration file onto any directory
within
the Step Functions Local container.
Then,
set an environment variable named
`SFN\_MOCK\_CONFIG`
that
contains the path to the mock configuration file
in
the container. This
method
enables the mock configuration file to be named anything as long as the environment variable
contains the file path and name.
The following command
shows
the format to start the Docker image.
```
`docker run -p 8083:8083
--mount type=bind,readonly,source={absolute path to mock config file},destination=/home/StepFunctionsLocal/MockConfigFile.json
-e SFN\_MOCK\_CONFIG="/home/StepFunctionsLocal/MockConfigFile.json" amazon/aws-stepfunctions-local`
```
The following example uses the command to start the Docker image.
```
`docker run -p 8083:8083
--mount type=bind,readonly,source=/Users/admin/Desktop/workplace/MockConfigFile.json,destination=/home/StepFunctionsLocal/MockConfigFile.json
-e SFN\_MOCK\_CONFIG="/home/StepFunctionsLocal/MockConfigFile.json" amazon/aws-stepfunctions-local`
```
JAR File
Use one of the following ways to provide the mock configuration file to Step Functions
Local:
* Place the mock configuration file in the same directory as
`Step FunctionsLocal.jar`. When using this method, you must name the mock
configuration file
`MockConfigFile.json`.
* In the session running Step Functions Local, set an environment variable named
`SFN\_MOCK\_CONFIG`, to the full path of the mock configuration file.
This
method enables the mock configuration file to be named anything as
long as the environment variable contains its file path and name.
In
the following example, the `SFN\_MOCK\_CONFIG` variable is set to point at a
mock configuration file named `EnvSpecifiedMockConfig.json`, located
in the `/home/workspace` directory.
```
`export SFN\_MOCK\_CONFIG="/home/workspace/EnvSpecifiedMockConfig.json"`
```
###### Note
* If you do not provide the environment variable `SFN\_MOCK\_CONFIG` to Step Functions Local, by default, it will attempt to read a mock configuration file named `MockConfigFile.json` in the directory from which you launched Step Functions Local.
* If you place the mock configuration file in the same directory as
`Step FunctionsLocal.jar`
and set
the environment variable `SFN\_MOCK\_CONFIG`, Step Functions Local will read the file
specified by the environment variable.
## Step 3: Run Mocked Service Integration Tests
After
you
create and provide a mock configuration file to Step Functions Local, run the state
machine configured in the mock configuration file using mocked service integrations. Then check the execution results using an API
action.
1. Create a state machine based on the previously mentioned definition in the [mock configuration file](#create-mock-config-file).
```
`aws stepfunctions create-state-machine \\
--endpoint http://localhost:8083 \\
--definition "{\\"Comment\\":\\"Thisstatemachineiscalled:LambdaSQSIntegration\\",\\"StartAt\\":\\"LambdaState\\",\\"States\\":{\\"LambdaState\\":{\\"Type\\":\\"Task\\",\\"Resource\\":\\"arn:aws:states:::lambda:invoke\\",\\"Parameters\\":{\\"Payload.$\\":\\"$\\",\\"FunctionName\\":\\"arn:aws:lambda:`region`:`account-id`:function:HelloWorldFunction\\"},\\"Retry\\":[{\\"ErrorEquals\\":[\\"States.ALL\\"],\\"IntervalSeconds\\":2,\\"MaxAttempts\\":3,\\"BackoffRate\\":2}],\\"Next\\":\\"SQSState\\"},\\"SQSState\\":{\\"Type\\":\\"Task\\",\\"Resource\\":\\"arn:aws:states:::sqs:sendMessage\\",\\"Parameters\\":{\\"QueueUrl\\":\\"https://sqs.us-east-1.amazonaws.com/`account-id`/myQueue\\",\\"MessageBody.$\\":\\"$\\"},\\"End\\":true}}}" \\
--name "LambdaSQSIntegration" --role-arn "arn:aws:iam::`account-id`:role/service-role/LambdaSQSIntegration"`
```
2. Run the
state machine using mocked service integrations.
To use the mock configuration file, make a `[StartExecution](https://docs.aws.amazon.com/step-functions/latest/apireference/API_StartExecution.html)` API call on a
state machine configured in the mock configuration file. To do this, append the suffix,
`#`test\_name``,
to the state machine ARN used by `StartExecution`.
``test\_name`` is a test case, which is configured for the state machine in the
same mock configuration file.
The following command is an example that uses the `LambdaSQSIntegration`
state machine and mock configuration. In this example, the
`LambdaSQSIntegration` state machine is executed using the
`HappyPath` test defined in [Step 1: Specify Mocked Service Integrations in a Mock Configuration File](#create-mock-config-file). The `HappyPath` test contains the
configuration for the execution to
handle
mock service integration calls that `LambdaState` and
`SQSState` states make using the `MockedLambdaSuccess` and
`MockedSQSSuccess` mocked service responses.
```
`aws stepfunctions start-execution \\
--endpoint http://localhost:8083 \\
--name executionWithHappyPathMockedServices \\
--state-machine arn:aws:states:`region`:`account-id`:stateMachine:LambdaSQSIntegration#HappyPath`
```
3. View the state machine execution response.
The response to calling `StartExecution` using a mocked service integration test is same as the response to calling `StartExecution` normally, which returns the execution ARN and start date.
The following is an example response to calling `StartExecution` using the
mocked service integration
test:
```
`{
"startDate":"2022-01-28T15:03:16.981000-05:00",
"executionArn":"arn:aws:states:`region`:`account-id`:execution:LambdaSQSIntegration:executionWithHappyPathMockedServices"
}`
```
4. Check the
execution's
results by making a `[ListExecutions](https://docs.aws.amazon.com/step-functions/latest/apireference/API_ListExecutions.html)`, `[DescribeExecution](https://docs.aws.amazon.com/step-functions/latest/apireference/API_DescribeExecution.html)`, or
`[GetExecutionHistory](https://docs.aws.amazon.com/step-functions/latest/apireference/API_GetExecutionHistory.html)` API call.
```
`aws stepfunctions get-execution-history \\
--endpoint http://localhost:8083 \\
--execution-arn arn:aws:states:`region`:`account-id`:execution:LambdaSQSIntegration:executionWithHappyPathMockedServices`
```
The following example demonstrates parts of a response to calling
`GetExecutionHistory` using the execution ARN from the
example
response shown in step 2. In this example, the output of
`LambdaState` and `SQSState` is the mock data defined in
`MockedLambdaSuccess` and `MockedSQSSuccess` in the [mock configuration file](#create-mock-config-file). In addition, the
mocked data is used the same way that data returned by performing actual service
integration calls would be used.
Also,
in this example, the output from `LambdaState` is passed onto
`SQSState` as input.
```
`{
"events": [
...
{
"timestamp": "2021-12-02T19:39:48.988000+00:00",
"type": "TaskStateEntered",
"id": 2,
"previousEventId": 0,
"stateEnteredEventDetails": {
"name": "LambdaState",
"input": "{}",
"inputDetails": {
"truncated": false
}
}
},
...
{
"timestamp": "2021-11-25T23:39:10.587000+00:00",
"type": "LambdaFunctionSucceeded",
"id": 5,
"previousEventId": 4,
"lambdaFunctionSucceededEventDetails": {
"output": "{\\"statusCode\\":200,\\"body\\":\\"\\\\\\"Hello from Lambda!\\\\\\"\\"}",
"outputDetails": {
"truncated": false
}
}
},
...
"timestamp": "2021-12-02T19:39:49.464000+00:00",
"type": "TaskStateEntered",
"id": 7,
"previousEventId": 6,
"stateEnteredEventDetails": {
"name": "SQSState",
"input": "{\\"statusCode\\":200,\\"body\\":\\"\\\\\\"Hello from Lambda!\\\\\\"\\"}",
"inputDetails": {
"truncated": false
}
}
},
...
{
"timestamp": "2021-11-25T23:39:10.652000+00:00",
"type": "TaskSucceeded",
"id": 10,
"previousEventId": 9,
"taskSucceededEventDetails": {
"resourceType": "sqs",
"resource": "sendMessage",
"output": "{\\"MD5OfMessageBody\\":\\"3bcb6e8e-7h85-4375-b0bc-1a59812c6e51\\",\\"MessageId\\":\\"3bcb6e8e-8b51-4375-b0bc-1a59812c6e51\\"}",
"outputDetails": {
"truncated": false
}
}
},
...
]
}`
```
## Configuration file for mocked service integrations in Step Functions
###### Step Functions Local is unsupported
Step Functions Local does **not** provide feature parity and is **unsupported**.
You might consider third party solutions that emulate Step Functions for testing
purposes.
As an alternative to Step Functions Local, you can use the TestState API to unit test your state machine logic before deploying to your AWS account. For more information, see [Testing state machines with TestState API](https://docs.aws.amazon.com/step-functions/latest/dg/test-state-isolation.html).
To use mocked service integrations, you must first create a mock configuration file named `MockConfigFile.json` containing your mock configurations. Then provide Step Functions Local with the mock configuration file. This configuration file defines test cases, which contain mock states that use mocked service integration responses. The following section contains information about the structure of mock configuration that includes the mock states and mocked responses:
### Mock configuration file structure
A mock configuration is a JSON object containing the following top-level fields:
* `StateMachines` - The fields of this object represent state machines configured to use mocked service integrations.
* `MockedResponse` - The fields of this object represent mocked responses for service integration calls.
The following is an example of a mock configuration
file which includes a `StateMachine` definition and `MockedResponse`.
```
`{
"StateMachines":{
"LambdaSQSIntegration":{
"TestCases":{
"HappyPath":{
"LambdaState":"MockedLambdaSuccess",
"SQSState":"MockedSQSSuccess"
},
"RetryPath":{
"LambdaState":"MockedLambdaRetry",
"SQSState":"MockedSQSSuccess"
},
"HybridPath":{
"LambdaState":"MockedLambdaSuccess"
}
}
}
},
"MockedResponses":{
"MockedLambdaSuccess":{
"0":{
"Return":{
"StatusCode":200,
"Payload":{
"StatusCode":200,
"body":"Hello from Lambda!"
}
}
}
},
"LambdaMockedResourceNotReady":{
"0":{
"Throw":{
"Error":"Lambda.ResourceNotReadyException",
"Cause":"Lambda resource is not ready."
}
}
},
"MockedSQSSuccess":{
"0":{
"Return":{
"MD5OfMessageBody":"3bcb6e8e-7h85-4375-b0bc-1a59812c6e51",
"MessageId":"3bcb6e8e-8b51-4375-b0bc-1a59812c6e51"
}
}
},
"MockedLambdaRetry":{
"0":{
"Throw":{
"Error":"Lambda.ResourceNotReadyException",
"Cause":"Lambda resource is not ready."
}
},
"1-2":{
"Throw":{
"Error":"Lambda.TimeoutException",
"Cause":"Lambda timed out."
}
},
"3":{
"Return":{
"StatusCode":200,
"Payload":{
"StatusCode":200,
"body":"Hello from Lambda!"
}
}
}
}
}
}`
```
#### Mock configuration field reference
The following sections explain the top-level object fields that you must define in your mock configuration.
* [StateMachines](#mock-cfg-sm-sect)
* [MockedResponses](#mock-cfg-mckd-resp-sect)
##### StateMachines
The `StateMachines` object defines which state machines will use mocked service integrations. The configuration for each state machine is represented as a top-level field of `StateMachines`. The field name is the name of the state machine and value is an object containing a single field named `TestCases`, whose fields represent test cases of that state machine.
The following syntax shows a state machine with two test cases:
```
`"MyStateMachine": {
"TestCases": {
"HappyPath": {
...
},
"SadPath": {
...
}
}`
```
##### TestCases
The fields of `TestCases` represent individual test cases for the state machine. The name of each test case must be unique per state machine and the value of each test case is an object specifying a mocked response to use for Task states in the state machine.
The following example of a `TestCase` links two `Task` states to two `MockedResponses`:
```
`"HappyPath": {
"SomeTaskState": "SomeMockedResponse",
"AnotherTaskState": "AnotherMockedResponse"
}`
```
##### MockedResponses
`MockedResponses` is an object containing multiple mocked response objects with unique field names. A mocked response object defines the successful result or error output for each invocation of a mocked Task state. You specify the invocation number using individual integer strings, such as “0”, “1”, “2”, and “3” or an inclusive range of integers, such as “0-1”, “2-3”.
When you mock a Task, you must specify a mocked response for every invocation. A response must contain a single field named `Return` or `Throw` whose value is the result or error output for the mocked Task invocation. If you do not specify a mocked response, the state machine execution will fail.
The following is an example of a `MockedResponse` with `Throw` and `Return` objects. In this example, the first three times the state machine is run, the response specified in `"0-2"` is returned, and the fourth time the state machine runs, the response specified in `"3"` is returned.
```
`"SomeMockedResponse": {
"0-2": {
"Throw": {
...
}
},
"3": {
"Return": {
...
}
}
}`
```
###### Note
If you are using a `Map` state, and want to ensure predictable responses for the `Map` state, set the value of `maxConcurrency` to 1. If you set a value greater than 1, Step Functions Local will run multiple iterations concurrently, which will cause the overall execution order of states across iterations to be unpredictable. This may further cause Step Functions Local to use different mocked responses for iteration states from one execution to the next.
##### Return
`Return` is represented as a field of the `MockedResponse` objects. It specifies the successful result of a mocked Task state.
The following is an example of a `Return` object that contains a mocked response for calling [`Invoke`](https://docs.aws.amazon.com/lambda/latest/dg/API_Invoke.html) on a Lambda function:
```
`"Return": {
"StatusCode": 200,
"Payload": {
"StatusCode": 200,
"body": "Hello from Lambda!"
}
}`
```
##### Throw
`Throw` is represented as a field of the `MockedResponse`
objects. It specifies the [error output](./concepts-error-handling.html) of
a failed Task. The value of `Throw` must be an object containing an
`Error` and `Cause` fields with string values. In addition, the
string value you specify in `Error` field in the `MockConfigFile.json` must match the errors handled in the `Retry` and `Catch`
sections of your state machine.
The following is an example of a `Throw` object that contains a mocked response for calling [`Invoke`](https://docs.aws.amazon.com/lambda/latest/dg/API_Invoke.html) on a Lambda function:
```
`"Throw": {
"Error": "Lambda.TimeoutException",
"Cause": "Lambda timed out."
}`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Tutorial: Testing using Step Functions and AWS SAM CLI Local
Versions and aliases
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.