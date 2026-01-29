---
url: https://docs.aws.amazon.com/lambda/latest/api/API_CodeSigningConfig.html
title: API CodeSigningConfig.html
word_count: 136
filtered: true
elements_removed: 0
density_score: 0.93
---

CodeSigningConfig - AWS Lambda
CodeSigningConfig - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/api/lambda-api.pdf#API_CodeSigningConfig)
[Contents](#API_CodeSigningConfig_Contents)[See Also](#API_CodeSigningConfig_SeeAlso)
## Contents
**
AllowedPublishers
**
List of allowed publishers.
Type: [AllowedPublishers](./API_AllowedPublishers.html) object
Required: Yes
**
CodeSigningConfigArn
**
The Amazon Resource Name (ARN) of the Code signing configuration.
Type: String
Length Constraints: Minimum length of 0. Maximum length of 200.
Pattern: `arn:(aws[a-zA-Z-]\*)?:lambda:[a-z]{2}((-gov)|(-iso(b?)))?-[a-z]+-\\d{1}:\\d{12}:code-signing-config:csc-[a-z0-9]{17}`
Required: Yes
**
CodeSigningConfigId
**
Unique identifer for the Code signing configuration.
Type: String
Pattern: `csc-[a-zA-Z0-9-\_\\.]{17}`
Required: Yes
**
CodeSigningPolicies
**
The code signing policy controls the validation failure action for signature mismatch or expiry.
Type: [CodeSigningPolicies](./API_CodeSigningPolicies.html) object
Required: Yes
**
LastModified
**
The date and time that the Code signing configuration was last modified, in ISO-8601 format (YYYY-MM-DDThh:mm:ss.sTZD).
Type: String
Required: Yes
**
Description
**
Code signing configuration description.
Type: String
Length Constraints: Minimum length of 0. Maximum length of 256.
Required: No