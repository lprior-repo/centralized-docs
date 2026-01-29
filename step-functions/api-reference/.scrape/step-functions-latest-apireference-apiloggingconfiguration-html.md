---
url: https://docs.aws.amazon.com/step-functions/latest/apireference/API_LoggingConfiguration.html
title: LoggingConfiguration
word_count: 114
filtered: true
elements_removed: 0
density_score: 0.93
---

LoggingConfiguration - AWS Step Functions
LoggingConfiguration - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/apireference/step-functions-api.pdf#API_LoggingConfiguration)
[Contents](#API_LoggingConfiguration_Contents)[See Also](#API_LoggingConfiguration_SeeAlso)
# LoggingConfiguration
The `LoggingConfiguration` data type is used to set CloudWatch Logs
options.
## Contents
**
destinations
**
An array of objects that describes where your execution history events will be logged.
Limited to size 1. Required, if your log level is not set to `OFF`.
Type: Array of [LogDestination](./API_LogDestination.html) objects
Required: No
**
includeExecutionData
**
Determines whether execution data is included in your log. When set to `false`,
data is excluded.
Type: Boolean
Required: No
**
level
**
Defines which category of execution history events are logged.
Type: String
Valid Values: `ALL | ERROR | FATAL | OFF`
Required: No