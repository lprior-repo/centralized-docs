---
url: https://docs.aws.amazon.com/step-functions/latest/apireference/API_ValidateStateMachineDefinitionDiagnostic.html
title: ValidateStateMachineDefinitionDiagnostic
word_count: 373
filtered: true
elements_removed: 0
density_score: 0.88
---

ValidateStateMachineDefinitionDiagnostic - AWS Step Functions
ValidateStateMachineDefinitionDiagnostic - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/apireference/step-functions-api.pdf#API_ValidateStateMachineDefinitionDiagnostic)
[Contents](#API_ValidateStateMachineDefinitionDiagnostic_Contents)[See Also](#API_ValidateStateMachineDefinitionDiagnostic_SeeAlso)
# ValidateStateMachineDefinitionDiagnostic
Describes potential issues found during state machine validation. Rather than raise an
exception, validation will return a list of **diagnostic
elements** containing diagnostic information.
###### Note
The [ValidateStateMachineDefinitionlAPI](https://docs.aws.amazon.com/step-functions/latest/apireference/API_ValidateStateMachineDefinition.html) might add
new diagnostics in the future, adjust diagnostic codes, or change the message
wording. Your automated processes should only rely on the value of the **result** field value (OK, FAIL). Do **not** rely on the exact order, count, or
wording of diagnostic messages.
**List of warning codes**
NO\_DOLLAR
No `.$` on a field that appears to be a JSONPath or Intrinsic Function.
NO\_PATH
Field value looks like a path, but field name does not end with 'Path'.
PASS\_RESULT\_IS\_STATIC
Attempt to use a path in the result of a pass state.
**List of error codes**
INVALID\_JSON\_DESCRIPTION
JSON syntax problem found.
MISSING\_DESCRIPTION
Received a null or empty workflow input.
SCHEMA\_VALIDATION\_FAILED
Schema validation reported errors.
INVALID\_RESOURCE
The value of a Task-state resource field is invalid.
MISSING\_END\_STATE
The workflow does not have a terminal state.
DUPLICATE\_STATE\_NAME
The same state name appears more than once.
INVALID\_STATE\_NAME
The state name does not follow the naming convention.
STATE\_MACHINE\_NAME\_EMPTY
The state machine name has not been specified.
STATE\_MACHINE\_NAME\_INVALID
The state machine name does not follow the naming convention.
STATE\_MACHINE\_NAME\_TOO\_LONG
The state name exceeds the allowed length.
STATE\_MACHINE\_NAME\_ALREADY\_EXISTS
The state name already exists.
DUPLICATE\_LABEL\_NAME
A label name appears more than once.
INVALID\_LABEL\_NAME
You have provided an invalid label name.
MISSING\_TRANSITION\_TARGET
The value of "Next" field doesn't match a known state name.
TOO\_DEEPLY\_NESTED
The states are too deeply nested.
## Contents
**
code
**
Identifying code for the diagnostic.
Type: String
Required: Yes
**
message
**
Message describing the diagnostic condition.
Type: String
Required: Yes
**
severity
**
A value of `ERROR` means that you cannot create or update a state machine with this definition.
`WARNING` level diagnostics alert you to potential issues, but they will not prevent you from creating or updating your state machine.
Type: String
Valid Values: `ERROR | WARNING`
Required: Yes
**
location
**
Location of the issue in the state machine, if available.
For errors specific to a field, the location could be in the format: `/States/&lt;StateName&gt;/&lt;FieldName&gt;`, for example: `/States/FailState/ErrorPath`.
Type: String
Required: No