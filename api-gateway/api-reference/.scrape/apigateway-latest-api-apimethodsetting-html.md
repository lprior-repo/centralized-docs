---
url: https://docs.aws.amazon.com/apigateway/latest/api/API_MethodSetting.html
title: API MethodSetting.html
word_count: 277
filtered: true
elements_removed: 0
density_score: 0.80
---

MethodSetting - Amazon API Gateway
MethodSetting - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/api/apigw-api.pdf#API_MethodSetting)
[Contents](#API_MethodSetting_Contents)[See Also](#API_MethodSetting_SeeAlso)
## Contents
**
cacheDataEncrypted
**
Specifies whether the cached responses are encrypted.
Type: Boolean
Required: No
**
cacheTtlInSeconds
**
Specifies the time to live (TTL), in seconds, for cached responses. The higher the TTL, the longer the response will be cached.
Type: Integer
Required: No
**
cachingEnabled
**
Specifies whether responses should be cached and returned for requests. A cache cluster must be enabled on the stage for responses to be cached.
Type: Boolean
Required: No
**
dataTraceEnabled
**
Specifies whether data trace logging is enabled for this method, which affects the log entries pushed to Amazon CloudWatch Logs. This can be useful to troubleshoot APIs, but can result in logging sensitive data. We recommend that you don't enable this option for production APIs.
Type: Boolean
Required: No
**
loggingLevel
**
Specifies the logging level for this method, which affects the log entries pushed to Amazon CloudWatch Logs. Valid values are `OFF`, `ERROR`, and `INFO`. Choose `ERROR` to write only error-level entries to CloudWatch Logs, or choose `INFO` to include all `ERROR` events as well as extra informational events.
Type: String
Required: No
**
metricsEnabled
**
Specifies whether Amazon CloudWatch metrics are enabled for this method.
Type: Boolean
Required: No
**
requireAuthorizationForCacheControl
**
Specifies whether authorization is required for a cache invalidation request.
Type: Boolean
Required: No
**
throttlingBurstLimit
**
Specifies the throttling burst limit.
Type: Integer
Required: No
**
throttlingRateLimit
**
Specifies the throttling rate limit.
Type: Double
Required: No
**
unauthorizedCacheControlHeaderStrategy
**
Specifies how to handle unauthorized requests for cache invalidation.
Type: String
Valid Values: `FAIL\_WITH\_403 | SUCCEED\_WITH\_RESPONSE\_HEADER | SUCCEED\_WITHOUT\_RESPONSE\_HEADER`
Required: No