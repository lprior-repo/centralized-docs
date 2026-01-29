---
url: https://docs.aws.amazon.com/lambda/latest/api/API_CheckpointUpdatedExecutionState.html
title: CheckpointUpdatedExecutionState
word_count: 83
filtered: true
elements_removed: 0
density_score: 0.93
---

CheckpointUpdatedExecutionState - AWS Lambda
CheckpointUpdatedExecutionState - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/api/lambda-api.pdf#API_CheckpointUpdatedExecutionState)
[Contents](#API_CheckpointUpdatedExecutionState_Contents)[See Also](#API_CheckpointUpdatedExecutionState_SeeAlso)
# CheckpointUpdatedExecutionState
Contains operations that have been updated since the last checkpoint, such as completed asynchronous work like timers or callbacks.
## Contents
**
NextMarker
**
Indicates that more results are available. Use this value in a subsequent call to retrieve the next page of results.
Type: String
Required: No
**
Operations
**
A list of operations that have been updated since the last checkpoint.
Type: Array of [Operation](./API_Operation.html) objects
Required: No