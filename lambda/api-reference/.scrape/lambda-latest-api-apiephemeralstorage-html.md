---
url: https://docs.aws.amazon.com/lambda/latest/api/API_EphemeralStorage.html
title: EphemeralStorage
word_count: 72
filtered: true
elements_removed: 0
density_score: 0.89
---

EphemeralStorage - AWS Lambda
EphemeralStorage - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/api/lambda-api.pdf#API_EphemeralStorage)
[Contents](#API_EphemeralStorage_Contents)[See Also](#API_EphemeralStorage_SeeAlso)
# EphemeralStorage
The size of the function's `/tmp` directory in MB. The default value is 512, but can be any whole
number between 512 and 10,240 MB. For more information, see [Configuring ephemeral storage (console)](https://docs.aws.amazon.com/lambda/latest/dg/configuration-function-common.html#configuration-ephemeral-storage).
## Contents
**
Size
**
The size of the function's `/tmp` directory.
Type: Integer
Valid Range: Minimum value of 512. Maximum value of 10240.
Required: Yes