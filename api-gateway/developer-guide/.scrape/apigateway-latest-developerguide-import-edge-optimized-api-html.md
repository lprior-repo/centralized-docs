---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/import-edge-optimized-api.html
title: Import an edge-optimized API into API Gateway
word_count: 368
filtered: true
elements_removed: 0
density_score: 0.84
---

Import an edge-optimized API into API Gateway - Amazon API Gateway
Import an edge-optimized API into API Gateway - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#import-edge-optimized-api)
[Import an edge-optimized API
using the API Gateway console](#import-edge-optimized-api-with-console)[Import an edge-optimized API
using the AWS CLI](#import-edge-optimized-api-with-awscli)
# Import an edge-optimized API into API Gateway
You can import an API's OpenAPI definition file to create a new edge-optimized API by
specifying the `EDGE` endpoint type as an additional input, besides the OpenAPI
file, to the import operation. You can do so using the API Gateway console, AWS CLI, or an AWS SDK.
For a tutorial on using the Import API feature
from the API Gateway console, see [Tutorial: Create a REST API by importing an
example](./api-gateway-create-api-from-example.html).
###### Topics
* [Import an edge-optimized API
using the API Gateway console](#import-edge-optimized-api-with-console)
* [Import an edge-optimized API
using the AWS CLI](#import-edge-optimized-api-with-awscli)
## Import an edge-optimized API
using the API Gateway console
To import an edge-optimized API using the API Gateway console, do
the following:
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. Choose **Create API**.
3. Under **REST API**, choose **Import**.
4. Copy an API's OpenAPI definition and paste it into the code editor, or choose
**Choose file** to load an OpenAPI file from a local
drive.
5. For **API endpoint type**,
select **Edge-optimized**.
6. Choose **Create API** to start importing the OpenAPI
definitions.
## Import an edge-optimized API
using the AWS CLI
The following [import-rest-api](https://docs.aws.amazon.com/cli/latest/reference/apigateway/import-rest-api.html) command
imports an API from an OpenAPI definition file to create a new edge-optimized API:
```
`aws apigateway import-rest-api \\
--fail-on-warnings \\
--body 'file://path/to/API\_OpenAPI\_template.json'`
```
or with an explicit specification of the `endpointConfigurationTypes` query
string parameter to `EDGE`:
```
`aws apigateway import-rest-api \\
--parameters endpointConfigurationTypes=EDGE \\
--fail-on-warnings \\
--body 'file://path/to/API\_OpenAPI\_template.json'`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
OpenAPI
Import a Regional API
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.