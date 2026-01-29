---
url: https://docs.aws.amazon.com/lambda/latest/api/API_AliasConfiguration.html
title: AliasConfiguration
word_count: 153
filtered: true
elements_removed: 0
density_score: 0.93
---

AliasConfiguration - AWS Lambda
AliasConfiguration - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/api/lambda-api.pdf#API_AliasConfiguration)
[Contents](#API_AliasConfiguration_Contents)[See Also](#API_AliasConfiguration_SeeAlso)
# AliasConfiguration
Provides configuration information about a Lambda function [alias](https://docs.aws.amazon.com/lambda/latest/dg/configuration-aliases.html).
## Contents
**
AliasArn
**
The Amazon Resource Name (ARN) of the alias.
Type: String
Length Constraints: Minimum length of 0. Maximum length of 10000.
Pattern: `arn:(aws[a-zA-Z-]\*)?:lambda:(eusc-)?[a-z]{2}((-gov)|(-iso([a-z]?)))?-[a-z]+-\\d{1}:\\d{12}:function:[a-zA-Z0-9-\_]+(:(\\$LATEST|[a-zA-Z0-9-\_]+))?`
Required: No
**
Description
**
A description of the alias.
Type: String
Length Constraints: Minimum length of 0. Maximum length of 256.
Required: No
**
FunctionVersion
**
The function version that the alias invokes.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 1024.
Pattern: `(\\$LATEST|[0-9]+)`
Required: No
**
Name
**
The name of the alias.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 128.
Pattern: `(?!^[0-9]+$)([a-zA-Z0-9-\_]+)`
Required: No
**
RevisionId
**
A unique identifier that changes when you update the alias.
Type: String
Required: No
**
RoutingConfig
**
The [routing
configuration](https://docs.aws.amazon.com/lambda/latest/dg/lambda-traffic-shifting-using-aliases.html) of the alias.
Type: [AliasRoutingConfiguration](./API_AliasRoutingConfiguration.html) object
Required: No