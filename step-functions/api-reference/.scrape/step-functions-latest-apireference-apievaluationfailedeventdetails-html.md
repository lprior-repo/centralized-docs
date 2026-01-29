---
url: https://docs.aws.amazon.com/step-functions/latest/apireference/API_EvaluationFailedEventDetails.html
title: EvaluationFailedEventDetails
word_count: 169
filtered: true
elements_removed: 0
density_score: 0.83
---

EvaluationFailedEventDetails - AWS Step Functions
EvaluationFailedEventDetails - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/apireference/step-functions-api.pdf#API_EvaluationFailedEventDetails)
[Contents](#API_EvaluationFailedEventDetails_Contents)[See Also](#API_EvaluationFailedEventDetails_SeeAlso)
# EvaluationFailedEventDetails
Contains details about an evaluation failure that occurred while processing a state, for example, when a JSONata expression throws an error. This event will only be present in state machines that have ** QueryLanguage** set to JSONata, or individual states set to JSONata.
## Contents
**
state
**
The name of the state in which the evaluation error occurred.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 80.
Required: Yes
**
cause
**
A more detailed explanation of the cause of the failure.
Type: String
Length Constraints: Minimum length of 0. Maximum length of 32768.
Required: No
**
error
**
The error code of the failure.
Type: String
Length Constraints: Minimum length of 0. Maximum length of 256.
Required: No
**
location
**
The location of the field in the state in which the evaluation error occurred.
Type: String
Length Constraints: Minimum length of 0. Maximum length of 256.
Required: No