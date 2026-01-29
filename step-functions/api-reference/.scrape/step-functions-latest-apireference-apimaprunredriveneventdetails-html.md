---
url: https://docs.aws.amazon.com/step-functions/latest/apireference/API_MapRunRedrivenEventDetails.html
title: MapRunRedrivenEventDetails
word_count: 95
filtered: true
elements_removed: 0
density_score: 0.93
---

MapRunRedrivenEventDetails - AWS Step Functions
MapRunRedrivenEventDetails - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/apireference/step-functions-api.pdf#API_MapRunRedrivenEventDetails)
[Contents](#API_MapRunRedrivenEventDetails_Contents)[See Also](#API_MapRunRedrivenEventDetails_SeeAlso)
# MapRunRedrivenEventDetails
Contains details about a Map Run that was redriven.
## Contents
**
mapRunArn
**
The Amazon Resource Name (ARN) of a Map Run that was redriven.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 2000.
Required: No
**
redriveCount
**
The number of times the Map Run has been redriven at this point in the execution's history including this event. The redrive count for a redriven Map Run is always greater than 0.
Type: Integer
Required: No