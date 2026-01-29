---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/api-key-file-format.html
title: API Gateway API key file format
word_count: 331
filtered: true
elements_removed: 0
density_score: 0.86
---

API Gateway API key file format - Amazon API Gateway
API Gateway API key file format - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#api-key-file-format)
# API Gateway API key file format
API Gateway can import API keys from external files of a comma-separated value (CSV) format,
and then associate the imported keys with one or more usage plans. The imported file
must contain the `Name` and `Key` columns. The column header names
aren't case sensitive, and columns can be in any order, as shown in the following
example:
```
`Key,name
apikey1234abcdefghij0123456789,MyFirstApiKey`
```
A `Key` value must be between 20 and 128 characters. A `Name` value cannot exceed 1024 characters.
An API key file can also have the `Description`, `Enabled`, or
`UsagePlanIds` column, as shown in the following example:
```
`Name,key,description,Enabled,usageplanIds
MyFirstApiKey,apikey1234abcdefghij0123456789,An imported key,TRUE,c7y23b`
```
When a key is associated with more than one usage plan, the `UsagePlanIds`
value is a comma-separated string of the usage plan IDs, enclosed with a pair of double
or single quotes, as shown in the following example:
```
`Enabled,Name,key,UsageplanIds
true,MyFirstApiKey,apikey1234abcdefghij0123456789,"c7y23b,glvrsr"`
```
Unrecognized columns are permitted, but are ignored. The default value is an empty
string or a `true` Boolean value.
The same API key can be imported multiple times, with the most recent version
overwriting the previous one. Two API keys are identical if they have the same
`key` value.
###### Note
For best practices to consider, see [Best practices for API keys and usage plans](./api-gateway-api-usage-plans.html#apigateway-usage-plans-best-practices).
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Choose an API key source in API Gateway
Set up API keys for REST APIs in API Gateway
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.