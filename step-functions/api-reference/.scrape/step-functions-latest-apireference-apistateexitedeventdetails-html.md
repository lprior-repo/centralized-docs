---
url: https://docs.aws.amazon.com/step-functions/latest/apireference/API_StateExitedEventDetails.html
title: StateExitedEventDetails
word_count: 212
filtered: true
elements_removed: 0
density_score: 0.93
---

StateExitedEventDetails - AWS Step Functions
StateExitedEventDetails - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/apireference/step-functions-api.pdf#API_StateExitedEventDetails)
[Contents](#API_StateExitedEventDetails_Contents)[See Also](#API_StateExitedEventDetails_SeeAlso)
# StateExitedEventDetails
Contains details about an exit from a state during an execution.
## Contents
**
name
**
The name of the state.
A name must *not* contain:
* white space
* brackets `&lt; &gt; { } [ ]`
* wildcard characters `? \*`
* special characters `" # % \\ ^ | \~ ` $ &amp;&amp; , ; : /`
* control characters (`U+0000-001F`, `U+007F-009F`, `U+FFFE-FFFF`)
* surrogates (`U+D800-DFFF`)
* invalid characters (` U+10FFFF`)
To enable logging with CloudWatch Logs, the name should only contain 0-9, A-Z, a-z, - and \_.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 80.
Required: Yes
**
assignedVariables
**
Map of variable name and value as a serialized JSON representation.
Type: String to string map
Required: No
**
assignedVariablesDetails
**
Provides details about input or output in an execution history event.
Type: [AssignedVariablesDetails](./API_AssignedVariablesDetails.html) object
Required: No
**
output
**
The JSON output data of the state. Length constraints apply to the payload size, and are expressed as bytes in UTF-8 encoding.
Type: String
Length Constraints: Maximum length of 262144.
Required: No
**
outputDetails
**
Contains details about the output of an execution history event.
Type: [HistoryEventExecutionDataDetails](./API_HistoryEventExecutionDataDetails.html) object
Required: No