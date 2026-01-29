---
url: https://docs.aws.amazon.com/step-functions/latest/apireference/API_MockInput.html
title: MockInput
word_count: 159
filtered: true
elements_removed: 0
density_score: 0.93
---

MockInput - AWS Step Functions
MockInput - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/apireference/step-functions-api.pdf#API_MockInput)
[Contents](#API_MockInput_Contents)[See Also](#API_MockInput_SeeAlso)
# MockInput
A JSON object that contains a mocked `result` or `errorOutput`.
## Contents
**
errorOutput
**
The mocked error output when calling TestState. When specified, the mocked response is returned as a JSON object that contains an `error` and `cause` field.
Type: [MockErrorOutput](./API_MockErrorOutput.html) object
Required: No
**
fieldValidationMode
**
Determines the level of strictness when validating mocked results against their respective API models. Values include:
* `STRICT`: All required fields must be present, and all present fields must conform to the API's schema.
* `PRESENT`: All present fields must conform to the API's schema.
* `NONE`: No validation is performed.
If no value is specified, the default value is `STRICT`.
Type: String
Valid Values: `STRICT | PRESENT | NONE`
Required: No
**
result
**
A JSON string containing the mocked result of the state invocation.
Type: String
Length Constraints: Maximum length of 262144.
Required: No