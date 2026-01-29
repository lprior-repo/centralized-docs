---
url: https://docs.aws.amazon.com/lambda/latest/dg/tenant-isolation-troubleshooting.html
title: Troubleshooting tenant isolation for Lambda functions
word_count: 270
filtered: true
elements_removed: 0
density_score: 0.89
---

Troubleshooting tenant isolation for Lambda functions - AWS Lambda
Troubleshooting tenant isolation for Lambda functions - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#tenant-isolation-troubleshooting)
[InvalidParameterValueException](#tenant-isolation-invalidparametervalueexception)[TooManyRequestsException](#tenant-isolation-toomanyrequestsexception)
# Troubleshooting tenant isolation for Lambda functions
This page addresses common issues that occur when using tenant isolation for
AWS Lambda.
## InvalidParameterValueException
**Error :** Tenant ID configuration not specified or
passed to a function when tenant isolation is not enabled.
### Common causes
This error occurs when invoking a tenant-isolated function without a tenant ID, or
invoking a non-tenant-isolated function with a tenant ID.
### Resolution
Add a tenant ID if the function has tenant isolation enabled, or remove the tenant
ID if the function doesn't have tenant isolation enabled.
### Common causes
In addition to rate limiting based on [maximum concurrent executions](./gettingstarted-limits.html#compute-and-storage) and [function scaling rate](./scaling-behavior.html), Lambda limits the maximum number of tenant-aware execution environments (active or idle) that can exist at a time to 2,500 for every 1,000 concurrent executions of your function.
### Resolution
To fix this issue, you can either lower the rate at which invocation requests with unique tenant identifiers are made, [implement retries with backoff and jitter](https://aws.amazon.com/builders-library/timeouts-retries-and-backoff-with-jitter/), or [request a function concurrency limit increase](https://docs.aws.amazon.com/servicequotas/latest/userguide/request-quota-increase.html).
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Monitoring
Integrating other services
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.