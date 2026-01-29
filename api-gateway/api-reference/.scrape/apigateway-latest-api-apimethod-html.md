---
url: https://docs.aws.amazon.com/apigateway/latest/api/API_Method.html
title: Method
word_count: 555
filtered: true
elements_removed: 0
density_score: 0.82
---

Method - Amazon API Gateway
Method - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/api/apigw-api.pdf#API_Method)
[Contents](#API_Method_Contents)[See Also](#API_Method_SeeAlso)
# Method
Represents a client-facing interface by which the client calls the API to access back-end resources. A Method resource is
integrated with an Integration resource. Both consist of a request and one or more responses. The method request takes
the client input that is passed to the back end through the integration request. A method response returns the output from
the back end to the client through an integration response. A method request is embodied in a Method resource, whereas
an integration request is embodied in an Integration resource. On the other hand, a method response is represented
by a MethodResponse resource, whereas an integration response is represented by an IntegrationResponse resource.
## Contents
**
apiKeyRequired
**
A boolean flag specifying whether a valid ApiKey is required to invoke this method.
Type: Boolean
Required: No
**
authorizationScopes
**
A list of authorization scopes configured on the method. The scopes are used with a `COGNITO\_USER\_POOLS` authorizer to authorize the method invocation. The authorization works by matching the method scopes against the scopes parsed from the access token in the incoming request. The method invocation is authorized if any method scopes matches a claimed scope in the access token. Otherwise, the invocation is not authorized. When the method scope is configured, the client must provide an access token instead of an identity token for authorization purposes.
Type: Array of strings
Required: No
**
authorizationType
**
The method's authorization type. Valid values are `NONE` for open access, `AWS\_IAM` for using AWS IAM permissions, `CUSTOM` for using a custom authorizer, or `COGNITO\_USER\_POOLS` for using a Cognito user pool.
Type: String
Required: No
**
authorizerId
**
The identifier of an authorizer to use on this method. The method's authorization type must be `CUSTOM` or `COGNITO\_USER\_POOLS`.
Type: String
Required: No
**
httpMethod
**
The method's HTTP verb.
Type: String
Required: No
**
methodIntegration
**
Gets the method's integration responsible for passing the client-submitted request to the back end and performing necessary transformations to make the request compliant with the back end.
Type: [Integration](./API_Integration.html) object
Required: No
**
methodResponses
**
Gets a method response associated with a given HTTP status code.
Type: String to [MethodResponse](./API_MethodResponse.html) object map
Required: No
**
operationName
**
A human-friendly operation identifier for the method. For example, you can assign the `operationName` of `ListPets` for the `GET /pets` method in the `PetStore` example.
Type: String
Required: No
**
requestModels
**
A key-value map specifying data schemas, represented by Model resources, (as the mapped value) of the request payloads of given content types (as the mapping key).
Type: String to string map
Required: No
**
requestParameters
**
A key-value map defining required or optional method request parameters that can be accepted by API Gateway. A key is a method request parameter name matching the pattern of `method.request.{location}.{name}`, where `location` is `querystring`, `path`, or `header` and `name` is a valid and unique parameter name. The value associated with the key is a Boolean flag indicating whether the parameter is required (`true`) or optional (`false`). The method request parameter names defined here are available in Integration to be mapped to integration request parameters or templates.
Type: String to boolean map
Required: No
**
requestValidatorId
**
The identifier of a RequestValidator for request validation.
Type: String
Required: No