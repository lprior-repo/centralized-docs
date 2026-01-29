---
url: https://docs.aws.amazon.com/step-functions/latest/apireference/API_DescribeActivity.html
title: API DescribeActivity.html
word_count: 320
filtered: true
elements_removed: 0
density_score: 0.92
---

DescribeActivity - AWS Step Functions
DescribeActivity - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/apireference/step-functions-api.pdf#API_DescribeActivity)
[Request Syntax](#API_DescribeActivity_RequestSyntax)[Request Parameters](#API_DescribeActivity_RequestParameters)[Response Syntax](#API_DescribeActivity_ResponseSyntax)[Response Elements](#API_DescribeActivity_ResponseElements)[Errors](#API_DescribeActivity_Errors)[See Also](#API_DescribeActivity_SeeAlso)
###### Note
This operation is eventually consistent. The results are best effort and may not reflect very recent updates and changes.
## Request Parameters
For information about the parameters that are common to all actions, see [Common Parameters](./CommonParameters.html).
The request accepts the following data in JSON format.
**
[activityArn](#API_DescribeActivity_RequestSyntax)
**
The Amazon Resource Name (ARN) of the activity to describe.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 256.
Required: Yes
## Response Syntax
```
`{
"[activityArn](#StepFunctions-DescribeActivity-response-activityArn)": "***string***",
"[creationDate](#StepFunctions-DescribeActivity-response-creationDate)": ***number***,
"[encryptionConfiguration](#StepFunctions-DescribeActivity-response-encryptionConfiguration)": {
"[kmsDataKeyReusePeriodSeconds](./API_EncryptionConfiguration.html#StepFunctions-Type-EncryptionConfiguration-kmsDataKeyReusePeriodSeconds)": ***number***,
"[kmsKeyId](./API_EncryptionConfiguration.html#StepFunctions-Type-EncryptionConfiguration-kmsKeyId)": "***string***",
"[type](./API_EncryptionConfiguration.html#StepFunctions-Type-EncryptionConfiguration-type)": "***string***"
},
"[name](#StepFunctions-DescribeActivity-response-name)": "***string***"
}`
```
## Response Elements
If the action is successful, the service sends back an HTTP 200 response.
The following data is returned in JSON format by the service.
**
[activityArn](#API_DescribeActivity_ResponseSyntax)
**
The Amazon Resource Name (ARN) that identifies the activity.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 256.
**
[creationDate](#API_DescribeActivity_ResponseSyntax)
**
The date the activity is created.
Type: Timestamp
**
[encryptionConfiguration](#API_DescribeActivity_ResponseSyntax)
**
Settings for configured server-side encryption.
Type: [EncryptionConfiguration](./API_EncryptionConfiguration.html) object
**
[name](#API_DescribeActivity_ResponseSyntax)
**
The name of the activity.
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
## Errors
For information about the errors that are common to all actions, see [Common Errors](./CommonErrors.html).
**
ActivityDoesNotExist
**
The specified activity does not exist.
HTTP Status Code: 400
**
InvalidArn
**
The provided Amazon Resource Name (ARN) is not valid.
HTTP Status Code: 400