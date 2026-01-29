---
url: https://docs.aws.amazon.com/apigateway/latest/api/API_ApiKey.html
title: ApiKey
word_count: 223
filtered: true
elements_removed: 0
density_score: 0.79
---

ApiKey - Amazon API Gateway
ApiKey - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/api/apigw-api.pdf#API_ApiKey)
[Contents](#API_ApiKey_Contents)[See Also](#API_ApiKey_SeeAlso)
# ApiKey
A resource that can be distributed to callers for executing Method resources that require an API key. API keys can be mapped to any Stage on any RestApi, which indicates that the callers with the API key can make requests to that stage.
## Contents
**
createdDate
**
The timestamp when the API Key was created.
Type: Timestamp
Required: No
**
customerId
**
An AWS Marketplace customer identifier, when integrating with the AWS SaaS Marketplace.
Type: String
Required: No
**
description
**
The description of the API Key.
Type: String
Required: No
**
enabled
**
Specifies whether the API Key can be used by callers.
Type: Boolean
Required: No
**
id
**
The identifier of the API Key.
Type: String
Required: No
**
lastUpdatedDate
**
The timestamp when the API Key was last updated.
Type: Timestamp
Required: No
**
name
**
The name of the API Key.
Type: String
Required: No
**
stageKeys
**
A list of Stage resources that are associated with the ApiKey resource.
Type: Array of strings
Required: No
**
tags
**
The collection of tags. Each tag element is associated with a given resource.
Type: String to string map
Required: No
**
value
**
The value of the API Key.
Type: String
Required: No