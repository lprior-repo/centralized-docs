---
url: https://docs.aws.amazon.com/step-functions/latest/apireference/API_HistoryEvent.html
title: HistoryEvent
word_count: 753
filtered: true
elements_removed: 0
density_score: 0.84
---

HistoryEvent - AWS Step Functions
HistoryEvent - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/apireference/step-functions-api.pdf#API_HistoryEvent)
[Contents](#API_HistoryEvent_Contents)[See Also](#API_HistoryEvent_SeeAlso)
# HistoryEvent
Contains details about the events of an execution.
## Contents
**
id
**
The id of the event. Events are numbered sequentially, starting at one.
Type: Long
Required: Yes
**
timestamp
**
The date and time the event occurred, expressed in seconds and fractional milliseconds since the Unix epoch, which is defined as January 1, 1970, at 00:00:00 Coordinated Universal Time (UTC).
Type: Timestamp
Required: Yes
**
type
**
The type of the event.
Type: String
Valid Values: `ActivityFailed | ActivityScheduled | ActivityScheduleFailed | ActivityStarted | ActivitySucceeded | ActivityTimedOut | ChoiceStateEntered | ChoiceStateExited | ExecutionAborted | ExecutionFailed | ExecutionStarted | ExecutionSucceeded | ExecutionTimedOut | FailStateEntered | LambdaFunctionFailed | LambdaFunctionScheduled | LambdaFunctionScheduleFailed | LambdaFunctionStarted | LambdaFunctionStartFailed | LambdaFunctionSucceeded | LambdaFunctionTimedOut | MapIterationAborted | MapIterationFailed | MapIterationStarted | MapIterationSucceeded | MapStateAborted | MapStateEntered | MapStateExited | MapStateFailed | MapStateStarted | MapStateSucceeded | ParallelStateAborted | ParallelStateEntered | ParallelStateExited | ParallelStateFailed | ParallelStateStarted | ParallelStateSucceeded | PassStateEntered | PassStateExited | SucceedStateEntered | SucceedStateExited | TaskFailed | TaskScheduled | TaskStarted | TaskStartFailed | TaskStateAborted | TaskStateEntered | TaskStateExited | TaskSubmitFailed | TaskSubmitted | TaskSucceeded | TaskTimedOut | WaitStateAborted | WaitStateEntered | WaitStateExited | MapRunAborted | MapRunFailed | MapRunStarted | MapRunSucceeded | ExecutionRedriven | MapRunRedriven | EvaluationFailed`
Required: Yes
**
activityFailedEventDetails
**
Type: [ActivityFailedEventDetails](./API_ActivityFailedEventDetails.html) object
Required: No
**
activityScheduledEventDetails
**
Type: [ActivityScheduledEventDetails](./API_ActivityScheduledEventDetails.html) object
Required: No
**
activityScheduleFailedEventDetails
**
Contains details about an activity schedule event that failed during an execution.
Type: [ActivityScheduleFailedEventDetails](./API_ActivityScheduleFailedEventDetails.html) object
Required: No
**
activityStartedEventDetails
**
Type: [ActivityStartedEventDetails](./API_ActivityStartedEventDetails.html) object
Required: No
**
activitySucceededEventDetails
**
Type: [ActivitySucceededEventDetails](./API_ActivitySucceededEventDetails.html) object
Required: No
**
activityTimedOutEventDetails
**
Type: [ActivityTimedOutEventDetails](./API_ActivityTimedOutEventDetails.html) object
Required: No
**
evaluationFailedEventDetails
**
Contains details about an evaluation failure that occurred while processing a state.
Type: [EvaluationFailedEventDetails](./API_EvaluationFailedEventDetails.html) object
Required: No
**
executionAbortedEventDetails
**
Type: [ExecutionAbortedEventDetails](./API_ExecutionAbortedEventDetails.html) object
Required: No
**
executionFailedEventDetails
**
Type: [ExecutionFailedEventDetails](./API_ExecutionFailedEventDetails.html) object
Required: No
**
executionRedrivenEventDetails
**
Contains details about the redrive attempt of an execution.
Type: [ExecutionRedrivenEventDetails](./API_ExecutionRedrivenEventDetails.html) object
Required: No
**
executionStartedEventDetails
**
Type: [ExecutionStartedEventDetails](./API_ExecutionStartedEventDetails.html) object
Required: No
**
executionSucceededEventDetails
**
Type: [ExecutionSucceededEventDetails](./API_ExecutionSucceededEventDetails.html) object
Required: No
**
executionTimedOutEventDetails
**
Type: [ExecutionTimedOutEventDetails](./API_ExecutionTimedOutEventDetails.html) object
Required: No
**
lambdaFunctionFailedEventDetails
**
Type: [LambdaFunctionFailedEventDetails](./API_LambdaFunctionFailedEventDetails.html) object
Required: No
**
lambdaFunctionScheduledEventDetails
**
Type: [LambdaFunctionScheduledEventDetails](./API_LambdaFunctionScheduledEventDetails.html) object
Required: No
**
lambdaFunctionScheduleFailedEventDetails
**
Type: [LambdaFunctionScheduleFailedEventDetails](./API_LambdaFunctionScheduleFailedEventDetails.html) object
Required: No
**
lambdaFunctionStartFailedEventDetails
**
Contains details about a lambda function that failed to start during an execution.
Type: [LambdaFunctionStartFailedEventDetails](./API_LambdaFunctionStartFailedEventDetails.html) object
Required: No
**
lambdaFunctionSucceededEventDetails
**
Contains details about a Lambda function that terminated successfully during an
execution.
Type: [LambdaFunctionSucceededEventDetails](./API_LambdaFunctionSucceededEventDetails.html) object
Required: No
**
lambdaFunctionTimedOutEventDetails
**
Type: [LambdaFunctionTimedOutEventDetails](./API_LambdaFunctionTimedOutEventDetails.html) object
Required: No
**
mapIterationAbortedEventDetails
**
Contains details about an iteration of a Map state that was aborted.
Type: [MapIterationEventDetails](./API_MapIterationEventDetails.html) object
Required: No
**
mapIterationFailedEventDetails
**
Contains details about an iteration of a Map state that failed.
Type: [MapIterationEventDetails](./API_MapIterationEventDetails.html) object
Required: No
**
mapIterationStartedEventDetails
**
Contains details about an iteration of a Map state that was started.
Type: [MapIterationEventDetails](./API_MapIterationEventDetails.html) object
Required: No
**
mapIterationSucceededEventDetails
**
Contains details about an iteration of a Map state that succeeded.
Type: [MapIterationEventDetails](./API_MapIterationEventDetails.html) object
Required: No
**
mapRunFailedEventDetails
**
Contains error and cause details about a Map Run that failed.
Type: [MapRunFailedEventDetails](./API_MapRunFailedEventDetails.html) object
Required: No
**
mapRunRedrivenEventDetails
**
Contains details about the redrive attempt of a Map Run.
Type: [MapRunRedrivenEventDetails](./API_MapRunRedrivenEventDetails.html) object
Required: No
**
mapRunStartedEventDetails
**
Contains details, such as `mapRunArn`, and the start date and time of a Map Run. `mapRunArn` is the Amazon Resource Name (ARN) of the Map Run that was started.
Type: [MapRunStartedEventDetails](./API_MapRunStartedEventDetails.html) object
Required: No
**
mapStateStartedEventDetails
**
Contains details about Map state that was started.
Type: [MapStateStartedEventDetails](./API_MapStateStartedEventDetails.html) object
Required: No
**
previousEventId
**
The id of the previous event.
Type: Long
Required: No
**
stateEnteredEventDetails
**
Type: [StateEnteredEventDetails](./API_StateEnteredEventDetails.html) object
Required: No
**
stateExitedEventDetails
**
Type: [StateExitedEventDetails](./API_StateExitedEventDetails.html) object
Required: No
**
taskFailedEventDetails
**
Contains details about the failure of a task.
Type: [TaskFailedEventDetails](./API_TaskFailedEventDetails.html) object
Required: No
**
taskScheduledEventDetails
**
Contains details about a task that was scheduled.
Type: [TaskScheduledEventDetails](./API_TaskScheduledEventDetails.html) object
Required: No
**
taskStartedEventDetails
**
Contains details about a task that was started.
Type: [TaskStartedEventDetails](./API_TaskStartedEventDetails.html) object
Required: No
**
taskStartFailedEventDetails
**
Contains details about a task that failed to start.
Type: [TaskStartFailedEventDetails](./API_TaskStartFailedEventDetails.html) object
Required: No
**
taskSubmitFailedEventDetails
**
Contains details about a task that where the submit failed.
Type: [TaskSubmitFailedEventDetails](./API_TaskSubmitFailedEventDetails.html) object
Required: No
**
taskSubmittedEventDetails
**
Contains details about a submitted task.
Type: [TaskSubmittedEventDetails](./API_TaskSubmittedEventDetails.html) object
Required: No
**
taskSucceededEventDetails
**
Contains details about a task that succeeded.
Type: [TaskSucceededEventDetails](./API_TaskSucceededEventDetails.html) object
Required: No
**
taskTimedOutEventDetails
**
Contains details about a task that timed out.
Type: [TaskTimedOutEventDetails](./API_TaskTimedOutEventDetails.html) object
Required: No