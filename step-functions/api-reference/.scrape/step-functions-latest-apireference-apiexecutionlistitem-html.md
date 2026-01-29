---
url: https://docs.aws.amazon.com/step-functions/latest/apireference/API_ExecutionListItem.html
title: API ExecutionListItem.html
word_count: 481
filtered: true
elements_removed: 0
density_score: 0.82
---

ExecutionListItem - AWS Step Functions
ExecutionListItem - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/apireference/step-functions-api.pdf#API_ExecutionListItem)
[Contents](#API_ExecutionListItem_Contents)[See Also](#API_ExecutionListItem_SeeAlso)
## Contents
**
executionArn
**
The Amazon Resource Name (ARN) that identifies the execution.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 256.
Required: Yes
**
name
**
The name of the execution.
A name must *not* contain:
* white space
* brackets `&lt; &gt; { } [ ]`
* wildcard characters `? \*`
* special characters `" # % \\ ^ | \~ ` $ &amp;&amp; , ; : /`
* control characters (`U+0000-001F`, `U+007F-009F`, `U+FFFE-FFFF`)
* surrogates (`U+D800-DFFF`)
* invalid characters (` U+10FFFF`)
To enable logging with CloudWatch Logs, the name should only contain 0-9, A-Z, a-z, - and \_.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 80.
Required: Yes
**
startDate
**
The date the execution started.
Type: Timestamp
Required: Yes
**
stateMachineArn
**
The Amazon Resource Name (ARN) of the state machine that ran the execution.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 256.
Required: Yes
**
status
**
The current status of the execution.
Type: String
Valid Values: `RUNNING | SUCCEEDED | FAILED | TIMED\_OUT | ABORTED | PENDING\_REDRIVE`
Required: Yes
**
itemCount
**
The total number of items processed in a child workflow execution. This field is returned only if `mapRunArn` was specified in the `ListExecutions` API action. If `stateMachineArn` was specified in `ListExecutions`, the `itemCount` field isn't returned.
Type: Integer
Valid Range: Minimum value of 0.
Required: No
**
mapRunArn
**
The Amazon Resource Name (ARN) of a Map Run. This field is returned only if `mapRunArn` was specified in the `ListExecutions` API action. If `stateMachineArn` was specified in `ListExecutions`, the `mapRunArn` isn't returned.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 2000.
Required: No
**
redriveCount
**
The number of times you've redriven an execution. If you have not yet redriven an execution, the `redriveCount` is 0. This count is only updated when you successfully redrive an execution.
Type: Integer
Required: No
**
redriveDate
**
The date the execution was last redriven.
Type: Timestamp
Required: No
**
stateMachineAliasArn
**
The Amazon Resource Name (ARN) of the state machine alias used to start an execution.
If the state machine execution was started with an unqualified ARN or a version ARN, it returns null.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 256.
Required: No
**
stateMachineVersionArn
**
The Amazon Resource Name (ARN) of the state machine version associated with the execution.
If the state machine execution was started with an unqualified ARN, it returns null.
If the execution was started using a `stateMachineAliasArn`, both the `stateMachineAliasArn` and `stateMachineVersionArn` parameters contain the respective values.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 256.
Required: No
**
stopDate
**
If the execution already ended, the date the execution stopped.
Type: Timestamp
Required: No