---
url: https://docs.aws.amazon.com/step-functions/latest/apireference/API_InspectionData.html
title: InspectionData
word_count: 543
filtered: true
elements_removed: 0
density_score: 0.82
---

InspectionData - AWS Step Functions
InspectionData - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/apireference/step-functions-api.pdf#API_InspectionData)
[Contents](#API_InspectionData_Contents)[See Also](#API_InspectionData_SeeAlso)
# InspectionData
Contains additional details about the state's execution, including its input and output data processing flow, and HTTP request and response information.
## Contents
**
afterArguments
**
The input after Step Functions applies an Arguments filter. This event will only be present when QueryLanguage for the state machine or individual states is set to JSONata. For more info, see [Transforming data with Step Functions](https://docs.aws.amazon.com/step-functions/latest/dg/data-transform.html).
Type: String
Length Constraints: Maximum length of 262144.
Required: No
**
afterInputPath
**
The input after Step Functions applies the [InputPath](https://docs.aws.amazon.com/step-functions/latest/dg/input-output-inputpath-params.html#input-output-inputpath) filter. Not populated when QueryLanguage is JSONata.
Type: String
Length Constraints: Maximum length of 262144.
Required: No
**
afterItemBatcher
**
The effective input after the ItemBatcher filter is applied in a Map state.
Type: String
Length Constraints: Maximum length of 262144.
Required: No
**
afterItemSelector
**
An array containing the inputs for each Map iteration, transformed by the ItemSelector specified in a Map state.
Type: String
Length Constraints: Maximum length of 262144.
Required: No
**
afterItemsPath
**
The effective input after the ItemsPath filter is applied. Not populated when the QueryLanguage is JSONata.
Type: String
Length Constraints: Maximum length of 262144.
Required: No
**
afterItemsPointer
**
The effective input after the ItemsPointer filter is applied in a Map state.
Type: String
Length Constraints: Maximum length of 262144.
Required: No
**
afterParameters
**
The effective input after Step Functions applies the [Parameters](https://docs.aws.amazon.com/step-functions/latest/dg/input-output-inputpath-params.html#input-output-parameters) filter. Not populated when QueryLanguage is JSONata.
Type: String
Length Constraints: Maximum length of 262144.
Required: No
**
afterResultPath
**
The effective result combined with the raw state input after Step Functions applies the [ResultPath](https://docs.aws.amazon.com/step-functions/latest/dg/input-output-resultpath.html) filter. Not populated when QueryLanguage is JSONata.
Type: String
Length Constraints: Maximum length of 262144.
Required: No
**
afterResultSelector
**
The effective result after Step Functions applies the [ResultSelector](https://docs.aws.amazon.com/step-functions/latest/dg/input-output-inputpath-params.html#input-output-resultselector) filter. Not populated when QueryLanguage is JSONata.
Type: String
Length Constraints: Maximum length of 262144.
Required: No
**
errorDetails
**
An object containing data about a handled exception in the tested state.
Type: [InspectionErrorDetails](./API_InspectionErrorDetails.html) object
Required: No
**
input
**
The raw state input.
Type: String
Length Constraints: Maximum length of 262144.
Required: No
**
maxConcurrency
**
The max concurrency of the Map state.
Type: Integer
Valid Range: Minimum value of 0.
Required: No
**
request
**
The raw HTTP request that is sent when you test an HTTP Task.
Type: [InspectionDataRequest](./API_InspectionDataRequest.html) object
Required: No
**
response
**
The raw HTTP response that is returned when you test an HTTP Task.
Type: [InspectionDataResponse](./API_InspectionDataResponse.html) object
Required: No
**
result
**
The state's raw result.
Type: String
Length Constraints: Maximum length of 262144.
Required: No
**
toleratedFailureCount
**
The tolerated failure threshold for a Map state as defined in number of Map state iterations.
Type: Integer
Valid Range: Minimum value of 0.
Required: No
**
toleratedFailurePercentage
**
The tolerated failure threshold for a Map state as defined in percentage of Map state iterations.
Type: Float
Valid Range: Minimum value of 0. Maximum value of 100.
Required: No
**
variables
**
JSON string that contains the set of workflow variables after execution of the state. The set will include variables assigned in the state and variables set up as test state input.
Type: String
Length Constraints: Maximum length of 262144.
Required: No