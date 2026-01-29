---
url: https://docs.aws.amazon.com/step-functions/latest/apireference/API_CreateActivity.html
title: CreateActivity
word_count: 709
filtered: true
elements_removed: 0
density_score: 0.87
---

CreateActivity - AWS Step Functions
CreateActivity - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/apireference/step-functions-api.pdf#API_CreateActivity)
[Request Syntax](#API_CreateActivity_RequestSyntax)[Request Parameters](#API_CreateActivity_RequestParameters)[Response Syntax](#API_CreateActivity_ResponseSyntax)[Response Elements](#API_CreateActivity_ResponseElements)[Errors](#API_CreateActivity_Errors)[See Also](#API_CreateActivity_SeeAlso)
# CreateActivity
Creates an activity. An activity is a task that you write in any programming language and
host on any machine that has access to AWS Step Functions. Activities must poll Step Functions using the
`GetActivityTask` API action and respond using `SendTask\*` API
actions. This function lets Step Functions know the existence of your activity and returns an
identifier for use in a state machine and when polling from the activity.
###### Note
This operation is eventually consistent. The results are best effort and may not reflect very recent updates and changes.
###### Note
`CreateActivity` is an idempotent API. Subsequent requests won’t create a
duplicate resource if it was already created. `CreateActivity`'s idempotency
check is based on the activity `name`. If a following request has different
`tags` values, Step Functions will ignore these differences and treat it as an
idempotent request of the previous. In this case, `tags` will not be updated,
even if they are different.
## Request Syntax
```
`{
"[encryptionConfiguration](#StepFunctions-CreateActivity-request-encryptionConfiguration)": {
"[kmsDataKeyReusePeriodSeconds](./API_EncryptionConfiguration.html#StepFunctions-Type-EncryptionConfiguration-kmsDataKeyReusePeriodSeconds)": `number`,
"[kmsKeyId](./API_EncryptionConfiguration.html#StepFunctions-Type-EncryptionConfiguration-kmsKeyId)": "`string`",
"[type](./API_EncryptionConfiguration.html#StepFunctions-Type-EncryptionConfiguration-type)": "`string`"
},
"[name](#StepFunctions-CreateActivity-request-name)": "`string`",
"[tags](#StepFunctions-CreateActivity-request-tags)": [
{
"[key](./API_Tag.html#StepFunctions-Type-Tag-key)": "`string`",
"[value](./API_Tag.html#StepFunctions-Type-Tag-value)": "`string`"
}
]
}`
```
## Request Parameters
For information about the parameters that are common to all actions, see [Common Parameters](./CommonParameters.html).
The request accepts the following data in JSON format.
**
[encryptionConfiguration](#API_CreateActivity_RequestSyntax)
**
Settings to configure server-side encryption.
Type: [EncryptionConfiguration](./API_EncryptionConfiguration.html) object
Required: No
**
[name](#API_CreateActivity_RequestSyntax)
**
The name of the activity to create. This name must be unique for your AWS account and region for 90 days. For more information,
see [
Limits Related to State Machine Executions](https://docs.aws.amazon.com/step-functions/latest/dg/limits.html#service-limits-state-machine-executions) in the *
AWS Step Functions Developer Guide*.
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
Required: Yes
**
[tags](#API_CreateActivity_RequestSyntax)
**
The list of tags to add to a resource.
An array of key-value pairs. For more information, see [Using
Cost Allocation Tags](https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/cost-alloc-tags.html) in the *
AWS Billing and Cost Management User
Guide*, and [Controlling Access Using IAM
Tags](https://docs.aws.amazon.com/IAM/latest/UserGuide/access_iam-tags.html).
Tags may only contain Unicode letters, digits, white space, or these symbols: `\_ . : / = + - @`.
Type: Array of [Tag](./API_Tag.html) objects
Required: No
## Response Syntax
```
`{
"[activityArn](#StepFunctions-CreateActivity-response-activityArn)": "***string***",
"[creationDate](#StepFunctions-CreateActivity-response-creationDate)": ***number***
}`
```
## Response Elements
If the action is successful, the service sends back an HTTP 200 response.
The following data is returned in JSON format by the service.
**
[activityArn](#API_CreateActivity_ResponseSyntax)
**
The Amazon Resource Name (ARN) that identifies the created activity.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 256.
**
[creationDate](#API_CreateActivity_ResponseSyntax)
**
The date the activity is created.
Type: Timestamp
## Errors
For information about the errors that are common to all actions, see [Common Errors](./CommonErrors.html).
**
ActivityAlreadyExists
**
Activity already exists. `EncryptionConfiguration` may not be updated.
HTTP Status Code: 400
**
ActivityLimitExceeded
**
The maximum number of activities has been reached. Existing activities must be deleted
before a new activity can be created.
HTTP Status Code: 400
**
InvalidEncryptionConfiguration
**
Received when `encryptionConfiguration` is specified but various conditions exist which make the configuration invalid. For example, if `type` is set to `CUSTOMER\_MANAGED\_KMS\_KEY`, but `kmsKeyId` is null, or `kmsDataKeyReusePeriodSeconds` is not between 60 and 900, or the AWS KMS key is not symmetric or inactive.
HTTP Status Code: 400
**
InvalidName
**
The provided name is not valid.
HTTP Status Code: 400
**
KmsAccessDeniedException
**
Either your AWS KMS key policy or API caller does not have the required permissions.
HTTP Status Code: 400
**
KmsThrottlingException
**
Received when AWS KMS returns `ThrottlingException` for a AWS KMS call that Step Functions makes on behalf of the caller.
HTTP Status Code: 400
**
TooManyTags
**
You've exceeded the number of tags allowed for a resource. See the [ Limits Topic](https://docs.aws.amazon.com/step-functions/latest/dg/limits.html) in the
AWS Step Functions Developer Guide.
HTTP Status Code: 400