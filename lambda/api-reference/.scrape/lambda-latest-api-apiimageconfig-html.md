---
url: https://docs.aws.amazon.com/lambda/latest/api/API_ImageConfig.html
title: ImageConfig
word_count: 120
filtered: true
elements_removed: 0
density_score: 0.85
---

ImageConfig - AWS Lambda
ImageConfig - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/api/lambda-api.pdf#API_ImageConfig)
[Contents](#API_ImageConfig_Contents)[See Also](#API_ImageConfig_SeeAlso)
# ImageConfig
Configuration values that override the container image Dockerfile settings. For more information, see [Container image
settings](https://docs.aws.amazon.com/lambda/latest/dg/images-create.html#images-parms).
## Contents
**
Command
**
Specifies parameters that you want to pass in with ENTRYPOINT.
Type: Array of strings
Array Members: Minimum number of 0 items. Maximum number of 1500 items.
Required: No
**
EntryPoint
**
Specifies the entry point to their application, which is typically the location of the runtime
executable.
Type: Array of strings
Array Members: Minimum number of 0 items. Maximum number of 1500 items.
Required: No
**
WorkingDirectory
**
Specifies the working directory.
Type: String
Length Constraints: Minimum length of 0. Maximum length of 1000.
Required: No