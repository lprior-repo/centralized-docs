---
url: https://docs.aws.amazon.com/apigateway/latest/api/API_MethodSnapshot.html
title: MethodSnapshot
word_count: 83
filtered: true
elements_removed: 0
density_score: 0.92
---

MethodSnapshot - Amazon API Gateway
MethodSnapshot - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/api/apigw-api.pdf#API_MethodSnapshot)
[Contents](#API_MethodSnapshot_Contents)[See Also](#API_MethodSnapshot_SeeAlso)
# MethodSnapshot
Represents a summary of a Method resource, given a particular date and time.
## Contents
**
apiKeyRequired
**
Specifies whether the method requires a valid ApiKey.
Type: Boolean
Required: No
**
authorizationType
**
The method's authorization type. Valid values are `NONE` for open access, `AWS\_IAM` for using AWS IAM permissions, `CUSTOM` for using a custom authorizer, or `COGNITO\_USER\_POOLS` for using a Cognito user pool.
Type: String
Required: No