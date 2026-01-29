---
url: https://docs.aws.amazon.com/apigateway/latest/api/API_DocumentationPartLocation.html
title: DocumentationPartLocation
word_count: 386
filtered: true
elements_removed: 0
density_score: 0.86
---

DocumentationPartLocation - Amazon API Gateway
DocumentationPartLocation - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/api/apigw-api.pdf#API_DocumentationPartLocation)
[Contents](#API_DocumentationPartLocation_Contents)[See Also](#API_DocumentationPartLocation_SeeAlso)
# DocumentationPartLocation
Specifies the target API entity to which the documentation applies.
## Contents
**
type
**
The type of API entity to which the documentation content applies. Valid values are `API`, `AUTHORIZER`, `MODEL`, `RESOURCE`, `METHOD`, `PATH\_PARAMETER`, `QUERY\_PARAMETER`, `REQUEST\_HEADER`, `REQUEST\_BODY`, `RESPONSE`, `RESPONSE\_HEADER`, and `RESPONSE\_BODY`. Content inheritance does not apply to any entity of the `API`, `AUTHORIZER`, `METHOD`, `MODEL`, `REQUEST\_BODY`, or `RESOURCE` type.
Type: String
Valid Values: `API | AUTHORIZER | MODEL | RESOURCE | METHOD | PATH\_PARAMETER | QUERY\_PARAMETER | REQUEST\_HEADER | REQUEST\_BODY | RESPONSE | RESPONSE\_HEADER | RESPONSE\_BODY`
Required: Yes
**
method
**
The HTTP verb of a method. It is a valid field for the API entity types of `METHOD`, `PATH\_PARAMETER`, `QUERY\_PARAMETER`, `REQUEST\_HEADER`, `REQUEST\_BODY`, `RESPONSE`, `RESPONSE\_HEADER`, and `RESPONSE\_BODY`. The default value is `\*` for any method. When an applicable child entity inherits the content of an entity of the same type with more general specifications of the other `location` attributes, the child entity's `method` attribute must match that of the parent entity exactly.
Type: String
Required: No
**
name
**
The name of the targeted API entity. It is a valid and required field for the API entity types of `AUTHORIZER`, `MODEL`, `PATH\_PARAMETER`, `QUERY\_PARAMETER`, `REQUEST\_HEADER`, `REQUEST\_BODY` and `RESPONSE\_HEADER`. It is an invalid field for any other entity type.
Type: String
Required: No
**
path
**
The URL path of the target. It is a valid field for the API entity types of `RESOURCE`, `METHOD`, `PATH\_PARAMETER`, `QUERY\_PARAMETER`, `REQUEST\_HEADER`, `REQUEST\_BODY`, `RESPONSE`, `RESPONSE\_HEADER`, and `RESPONSE\_BODY`. The default value is `/` for the root resource. When an applicable child entity inherits the content of another entity of the same type with more general specifications of the other `location` attributes, the child entity's `path` attribute must match that of the parent entity as a prefix.
Type: String
Required: No
**
statusCode
**
The HTTP status code of a response. It is a valid field for the API entity types of `RESPONSE`, `RESPONSE\_HEADER`, and `RESPONSE\_BODY`. The default value is `\*` for any status code. When an applicable child entity inherits the content of an entity of the same type with more general specifications of the other `location` attributes, the child entity's `statusCode` attribute must match that of the parent entity exactly.
Type: String
Pattern: `^([1-5]\\d\\d|\\\*|\\s\*)$`
Required: No