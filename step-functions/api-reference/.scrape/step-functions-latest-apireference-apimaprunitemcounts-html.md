---
url: https://docs.aws.amazon.com/step-functions/latest/apireference/API_MapRunItemCounts.html
title: MapRunItemCounts
word_count: 353
filtered: true
elements_removed: 0
density_score: 0.80
---

MapRunItemCounts - AWS Step Functions
MapRunItemCounts - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/apireference/step-functions-api.pdf#API_MapRunItemCounts)
[Contents](#API_MapRunItemCounts_Contents)[See Also](#API_MapRunItemCounts_SeeAlso)
# MapRunItemCounts
Contains details about items that were processed in all of the child workflow executions that were started by a Map Run.
## Contents
**
aborted
**
The total number of items processed in child workflow executions that were either stopped by the user or by Step Functions, because the Map Run failed.
Type: Long
Valid Range: Minimum value of 0.
Required: Yes
**
failed
**
The total number of items processed in child workflow executions that have failed.
Type: Long
Valid Range: Minimum value of 0.
Required: Yes
**
pending
**
The total number of items to process in child workflow executions that haven't started running yet.
Type: Long
Valid Range: Minimum value of 0.
Required: Yes
**
resultsWritten
**
Returns the count of items whose results were written by `ResultWriter`. For more information, see [ResultWriter](https://docs.aws.amazon.com/step-functions/latest/dg/input-output-resultwriter.html) in the *
AWS Step Functions Developer Guide*.
Type: Long
Valid Range: Minimum value of 0.
Required: Yes
**
running
**
The total number of items being processed in child workflow executions that are currently in-progress.
Type: Long
Valid Range: Minimum value of 0.
Required: Yes
**
succeeded
**
The total number of items processed in child workflow executions that have completed successfully.
Type: Long
Valid Range: Minimum value of 0.
Required: Yes
**
timedOut
**
The total number of items processed in child workflow executions that have timed out.
Type: Long
Valid Range: Minimum value of 0.
Required: Yes
**
total
**
The total number of items processed in all the child workflow executions started by a Map Run.
Type: Long
Valid Range: Minimum value of 0.
Required: Yes
**
failuresNotRedrivable
**
The number of `FAILED`, `ABORTED`, or `TIMED\_OUT` items in child workflow executions that cannot be redriven because the execution status of those child workflows is terminal. For example, child workflows with an execution status of `FAILED`, `ABORTED`, or `TIMED\_OUT` and a `redriveStatus` of `NOT\_REDRIVABLE`.
Type: Long
Required: No
**
pendingRedrive
**
The number of unsuccessful items in child workflow executions currently waiting to be redriven.
Type: Long
Required: No