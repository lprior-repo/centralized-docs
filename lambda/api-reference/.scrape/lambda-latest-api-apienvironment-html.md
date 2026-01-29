---
url: https://docs.aws.amazon.com/lambda/latest/api/API_Environment.html
title: Environment
word_count: 74
filtered: true
elements_removed: 0
density_score: 0.88
---

Environment - AWS Lambda
Environment - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/api/lambda-api.pdf#API_Environment)
[Contents](#API_Environment_Contents)[See Also](#API_Environment_SeeAlso)
# Environment
A function's environment variable settings. You can use environment variables to adjust your function's
behavior without updating code. An environment variable is a pair of strings that are stored in a function's
version-specific configuration.
## Contents
**
Variables
**
Environment variable key-value pairs. For more information, see [Using Lambda environment variables](https://docs.aws.amazon.com/lambda/latest/dg/configuration-envvars.html).
Type: String to string map
Key Pattern: `[a-zA-Z]([a-zA-Z0-9\_])+`
Required: No