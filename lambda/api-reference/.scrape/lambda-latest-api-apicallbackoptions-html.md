---
url: https://docs.aws.amazon.com/lambda/latest/api/API_CallbackOptions.html
title: CallbackOptions
word_count: 95
filtered: true
elements_removed: 0
density_score: 0.86
---

CallbackOptions - AWS Lambda
CallbackOptions - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/api/lambda-api.pdf#API_CallbackOptions)
[Contents](#API_CallbackOptions_Contents)[See Also](#API_CallbackOptions_SeeAlso)
# CallbackOptions
Configuration options for callback operations in durable executions, including timeout settings and retry behavior.
## Contents
**
HeartbeatTimeoutSeconds
**
The heartbeat timeout for the callback operation, in seconds. If not specified or set to 0, heartbeat timeout is disabled.
Type: Integer
Valid Range: Minimum value of 0.
Required: No
**
TimeoutSeconds
**
The timeout for the callback operation in seconds. If not specified or set to 0, the callback has no timeout.
Type: Integer
Valid Range: Minimum value of 0.
Required: No