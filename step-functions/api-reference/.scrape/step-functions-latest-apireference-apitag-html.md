---
url: https://docs.aws.amazon.com/step-functions/latest/apireference/API_Tag.html
title: Tag
word_count: 125
filtered: true
elements_removed: 0
density_score: 0.85
---

Tag - AWS Step Functions
Tag - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/apireference/step-functions-api.pdf#API_Tag)
[Contents](#API_Tag_Contents)[See Also](#API_Tag_SeeAlso)
# Tag
Tags are key-value pairs that can be associated with Step Functions state machines and
activities.
An array of key-value pairs. For more information, see [Using
Cost Allocation Tags](https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/cost-alloc-tags.html) in the *
AWS Billing and Cost Management User
Guide*, and [Controlling Access Using IAM
Tags](https://docs.aws.amazon.com/IAM/latest/UserGuide/access_iam-tags.html).
Tags may only contain Unicode letters, digits, white space, or these symbols: `\_ . : / = + - @`.
## Contents
**
key
**
The key of a tag.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 128.
Required: No
**
value
**
The value of a tag.
Type: String
Length Constraints: Minimum length of 0. Maximum length of 256.
Required: No