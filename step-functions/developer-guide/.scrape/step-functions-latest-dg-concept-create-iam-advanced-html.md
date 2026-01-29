---
url: https://docs.aws.amazon.com/step-functions/latest/dg/concept-create-iam-advanced.html
title: Creating granular permissions for non-admin users in Step Functions
word_count: 530
filtered: true
elements_removed: 0
density_score: 0.83
---

Creating granular permissions for non-admin users in Step Functions - AWS Step Functions
Creating granular permissions for non-admin users in Step Functions - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/dg/step-functions-dg.pdf#concept-create-iam-advanced)
[Service-Level Permissions](#concept-create-iam-advanced-service)[State
Machine-Level
Permissions](#concept-create-iam-advanced-state)[Execution-Level
Permissions](#concept-create-iam-advanced-execution)[Activity-Level
Permissions](#concept-create-iam-advanced-activity)
# Creating granular permissions for non-admin users in Step Functions
The default managed policies in IAM, such as `ReadOnly`, don't fully cover
all types of AWS Step Functions permissions. This section describes these different types of permissions
and provides some example configurations.
Step Functions has four categories of permissions. Depending on what access you want to provide to
a user, you can control access by using permissions in these categories.
[Service-Level Permissions](#concept-create-iam-advanced-service)
Apply to components of the API that do **not** act on a specific resource.
[State
Machine-Level
Permissions](#concept-create-iam-advanced-state)
Apply to all API components that act on a specific state machine.
[Execution-Level
Permissions](#concept-create-iam-advanced-execution)
Apply to all API components that act on a specific execution.
[Activity-Level
Permissions](#concept-create-iam-advanced-activity)
Apply to all API components that act on a specific activity or on a particular
instance of an activity.
## Service-Level Permissions
This permission level applies to all API actions that do **not** act on a specific resource. These include
`[CreateStateMachine](https://docs.aws.amazon.com/step-functions/latest/apireference/API_CreateStateMachine.html)`,
`[CreateActivity](https://docs.aws.amazon.com/step-functions/latest/apireference/API_CreateActivity.html)`,
`[ListStateMachines](https://docs.aws.amazon.com/step-functions/latest/apireference/API_ListStateMachines.html)`,
`[ListActivities](https://docs.aws.amazon.com/step-functions/latest/apireference/API_ListActivities.html)`,
and `[ValidateStateMachineDefinition](https://docs.aws.amazon.com/step-functions/latest/apireference/API_ValidateStateMachineDefinition.html)`.
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Action": [
"states:ListStateMachines",
"states:ListActivities",
"states:CreateStateMachine",
"states:CreateActivity",
"states:ValidateStateMachineDefinition"
],
"Resource": [
"arn:aws:states:\*:\*:\*"
]
},
{
"Effect": "Allow",
"Action": [
"iam:PassRole"
],
"Resource": [
"arn:aws:iam::123456789012:role/my-execution-role"
]
}
]
}`
`
```
## State
Machine-Level
Permissions
This permission level applies to all API actions that act on a specific state machine. These API operations require the Amazon Resource Name (ARN) of the
state machine as part of the request, such as `[DeleteStateMachine](https://docs.aws.amazon.com/step-functions/latest/apireference/API_DeleteStateMachine.html)`, `[DescribeStateMachine](https://docs.aws.amazon.com/step-functions/latest/apireference/API_DescribeStateMachine.html)`, `[StartExecution](https://docs.aws.amazon.com/step-functions/latest/apireference/API_StartExecution.html)`, and `[ListExecutions](https://docs.aws.amazon.com/step-functions/latest/apireference/API_ListExecutions.html)`.
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Action": [
"states:DescribeStateMachine",
"states:StartExecution",
"states:DeleteStateMachine",
"states:ListExecutions",
"states:UpdateStateMachine",
"states:TestState",
"states:RevealSecrets"
],
"Resource": [
"arn:aws:states:\*:\*:stateMachine:StateMachinePrefix\*"
]
}
]
}`
`
```
## Execution-Level
Permissions
This permission level applies to all the API actions that act on a specific execution.
These API operations require the ARN of the execution as part of the request, such as
`[DescribeExecution](https://docs.aws.amazon.com/step-functions/latest/apireference/API_DescribeExecution.html)`, `[GetExecutionHistory](https://docs.aws.amazon.com/step-functions/latest/apireference/API_GetExecutionHistory.html)`, and `[StopExecution](https://docs.aws.amazon.com/step-functions/latest/apireference/API_StopExecution.html)`.
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Action": [
"states:DescribeExecution",
"states:DescribeStateMachineForExecution",
"states:GetExecutionHistory",
"states:StopExecution"
],
"Resource": [
"arn:aws:states:\*:\*:execution:\*:ExecutionPrefix\*"
]
}
]
}`
`
```
## Activity-Level
Permissions
This permission level applies to all the API actions that act on a specific activity
or on a particular instance of it. These API operations require the ARN of the activity or the
token of the instance as part of the request, such as `[DeleteActivity](https://docs.aws.amazon.com/step-functions/latest/apireference/API_DeleteActivity.html)`, `[DescribeActivity](https://docs.aws.amazon.com/step-functions/latest/apireference/API_DescribeActivity.html)`, `[GetActivityTask](https://docs.aws.amazon.com/step-functions/latest/apireference/API_GetActivityTask.html)`, and `[SendTaskHeartbeat](https://docs.aws.amazon.com/step-functions/latest/apireference/API_SendTaskHeartbeat.html)`.
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Action": [
"states:DescribeActivity",
"states:DeleteActivity",
"states:GetActivityTask",
"states:SendTaskHeartbeat"
],
"Resource": [
"arn:aws:states:\*:\*:activity:ActivityPrefix\*"
]
}
]
}`
`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Creating a state machine IAM role
Accessing cross-account AWS resources
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.