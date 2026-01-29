---
url: https://docs.aws.amazon.com/lambda/latest/api/API_RuntimeVersionConfig.html
title: RuntimeVersionConfig
word_count: 78
filtered: true
elements_removed: 0
density_score: 0.93
---

RuntimeVersionConfig - AWS Lambda
RuntimeVersionConfig - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/api/lambda-api.pdf#API_RuntimeVersionConfig)
[Contents](#API_RuntimeVersionConfig_Contents)[See Also](#API_RuntimeVersionConfig_SeeAlso)
# RuntimeVersionConfig
The ARN of the runtime and any errors that occured.
## Contents
**
Error
**
Error response when Lambda is unable to retrieve the runtime version for a function.
Type: [RuntimeVersionError](./API_RuntimeVersionError.html) object
Required: No
**
RuntimeVersionArn
**
The ARN of the runtime version you want the function to use.
Type: String
Length Constraints: Minimum length of 26. Maximum length of 2048.
Pattern: `arn:(aws[a-zA-Z-]\*):lambda:[a-z]{2}((-gov)|(-iso(b?)))?-[a-z]+-\\d{1}::runtime:.+`
Required: No