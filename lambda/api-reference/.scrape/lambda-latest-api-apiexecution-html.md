---
url: https://docs.aws.amazon.com/lambda/latest/api/API_Execution.html
title: API Execution.html
word_count: 176
filtered: true
elements_removed: 0
density_score: 0.86
---

Execution - AWS Lambda
Execution - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/api/lambda-api.pdf#API_Execution)
[Contents](#API_Execution_Contents)[See Also](#API_Execution_SeeAlso)
## Contents
**
DurableExecutionArn
**
The Amazon Resource Name (ARN) of the durable execution, if this execution is a durable execution.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 1024.
Pattern: `arn:([a-zA-Z0-9-]+):lambda:([a-zA-Z0-9-]+):(\\d{12}):function:([a-zA-Z0-9\_-]+):(\\$LATEST(?:\\.PUBLISHED)?|[0-9]+)/durable-execution/([a-zA-Z0-9\_-]+)/([a-zA-Z0-9\_-]+)`
Required: Yes
**
DurableExecutionName
**
The unique name of the durable execution, if one was provided when the execution was started.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 64.
Pattern: `[a-zA-Z0-9-\_]+`
Required: Yes
**
FunctionArn
**
The Amazon Resource Name (ARN) of the Lambda function.
Type: String
Length Constraints: Minimum length of 0. Maximum length of 10000.
Pattern: `arn:(aws[a-zA-Z-]\*)?:lambda:(eusc-)?[a-z]{2}((-gov)|(-iso([a-z]?)))?-[a-z]+-\\d{1}:\\d{12}:function:[a-zA-Z0-9-\_\\.]+(:(\\$LATEST(\\.PUBLISHED)?|[a-zA-Z0-9-\_]+))?`
Required: Yes
**
StartTimestamp
**
The date and time when the durable execution started, in [ISO-8601 format](https://www.w3.org/TR/NOTE-datetime) (YYYY-MM-DDThh:mm:ss.sTZD).
Type: Timestamp
Required: Yes
**
Status
**
The current status of the durable execution.
Type: String
Valid Values: `RUNNING | SUCCEEDED | FAILED | TIMED\_OUT | STOPPED`
Required: Yes
**
EndTimestamp
**
The date and time when the durable execution ended, in [ISO-8601 format](https://www.w3.org/TR/NOTE-datetime) (YYYY-MM-DDThh:mm:ss.sTZD).
Type: Timestamp
Required: No