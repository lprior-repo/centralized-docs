---
url: https://docs.aws.amazon.com/step-functions/latest/apireference/API_MockErrorOutput.html
title: MockErrorOutput
word_count: 96
filtered: true
elements_removed: 0
density_score: 0.93
---

MockErrorOutput - AWS Step Functions
MockErrorOutput - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/apireference/step-functions-api.pdf#API_MockErrorOutput)
[Contents](#API_MockErrorOutput_Contents)[See Also](#API_MockErrorOutput_SeeAlso)
# MockErrorOutput
A JSON object that contains a mocked error.
## Contents
**
cause
**
A string containing the cause of the exception thrown when executing the state's logic.
Type: String
Length Constraints: Minimum length of 0. Maximum length of 32768.
Required: No
**
error
**
A string denoting the error code of the exception thrown when invoking the tested state. This field is required if `mock.errorOutput` is specified.
Type: String
Length Constraints: Minimum length of 0. Maximum length of 256.
Required: No