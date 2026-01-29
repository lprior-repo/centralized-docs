---
url: https://docs.aws.amazon.com/lambda/latest/api/API_Cors.html
title: Cors
word_count: 363
filtered: true
elements_removed: 0
density_score: 0.82
---

Cors - AWS Lambda
Cors - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/api/lambda-api.pdf#API_Cors)
[Contents](#API_Cors_Contents)[See Also](#API_Cors_SeeAlso)
# Cors
The [cross-origin resource sharing
(CORS)](https://developer.mozilla.org/en-US/docs/Web/HTTP/CORS) settings for your Lambda function URL. Use CORS to grant access to your function URL
from any origin. You can also use CORS to control access for specific HTTP headers and methods in requests to your
function URL.
## Contents
**
AllowCredentials
**
Whether to allow cookies or other credentials in requests to your function URL. The default is
`false`.
Type: Boolean
Required: No
**
AllowHeaders
**
The HTTP headers that origins can include in requests to your function URL. For example: `Date`, `Keep-Alive`,
`X-Custom-Header`.
Type: Array of strings
Array Members: Minimum number of 0 items. Maximum number of 100 items.
Length Constraints: Minimum length of 0. Maximum length of 1024.
Pattern: `.\*`
Required: No
**
AllowMethods
**
The HTTP methods that are allowed when calling your function URL. For example: `GET`, `POST`, `DELETE`,
or the wildcard character (`\*`).
Type: Array of strings
Array Members: Minimum number of 0 items. Maximum number of 6 items.
Length Constraints: Minimum length of 0. Maximum length of 6.
Pattern: `.\*`
Required: No
**
AllowOrigins
**
The origins that can access your function URL. You can list any number of specific origins, separated by a comma. For example:
`https://www.example.com`, `http://localhost:60905`.
Alternatively, you can grant access to all origins using the wildcard character (`\*`).
Type: Array of strings
Array Members: Minimum number of 0 items. Maximum number of 100 items.
Length Constraints: Minimum length of 1. Maximum length of 253.
Pattern: `.\*`
Required: No
**
ExposeHeaders
**
The HTTP headers in your function response that you want to expose to origins that call your function URL. For example:
`Date`, `Keep-Alive`, `X-Custom-Header`.
Type: Array of strings
Array Members: Minimum number of 0 items. Maximum number of 100 items.
Length Constraints: Minimum length of 0. Maximum length of 1024.
Pattern: `.\*`
Required: No
**
MaxAge
**
The maximum amount of time, in seconds, that web browsers can cache results of a preflight request. By
default, this is set to `0`, which means that the browser doesn't cache results.
Type: Integer
Valid Range: Minimum value of 0. Maximum value of 86400.
Required: No