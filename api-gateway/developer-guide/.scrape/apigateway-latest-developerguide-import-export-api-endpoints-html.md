---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/import-export-api-endpoints.html
title: Import a Regional API into API Gateway
word_count: 353
filtered: true
elements_removed: 0
density_score: 0.84
---

Import a Regional API into API Gateway - Amazon API Gateway
Import a Regional API into API Gateway - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#import-export-api-endpoints)
[Import a regional API using the API Gateway
console](#import-regional-api-with-console)[Import a regional API using the
AWS CLI](#import-regional-api-with-awscli)
# Import a Regional API into API Gateway
When importing an API, you can choose the regional endpoint configuration for the API. You
can use the API Gateway console, the AWS CLI, or an AWS SDK.
When you export an API, the API endpoint configuration is not included in the exported API
definitions.
For a tutorial on using the Import API feature
from the API Gateway console, see [Tutorial: Create a REST API by importing an
example](./api-gateway-create-api-from-example.html).
###### Topics
* [Import a regional API using the API Gateway
console](#import-regional-api-with-console)
* [Import a regional API using the
AWS CLI](#import-regional-api-with-awscli)
## Import a regional API using the API Gateway
console
To import an API of a regional endpoint using the API Gateway console, do the
following:
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. Choose **Create API**.
3. Under **REST API**, choose **Import**.
4. Copy an API's OpenAPI definition and paste it into the code editor, or choose
**Choose file** to load an OpenAPI file from a local
drive.
5. For **API endpoint type**,
select **Regional**.
6. Choose **Create API** to start importing the OpenAPI
definitions.
## Import a regional API using the
AWS CLI
The following [import-rest-api](https://docs.aws.amazon.com/cli/latest/reference/apigateway/import-rest-api.html) command
imports an OpenAPI definition file and sets the endpoint type to Regional:
```
`aws apigateway import-rest-api \\
--parameters endpointConfigurationTypes=REGIONAL \\
--fail-on-warnings \\
--body 'file://path/to/API\_OpenAPI\_template.json'`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Import an edge-optimized API
Import an OpenAPI file to update an
existing API definition
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.