---
url: https://docs.aws.amazon.com/lambda/latest/api/API_LoggingConfig.html
title: API LoggingConfig.html
word_count: 224
filtered: true
elements_removed: 0
density_score: 0.80
---

LoggingConfig - AWS Lambda
LoggingConfig - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/api/lambda-api.pdf#API_LoggingConfig)
[Contents](#API_LoggingConfig_Contents)[See Also](#API_LoggingConfig_SeeAlso)
## Contents
**
ApplicationLogLevel
**
Set this property to filter the application logs for your function that Lambda sends to CloudWatch. Lambda only sends application logs at the
selected level of detail and lower, where `TRACE` is the highest level and `FATAL` is the lowest.
Type: String
Valid Values: `TRACE | DEBUG | INFO | WARN | ERROR | FATAL`
Required: No
**
LogFormat
**
The format in which Lambda sends your function's application and system logs to CloudWatch. Select between
plain text and structured JSON.
Type: String
Valid Values: `JSON | Text`
Required: No
**
LogGroup
**
The name of the Amazon CloudWatch log group the function sends logs to. By default, Lambda functions send logs to a default
log group named `/aws/lambda/&lt;function name&gt;`. To use a different log group, enter an existing log group or enter a new log group name.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 512.
Pattern: `[\\.\\-\_/#A-Za-z0-9]+`
Required: No
**
SystemLogLevel
**
Set this property to filter the system logs for your function that Lambda sends to CloudWatch. Lambda only sends system logs at the
selected level of detail and lower, where `DEBUG` is the highest level and `WARN` is the lowest.
Type: String
Valid Values: `DEBUG | INFO | WARN`
Required: No