---
url: https://docs.aws.amazon.com/lambda/latest/api/API_DocumentDBEventSourceConfig.html
title: DocumentDBEventSourceConfig
word_count: 148
filtered: true
elements_removed: 0
density_score: 0.80
---

DocumentDBEventSourceConfig - AWS Lambda
DocumentDBEventSourceConfig - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/api/lambda-api.pdf#API_DocumentDBEventSourceConfig)
[Contents](#API_DocumentDBEventSourceConfig_Contents)[See Also](#API_DocumentDBEventSourceConfig_SeeAlso)
# DocumentDBEventSourceConfig
Specific configuration settings for a DocumentDB event source.
## Contents
**
CollectionName
**
The name of the collection to consume within the database. If you do not specify a collection, Lambda consumes all collections.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 57.
Pattern: `(^(?!(system\\x2e)))(^[\_a-zA-Z0-9])([^$]\*)`
Required: No
**
DatabaseName
**
The name of the database to consume within the DocumentDB cluster.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 63.
Pattern: `[^ /\\.$\\x22]\*`
Required: No
**
FullDocument
**
Determines what DocumentDB sends to your event stream during document update operations. If set to UpdateLookup, DocumentDB sends a delta describing the changes, along with a copy of the entire document. Otherwise, DocumentDB sends only a partial document that contains the changes.
Type: String
Valid Values: `UpdateLookup | Default`
Required: No