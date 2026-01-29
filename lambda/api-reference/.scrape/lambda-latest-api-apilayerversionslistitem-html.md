---
url: https://docs.aws.amazon.com/lambda/latest/api/API_LayerVersionsListItem.html
title: LayerVersionsListItem
word_count: 319
filtered: true
elements_removed: 0
density_score: 0.83
---

LayerVersionsListItem - AWS Lambda
LayerVersionsListItem - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/api/lambda-api.pdf#API_LayerVersionsListItem)
[Contents](#API_LayerVersionsListItem_Contents)[See Also](#API_LayerVersionsListItem_SeeAlso)
# LayerVersionsListItem
Details about a version of an [AWS Lambda
layer](https://docs.aws.amazon.com/lambda/latest/dg/configuration-layers.html).
## Contents
**
CompatibleArchitectures
**
A list of compatible
[instruction set architectures](https://docs.aws.amazon.com/lambda/latest/dg/foundation-arch.html).
Type: Array of strings
Array Members: Minimum number of 0 items. Maximum number of 2 items.
Valid Values: `x86\_64 | arm64`
Required: No
**
CompatibleRuntimes
**
The layer's compatible runtimes.
The following list includes deprecated runtimes. For more information, see
[Runtime use after deprecation](https://docs.aws.amazon.com/lambda/latest/dg/lambda-runtimes.html#runtime-deprecation-levels).
For a list of all currently supported runtimes, see
[Supported runtimes](https://docs.aws.amazon.com/lambda/latest/dg/lambda-runtimes.html#runtimes-supported).
Type: Array of strings
Array Members: Minimum number of 0 items. Maximum number of 15 items.
Valid Values: `nodejs | nodejs4.3 | nodejs6.10 | nodejs8.9 | nodejs8.10 | nodejs8.x | nodejs10.x | nodejs12.x | nodejs14.x | nodejs16.x | nodejs18.x | nodejs20.x | nodejs22.x | nodejs24.x | java8 | java8.al2 | java11 | java17 | java21 | java25 | python2.7 | python3.4 | python3.6 | python3.7 | python3.8 | python3.9 | python3.10 | python3.11 | python3.12 | python3.13 | python3.14 | dotnetcore1.0 | dotnetcore2.0 | dotnetcore2.1 | dotnetcore3.1 | dotnet6 | dotnet8 | dotnet10 | nodejs4.3-edge | python2.7-greengrass | byol | go1.9 | go1.x | ruby2.5 | ruby2.6 | ruby2.7 | ruby3.2 | ruby3.3 | ruby3.4 | provided | provided.al2 | provided.al2023 | nasa | nodejs26.x | ruby3.5 | python3.15`
Required: No
**
CreatedDate
**
The date that the version was created, in ISO 8601 format. For example, `2018-11-27T15:10:45.123+0000`.
Type: String
Required: No
**
Description
**
The description of the version.
Type: String
Length Constraints: Minimum length of 0. Maximum length of 256.
Required: No
**
LayerVersionArn
**
The ARN of the layer version.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 140.
Pattern: `arn:[a-zA-Z0-9-]+:lambda:[a-zA-Z0-9-]+:\\d{12}:layer:[a-zA-Z0-9-\_]+:[0-9]+`
Required: No
**
LicenseInfo
**
The layer's open-source license.
Type: String
Length Constraints: Minimum length of 0. Maximum length of 512.
Required: No
**
Version
**
The version number.
Type: Long
Required: No