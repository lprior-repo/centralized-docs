---
url: https://docs.aws.amazon.com/lambda/latest/api/API_FilterCriteriaError.html
title: FilterCriteriaError
word_count: 81
filtered: true
elements_removed: 0
density_score: 0.93
---

FilterCriteriaError - AWS Lambda
FilterCriteriaError - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/api/lambda-api.pdf#API_FilterCriteriaError)
[Contents](#API_FilterCriteriaError_Contents)[See Also](#API_FilterCriteriaError_SeeAlso)
# FilterCriteriaError
An object that contains details about an error related to filter criteria encryption.
## Contents
**
ErrorCode
**
The AWS KMS exception that resulted from filter criteria encryption or decryption.
Type: String
Length Constraints: Minimum length of 10. Maximum length of 50.
Pattern: `[A-Za-z]+Exception`
Required: No
**
Message
**
The error message.
Type: String
Length Constraints: Minimum length of 10. Maximum length of 2048.
Pattern: `.\*`
Required: No