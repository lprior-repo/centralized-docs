---
url: https://docs.aws.amazon.com/step-functions/latest/apireference/API_StateMachineVersionListItem.html
title: StateMachineVersionListItem
word_count: 90
filtered: true
elements_removed: 0
density_score: 0.93
---

StateMachineVersionListItem - AWS Step Functions
StateMachineVersionListItem - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/apireference/step-functions-api.pdf#API_StateMachineVersionListItem)
[Contents](#API_StateMachineVersionListItem_Contents)[See Also](#API_StateMachineVersionListItem_SeeAlso)
# StateMachineVersionListItem
Contains details about a specific state machine version.
## Contents
**
creationDate
**
The creation date of a state machine version.
Type: Timestamp
Required: Yes
**
stateMachineVersionArn
**
The Amazon Resource Name (ARN) that identifies a state machine version. The version ARN is a combination of state machine ARN and the version number separated by a colon (:). For example, `stateMachineARN:1`.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 2000.
Required: Yes