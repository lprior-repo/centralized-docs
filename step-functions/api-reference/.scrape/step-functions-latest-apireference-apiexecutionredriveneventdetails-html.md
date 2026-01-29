---
url: https://docs.aws.amazon.com/step-functions/latest/apireference/API_ExecutionRedrivenEventDetails.html
title: API ExecutionRedrivenEventDetails.html
word_count: 59
filtered: true
elements_removed: 0
density_score: 0.93
---

ExecutionRedrivenEventDetails - AWS Step Functions
ExecutionRedrivenEventDetails - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/apireference/step-functions-api.pdf#API_ExecutionRedrivenEventDetails)
[Contents](#API_ExecutionRedrivenEventDetails_Contents)[See Also](#API_ExecutionRedrivenEventDetails_SeeAlso)
## Contents
**
redriveCount
**
The number of times you've redriven an execution. If you have not yet redriven an execution, the `redriveCount` is 0. This count is not updated for redrives that failed to start or are pending to be redriven.
Type: Integer
Required: No