---
url: https://docs.aws.amazon.com/apigateway/latest/api/API_AccessLogSettings.html
title: AccessLogSettings
word_count: 104
filtered: true
elements_removed: 0
density_score: 0.92
---

AccessLogSettings - Amazon API Gateway
AccessLogSettings - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/api/apigw-api.pdf#API_AccessLogSettings)
[Contents](#API_AccessLogSettings_Contents)[See Also](#API_AccessLogSettings_SeeAlso)
# AccessLogSettings
Access log settings, including the access log format and access log destination ARN.
## Contents
**
destinationArn
**
The Amazon Resource Name (ARN) of the CloudWatch Logs log group or Kinesis Data Firehose delivery stream to receive access logs. If you specify a Kinesis Data Firehose delivery stream, the stream name must begin with `amazon-apigateway-`.
Type: String
Required: No
**
format
**
A single line format of the access logs of data, as specified by selected $context variables. The format must include at least `$context.requestId`.
Type: String
Required: No