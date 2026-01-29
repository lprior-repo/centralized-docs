---
url: https://docs.aws.amazon.com/step-functions/latest/apireference/API_InspectionDataResponse.html
title: InspectionDataResponse
word_count: 117
filtered: true
elements_removed: 0
density_score: 0.84
---

InspectionDataResponse - AWS Step Functions
InspectionDataResponse - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/apireference/step-functions-api.pdf#API_InspectionDataResponse)
[Contents](#API_InspectionDataResponse_Contents)[See Also](#API_InspectionDataResponse_SeeAlso)
# InspectionDataResponse
Contains additional details about the state's execution, including its input and output data processing flow, and HTTP response information. The `inspectionLevel` request parameter specifies which details are returned.
## Contents
**
body
**
The HTTP response returned.
Type: String
Required: No
**
headers
**
The response headers associated with the HTTP response.
Type: String
Required: No
**
protocol
**
The protocol used to return the HTTP response.
Type: String
Required: No
**
statusCode
**
The HTTP response status code for the HTTP response.
Type: String
Required: No
**
statusMessage
**
The message associated with the HTTP status code.
Type: String
Required: No