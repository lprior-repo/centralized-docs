---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-import-api-basePath.html
title: Set the OpenAPI
word_count: 656
filtered: true
elements_removed: 0
density_score: 0.82
---

Set the OpenAPI basePath property - Amazon API Gateway
Set the OpenAPI basePath property - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#api-gateway-import-api-basePath)
[Ignore](#api-gateway-import-api-basePath-ignore)[Prepend](#api-gateway-import-api-basePath-prepend)[Split](#api-gateway-import-api-basePath-split)
# Set the OpenAPI
`basePath` property
In [OpenAPI 2.0](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/2.0.md), you can use the `basePath` property to provide one
or more path parts that precede each path defined in the `paths` property.
Because API Gateway has several ways to express a resource's path, the Import API feature
provides the following options for interpreting the `basePath` property
during import: ignore, prepend, and split.
In [OpenAPI 3.0](https://swagger.io/docs/specification/api-host-and-base-path/), `basePath` is no longer a
top-level property. Instead, API Gateway uses a [server variable](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.0.md#serverVariableObject) as a convention. The Import API
feature provides the same options for interpreting the base path during import. The base
path is identified as follows:
* If the API doesn't contain any `basePath` variables, the Import API
feature checks the `server.url` string to see if it contains a path
beyond `"/"`. If it does, that path is used as the base path.
* If the API contains only one `basePath` variable, the Import API
feature uses it as the base path, even if it's not referenced in the
`server.url`.
* If the API contains multiple `basePath` variables, the Import API
feature uses only the first one as the base path.
## Ignore
If the OpenAPI file has a `basePath` value of `/a/b/c` and
the `paths` property contains `/e` and `/f`, the
following `POST` or `PUT` request:
```
`POST /restapis?mode=import&amp;basepath=ignore`
```
```
`PUT /restapis/`api\_id`?basepath=ignore`
```
results in the following resources in the API:
* `/`
* `/e`
* `/f`
The effect is to treat the `basePath` as if it was not present, and all of the declared API resources are served relative to the
host. This can be used, for example, when you have a custom domain name with an API mapping that doesn't include a *Base
Path* and a *Stage* value that refers to your production stage.
###### Note
API Gateway automatically creates a root resource for you, even if it isn't explicitly declared in your definition file.
When unspecified, `basePath` takes `ignore` by default.
## Prepend
If the OpenAPI file has a `basePath` value of
`/a/b/c` and the `paths` property contains `/e`
and `/f`, the following `POST` or `PUT` request:
```
`POST /restapis?mode=import&amp;basepath=prepend`
```
```
`PUT /restapis/`api\_id`?basepath=prepend`
```
results in the following resources in the API:
* `/`
* `/a`
* `/a/b`
* `/a/b/c`
* `/a/b/c/e`
* `/a/b/c/f`
The effect is to treat the `basePath` as specifying additional
resources (without methods) and to add them to the declared resource set. This can
be used, for example, when different teams are responsible for different parts of an
API and the `basePath` could reference the path location for each team's
API part.
###### Note
API Gateway automatically creates intermediate resources for you, even if they aren't explicitly declared in your definition.
## Split
If the OpenAPI file has a `basePath` value of
`/a/b/c` and the `paths` property contains `/e`
and `/f`, the following `POST` or `PUT` request:
```
`POST /restapis?mode=import&amp;basepath=split`
```
```
`PUT /restapis/`api\_id`?basepath=split`
```
results in the following resources in the API:
* `/`
* `/b`
* `/b/c`
* `/b/c/e`
* `/b/c/f`
The effect is to treat top-most path part, `/a`, as the beginning of
each resource's path, and to create additional (no method) resources within the API
itself. This could, for example, be used when `a` is a stage name that
you want to expose as part of your API.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Import an OpenAPI file to update an
existing API definition
AWS variables
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.