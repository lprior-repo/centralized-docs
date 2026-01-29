---
url: https://docs.aws.amazon.com/step-functions/latest/apireference/API_EncryptionConfiguration.html
title: EncryptionConfiguration
word_count: 260
filtered: true
elements_removed: 0
density_score: 0.85
---

EncryptionConfiguration - AWS Step Functions
EncryptionConfiguration - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/apireference/step-functions-api.pdf#API_EncryptionConfiguration)
[Contents](#API_EncryptionConfiguration_Contents)[See Also](#API_EncryptionConfiguration_SeeAlso)
# EncryptionConfiguration
Settings to configure server-side encryption.
For additional control over security, you can encrypt your data using a **customer-managed key** for Step Functions state machines and activities. You can configure a symmetric AWS KMS key and data key reuse period when creating or updating a **State Machine**, and when creating an **Activity**. The execution history and state machine definition will be encrypted with the key applied to the State Machine. Activity inputs will be encrypted with the key applied to the Activity.
###### Note
Step Functions automatically enables encryption at rest using AWS owned keys at no charge. However, AWS KMS charges apply when using a customer managed key. For more information about pricing, see [AWS Key Management Service pricing](https://aws.amazon.com/kms/pricing/).
For more information on AWS KMS, see [What is AWS Key Management Service?](https://docs.aws.amazon.com/kms/latest/developerguide/overview.html)
## Contents
**
type
**
Encryption type
Type: String
Valid Values: `AWS\_OWNED\_KEY | CUSTOMER\_MANAGED\_KMS\_KEY`
Required: Yes
**
kmsDataKeyReusePeriodSeconds
**
Maximum duration that Step Functions will reuse data keys. When the period expires, Step Functions will call `GenerateDataKey`. Only applies to customer managed keys.
Type: Integer
Valid Range: Minimum value of 60. Maximum value of 900.
Required: No
**
kmsKeyId
**
An alias, alias ARN, key ID, or key ARN of a symmetric encryption AWS KMS key to encrypt data. To specify a AWS KMS key in a different AWS account, you must use the key ARN or alias ARN.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 2048.
Required: No