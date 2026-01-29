---
url: https://docs.aws.amazon.com/lambda/latest/api/API_SelfManagedEventSource.html
title: SelfManagedEventSource
word_count: 85
filtered: true
elements_removed: 0
density_score: 0.93
---

SelfManagedEventSource - AWS Lambda
SelfManagedEventSource - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/api/lambda-api.pdf#API_SelfManagedEventSource)
[Contents](#API_SelfManagedEventSource_Contents)[See Also](#API_SelfManagedEventSource_SeeAlso)
# SelfManagedEventSource
The self-managed Apache Kafka cluster for your event source.
## Contents
**
Endpoints
**
The list of bootstrap servers for your Kafka brokers in the following format: `"KAFKA\_BOOTSTRAP\_SERVERS":
["abc.xyz.com:xxxx","abc2.xyz.com:xxxx"]`.
Type: String to array of strings map
Map Entries: Maximum number of 2 items.
Valid Keys: `KAFKA\_BOOTSTRAP\_SERVERS`
Array Members: Minimum number of 1 item. Maximum number of 10 items.
Length Constraints: Minimum length of 1. Maximum length of 300.
Pattern: `(([a-zA-Z0-9]|[a-zA-Z0-9][a-zA-Z0-9\\-]\*[a-zA-Z0-9])\\.)\*([A-Za-z0-9]|[A-Za-z0-9][A-Za-z0-9\\-]\*[A-Za-z0-9]):[0-9]{1,5}`
Required: No