---
url: https://docs.aws.amazon.com/step-functions/latest/apireference/API_StateMachineAliasListItem.html
title: StateMachineAliasListItem
word_count: 90
filtered: true
elements_removed: 0
density_score: 0.93
---

StateMachineAliasListItem - AWS Step Functions
StateMachineAliasListItem - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/apireference/step-functions-api.pdf#API_StateMachineAliasListItem)
[Contents](#API_StateMachineAliasListItem_Contents)[See Also](#API_StateMachineAliasListItem_SeeAlso)
# StateMachineAliasListItem
Contains details about a specific state machine alias.
## Contents
**
creationDate
**
The creation date of a state machine alias.
Type: Timestamp
Required: Yes
**
stateMachineAliasArn
**
The Amazon Resource Name (ARN) that identifies a state machine alias. The alias ARN is a combination of state machine ARN and the alias name separated by a colon (:). For example, `stateMachineARN:PROD`.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 2000.
Required: Yes