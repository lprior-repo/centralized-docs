---
url: https://docs.aws.amazon.com/step-functions/latest/apireference/API_MapRunExecutionCounts.html
title: MapRunExecutionCounts
word_count: 385
filtered: true
elements_removed: 0
density_score: 0.80
---

MapRunExecutionCounts - AWS Step Functions
MapRunExecutionCounts - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/apireference/step-functions-api.pdf#API_MapRunExecutionCounts)
[Contents](#API_MapRunExecutionCounts_Contents)[See Also](#API_MapRunExecutionCounts_SeeAlso)
# MapRunExecutionCounts
Contains details about all of the child workflow executions started by a Map Run.
## Contents
**
aborted
**
The total number of child workflow executions that were started by a Map Run and were running, but were either stopped by the user or by Step Functions because the Map Run failed.
Type: Long
Valid Range: Minimum value of 0.
Required: Yes
**
failed
**
The total number of child workflow executions that were started by a Map Run, but have failed.
Type: Long
Valid Range: Minimum value of 0.
Required: Yes
**
pending
**
The total number of child workflow executions that were started by a Map Run, but haven't started executing yet.
Type: Long
Valid Range: Minimum value of 0.
Required: Yes
**
resultsWritten
**
Returns the count of child workflow executions whose results were written by `ResultWriter`. For more information, see [ResultWriter](https://docs.aws.amazon.com/step-functions/latest/dg/input-output-resultwriter.html) in the *
AWS Step Functions Developer Guide*.
Type: Long
Valid Range: Minimum value of 0.
Required: Yes
**
running
**
The total number of child workflow executions that were started by a Map Run and are currently in-progress.
Type: Long
Valid Range: Minimum value of 0.
Required: Yes
**
succeeded
**
The total number of child workflow executions that were started by a Map Run and have completed successfully.
Type: Long
Valid Range: Minimum value of 0.
Required: Yes
**
timedOut
**
The total number of child workflow executions that were started by a Map Run and have timed out.
Type: Long
Valid Range: Minimum value of 0.
Required: Yes
**
total
**
The total number of child workflow executions that were started by a Map Run.
Type: Long
Valid Range: Minimum value of 0.
Required: Yes
**
failuresNotRedrivable
**
The number of `FAILED`, `ABORTED`, or `TIMED\_OUT` child workflow executions that cannot be redriven because their execution status is terminal. For example, child workflows with an execution status of `FAILED`, `ABORTED`, or `TIMED\_OUT` and a `redriveStatus` of `NOT\_REDRIVABLE`.
Type: Long
Required: No
**
pendingRedrive
**
The number of unsuccessful child workflow executions currently waiting to be redriven. The status of these child workflow executions could be `FAILED`, `ABORTED`, or `TIMED\_OUT` in the original execution attempt or a previous redrive attempt.
Type: Long
Required: No