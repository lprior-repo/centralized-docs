---
url: https://docs.aws.amazon.com/lambda/latest/api/API_TagsError.html
title: TagsError
word_count: 71
filtered: true
elements_removed: 0
density_score: 0.92
---

TagsError - AWS Lambda
TagsError - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/api/lambda-api.pdf#API_TagsError)
[Contents](#API_TagsError_Contents)[See Also](#API_TagsError_SeeAlso)
# TagsError
An object that contains details about an error related to retrieving tags.
## Contents
**
ErrorCode
**
The error code.
Type: String
Length Constraints: Minimum length of 10. Maximum length of 21.
Pattern: `[A-Za-z]+Exception`
Required: Yes
**
Message
**
The error message.
Type: String
Length Constraints: Minimum length of 84. Maximum length of 1000.
Pattern: `.\*`
Required: Yes