---
url: https://docs.aws.amazon.com/step-functions/latest/apireference/API_MapRunListItem.html
title: API MapRunListItem.html
word_count: 127
filtered: true
elements_removed: 0
density_score: 0.93
---

MapRunListItem - AWS Step Functions
MapRunListItem - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/apireference/step-functions-api.pdf#API_MapRunListItem)
[Contents](#API_MapRunListItem_Contents)[See Also](#API_MapRunListItem_SeeAlso)
## Contents
**
executionArn
**
The `executionArn` of the execution from which the Map Run was started.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 256.
Required: Yes
**
mapRunArn
**
The Amazon Resource Name (ARN) of the Map Run.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 2000.
Required: Yes
**
startDate
**
The date on which the Map Run started.
Type: Timestamp
Required: Yes
**
stateMachineArn
**
The Amazon Resource Name (ARN) of the executed state machine.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 256.
Required: Yes
**
stopDate
**
The date on which the Map Run stopped.
Type: Timestamp
Required: No