---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-documenting-api-quick-start-with-console.html
title: Document an API
word_count: 2018
filtered: true
elements_removed: 0
density_score: 0.85
---

Document an API using the API Gateway console - Amazon API Gateway
Document an API using the API Gateway console - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#api-gateway-documenting-api-quick-start-with-console)
[Document the API entity](#api-gateway-document-api-add-document-part-for-api-entity-with-console)[Document a RESOURCE entity](#api-gateway-document-api-add-document-part-for-resource-entity-with-console)[Document a METHOD entity](#api-gateway-document-api-add-document-part-for-method-entity-with-console)[Document a QUERY\_PARAMETER entity](#api-gateway-document-api-add-document-part-for-request-query-entity-with-console)[Document a PATH\_PARAMETER entity](#api-gateway-document-api-add-document-part-for-path-parameter-entity-with-console)[ Document a REQUEST\_HEADER entity ](#api-gateway-document-api-add-document-part-for-request-header-entity-with-console)[Document a REQUEST\_BODY entity](#api-gateway-document-api-add-document-part-for-request-body-entity-with-console)[Document a RESPONSE entity](#api-gateway-document-api-add-document-part-for-response-with-console)[Document a RESPONSE\_HEADER entity](#api-gateway-document-api-add-document-part-for-response-header-entity-with-console)[Document a RESPONSE\_BODY entity](#api-gateway-document-api-add-document-part-for-response-body-entity-with-console)[Document a MODEL entity](#api-gateway-document-api-add-document-part-for-model-entity-with-console)[Document an AUTHORIZER entity](#api-gateway-document-api-add-document-part-for-authorizer-entity-with-console)
# Document an API
using the API Gateway console
In this section, we describe how to create and maintain documentation parts of an API
using the API Gateway console.
A prerequisite for creating and editing the documentation of an API is that you must
have already created the API. In this section, we use the [PetStore](http://petstore-demo-endpoint.execute-api.com/petstore/pets)
API as an example. To create an API using the API Gateway console, follow the instructions in
[Tutorial: Create a REST API by importing an
example](./api-gateway-create-api-from-example.html).
###### Topics
* [Document the API entity](#api-gateway-document-api-add-document-part-for-api-entity-with-console)
* [Document a RESOURCE entity](#api-gateway-document-api-add-document-part-for-resource-entity-with-console)
* [Document a METHOD entity](#api-gateway-document-api-add-document-part-for-method-entity-with-console)
* [Document a QUERY\_PARAMETER entity](#api-gateway-document-api-add-document-part-for-request-query-entity-with-console)
* [Document a PATH\_PARAMETER entity](#api-gateway-document-api-add-document-part-for-path-parameter-entity-with-console)
* [ Document a REQUEST\_HEADER entity ](#api-gateway-document-api-add-document-part-for-request-header-entity-with-console)
* [Document a REQUEST\_BODY entity](#api-gateway-document-api-add-document-part-for-request-body-entity-with-console)
* [Document a RESPONSE entity](#api-gateway-document-api-add-document-part-for-response-with-console)
* [Document a RESPONSE\_HEADER entity](#api-gateway-document-api-add-document-part-for-response-header-entity-with-console)
* [Document a RESPONSE\_BODY entity](#api-gateway-document-api-add-document-part-for-response-body-entity-with-console)
* [Document a MODEL entity](#api-gateway-document-api-add-document-part-for-model-entity-with-console)
* [Document an AUTHORIZER entity](#api-gateway-document-api-add-document-part-for-authorizer-entity-with-console)
## Document the `API` entity
To add a new documentation part for the `API` entity, do the following:
1. In the main navigation pane, choose **Documentation**, and then choose **Create documentation part**.
2. For **Documentation type**, select
**API**.
If a documentation part was not created for the `API`, you get the documentation part's `properties` map editor. Enter the following `properties` map in the text editor.
```
`{
"info": {
"description": "Your first API Gateway API.",
"contact": {
"name": "John Doe",
"email": "john.doe@api.com"
}
}
}`
```
###### Note
You do not need to encode the `properties` map into a JSON string.
The API Gateway console stringifies the JSON object for you.
3. Choose **Create documentation part**.
To add a new documentation part for the `API` entity in the **Resources** pane, do the following:
1. In the main navigation pane, choose **Resources**.
2. Choose the **API actions** menu, and then choose **Update API documentation**.
![Edit documentation for the API entity in the API Gateway console](https://docs.aws.amazon.com/images/apigateway/latest/developerguide/images/document-api-entity-using-new-console.png)
To edit an existing documentation part, do the following:
1. In the **Documentation** pane, choose the **Resources and methods** tab.
2. Select the name of your API, and then on the API card, choose
**Edit**.
## Document a `RESOURCE` entity
To add a new documentation part for a `RESOURCE` entity, do the following:
1. In the main navigation pane, choose **Documentation**, and then choose **Create documentation part**.
2. For **Documentation type**, select
**Resource**.
3. For **Path**, enter a path.
4. Enter a description in the text
editor, for example:
```
`{
"description": "The PetStore's root resource."
}`
```
5. Choose **Create documentation part**. You can create documentation for an unlisted resource.
6. If required, repeat these steps to add or edit another documentation part.
To add a new documentation part for a `RESOURCE` entity in the **Resources** pane, do the following:
1. In the main navigation pane, choose **Resources**.
2. Choose the resource, and then choose **Update documentation**.
![Edit documentation for the resource entity in the API Gateway console](https://docs.aws.amazon.com/images/apigateway/latest/developerguide/images/document-resource-entity-using-new-console.png)
To edit an existing documentation part, do the following:
1. In the **Documentation** pane, choose the **Resources and methods** tab.
2. Select the resource containing your documentation part, and then choose
**Edit**.
## Document a `METHOD` entity
To add a new documentation part for a `METHOD` entity, do the following:
1. In the main navigation pane, choose **Documentation**, and then choose **Create documentation part**.
2. For **Documentation type**, select
**Method**.
3. For **Path**, enter a path.
4. For **Method**, select an HTTP verb.
5. Enter a description in the text
editor, for example:
```
`{
"tags" : [ "pets" ],
"summary" : "List all pets"
}`
```
6. Choose **Create documentation part**. You can create documentation for an unlisted method.
7. If required, repeat these steps to add or edit another documentation part.
To add a new documentation part for a `METHOD` entity in the **Resources** pane, do the following:
1. In the main navigation pane, choose **Resources**.
2. Choose the method, and then choose **Update documentation**.
![Edit documentation for the method entity in the API Gateway console](https://docs.aws.amazon.com/images/apigateway/latest/developerguide/images/document-method-entity-using-new-console.png)
To edit an existing documentation part, do the following:
1. In the **Documentation** pane, choose the **Resources and methods** tab.
2. You can select the method or select the resource containing the method, and then use the search bar to find and select your documentation part.
3. Choose
**Edit**.
## Document a `QUERY\_PARAMETER` entity
To add a new documentation part for a `QUERY\_PARAMETER` entity, do the following:
1. In the main navigation pane, choose **Documentation**, and then choose **Create documentation part**.
2. For **Documentation type**, select
**Query parameter**.
3. For **Path**, enter a path.
4. For **Method**, select an HTTP verb.
5. For **Name**, enter a name.
6. Enter a description in the text
editor.
7. Choose **Create documentation part**. You can create documentation for an unlisted query parameter.
8. If required, repeat these steps to add or edit another documentation part.
To edit an existing documentation part, do the following:
1. In the **Documentation** pane, choose the **Resources and methods** tab.
2. You can select the query parameter or select the resource containing the query parameter, and then use the search bar to find and select your documentation part.
3. Choose
**Edit**.
## Document a `PATH\_PARAMETER` entity
To add a new documentation part for a `PATH\_PARAMETER` entity, do the following:
1. In the main navigation pane, choose **Documentation**, and then choose **Create documentation part**.
2. For **Documentation type**, select
**Path parameter**.
3. For **Path**, enter a path.
4. For **Method**, select an HTTP verb.
5. For **Name**, enter a name.
6. Enter a description in the text
editor.
7. Choose **Create documentation part**. You can create documentation for an unlisted path parameter.
8. If required, repeat these steps to add or edit another documentation part.
To edit an existing documentation part, do the following:
1. In the **Documentation** pane, choose the **Resources and methods** tab.
2. You can select the path parameter or select the resource containing the path parameter, and then use the search bar to find and select your documentation part.
3. Choose
**Edit**.
## Document a `REQUEST\_BODY` entity
To add a new documentation part for a `REQUEST\_BODY` entity, do the following:
1. In the main navigation pane, choose **Documentation**, and then choose **Create documentation part**.
2. For **Documentation type**, select
**Request body**.
3. For **Path**, enter a path for the request body.
4. For **Method**, select an HTTP verb.
5. Enter a description in the text
editor.
6. Choose **Create documentation part**. You can create documentation for an unlisted request body.
7. If required, repeat these steps to add or edit another documentation part.
To edit an existing documentation part, do the following:
1. In the **Documentation** pane, choose the **Resources and methods** tab.
2. You can select the request body or select the resource containing the request body, and then use the search bar to find and select your documentation part.
3. Choose
**Edit**.
## Document a `RESPONSE` entity
To add a new documentation part for a `RESPONSE` entity, do the following:
1. In the main navigation pane, choose **Documentation**, and then choose **Create documentation part**.
2. For **Documentation type**, select
**Response (status code)**.
3. For **Path**, enter a path for the response.
4. For **Method**, select an HTTP verb.
5. For **Status code**, enter an HTTP status code.
6. Enter a description in the text
editor.
7. Choose **Create documentation part**. You can create documentation for an unlisted response status code.
8. If required, repeat these steps to add or edit another documentation part.
To edit an existing documentation part, do the following:
1. In the **Documentation** pane, choose the **Resources and methods** tab.
2. You can select the response status code or select the resource containing the response status code, and then use the search bar to find and select your documentation part.
3. Choose
**Edit**.
## Document a `RESPONSE\_BODY` entity
To add a new documentation part for a `RESPONSE\_BODY` entity, do the following:
1. In the main navigation pane, choose **Documentation**, and then choose **Create documentation part**.
2. For **Documentation type**, select
**Response body**.
3. For **Path**, enter a path for the response body.
4. For **Method**, select an HTTP verb.
5. For **Status code**, enter an HTTP status code.
6. Enter a description in the text
editor.
7. Choose **Create documentation part**. You can create documentation for an unlisted response body.
8. If required, repeat these steps to add or edit another documentation part.
To edit an existing documentation part, do the following:
1. In the **Documentation** pane, choose the **Resources and methods** tab.
2. You can select the response body or select the resource containing the response body, and then use the search bar to find and select your documentation part.
3. Choose
**Edit**.
## Document a `MODEL` entity
Documenting a `MODEL` entity involves creating and
managing `DocumentPart` instances for the model and
each of the model's `properties`'. For example, for
the `Error` model that comes with every API by default
has the following schema definition,
```
`{
"$schema" : "http://json-schema.org/draft-04/schema#",
"title" : "Error Schema",
"type" : "object",
"properties" : {
"message" : { "type" : "string" }
}
}`
```
and requires two `DocumentationPart` instances, one
for the `Model` and the other for its `message` property:
```
`{
"location": {
"type": "MODEL",
"name": "Error"
},
"properties": {
"title": "Error Schema",
"description": "A description of the Error model"
}
}
`
```
and
```
`{
"location": {
"type": "MODEL",
"name": "Error.message"
},
"properties": {
"description": "An error message."
}
}
`
```
When the API is exported, the `DocumentationPart`'s
properties will override the values in the original schema.
To add a new documentation part for a `MODEL` entity, do the following:
1. In the main navigation pane, choose **Documentation**, and then choose **Create documentation part**.
2. For **Documentation type**, select
**Model**.
3. For **Name**, enter a name for the model.
4. Enter a description in the text
editor.
5. Choose **Create documentation part**. You can create documentation for unlisted models.
6. If required, repeat these steps to add or edit a documentation part to other models.
To add a new documentation part for a `MODEL` entity in the **Models** pane, do the following:
1. In the main navigation pane, choose **Models**.
2. Choose the model, and then choose **Update documentation**.
![Edit documentation for the model entity in the API Gateway console](https://docs.aws.amazon.com/images/apigateway/latest/developerguide/images/document-model-entity-using-new-console.png)
To edit an existing documentation part, do the following:
1. In the **Documentation** pane, choose the **Models** tab.
2. Use the search bar or select the model, and then choose
**Edit**.
## Document an `AUTHORIZER` entity
To add a new documentation part for an `AUTHORIZER` entity, do the following:
1. In the main navigation pane, choose **Documentation**, and then choose **Create documentation part**.
2. For **Documentation type**, select
**Authorizer**.
3. For **Name**, enter the name of your authorizer.
4. Enter a description in the text
editor. Specify a value for the valid `location`
field for the authorizer.
5. Choose **Create documentation part**. You can create documentation for unlisted authorizers.
6. If required, repeat these steps to add or edit a documentation part to other authorizers.
To edit an existing documentation part, do the following:
1. In the **Documentation** pane, choose the **Authorizers** tab.
2. Use the search bar or select the authorizer, and then choose
**Edit**.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Representation of
API documentation in API Gateway
Publish API
documentation using the API Gateway console
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.