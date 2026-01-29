---
url: https://docs.aws.amazon.com/step-functions/latest/apireference/API_DescribeStateMachineAlias.html
title: DescribeStateMachineAlias
word_count: 351
filtered: true
elements_removed: 0
density_score: 0.91
---

DescribeStateMachineAlias - AWS Step Functions
DescribeStateMachineAlias - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/apireference/step-functions-api.pdf#API_DescribeStateMachineAlias)
[Request Syntax](#API_DescribeStateMachineAlias_RequestSyntax)[Request Parameters](#API_DescribeStateMachineAlias_RequestParameters)[Response Syntax](#API_DescribeStateMachineAlias_ResponseSyntax)[Response Elements](#API_DescribeStateMachineAlias_ResponseElements)[Errors](#API_DescribeStateMachineAlias_Errors)[See Also](#API_DescribeStateMachineAlias_SeeAlso)
# DescribeStateMachineAlias
Returns details about a state machine [alias](https://docs.aws.amazon.com/step-functions/latest/dg/concepts-state-machine-alias.html).
**Related operations:**
* [CreateStateMachineAlias](./API_CreateStateMachineAlias.html)
* [ListStateMachineAliases](./API_ListStateMachineAliases.html)
* [UpdateStateMachineAlias](./API_UpdateStateMachineAlias.html)
* [DeleteStateMachineAlias](./API_DeleteStateMachineAlias.html)
## Request Parameters
For information about the parameters that are common to all actions, see [Common Parameters](./CommonParameters.html).
The request accepts the following data in JSON format.
**
[stateMachineAliasArn](#API_DescribeStateMachineAlias_RequestSyntax)
**
The Amazon Resource Name (ARN) of the state machine alias.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 256.
Required: Yes
## Response Syntax
```
`{
"[creationDate](#StepFunctions-DescribeStateMachineAlias-response-creationDate)": ***number***,
"[description](#StepFunctions-DescribeStateMachineAlias-response-description)": "***string***",
"[name](#StepFunctions-DescribeStateMachineAlias-response-name)": "***string***",
"[routingConfiguration](#StepFunctions-DescribeStateMachineAlias-response-routingConfiguration)": [
{
"[stateMachineVersionArn](./API_RoutingConfigurationListItem.html#StepFunctions-Type-RoutingConfigurationListItem-stateMachineVersionArn)": "***string***",
"[weight](./API_RoutingConfigurationListItem.html#StepFunctions-Type-RoutingConfigurationListItem-weight)": ***number***
}
],
"[stateMachineAliasArn](#StepFunctions-DescribeStateMachineAlias-response-stateMachineAliasArn)": "***string***",
"[updateDate](#StepFunctions-DescribeStateMachineAlias-response-updateDate)": ***number***
}`
```
## Response Elements
If the action is successful, the service sends back an HTTP 200 response.
The following data is returned in JSON format by the service.
**
[creationDate](#API_DescribeStateMachineAlias_ResponseSyntax)
**
The date the state machine alias was created.
Type: Timestamp
**
[description](#API_DescribeStateMachineAlias_ResponseSyntax)
**
A description of the alias.
Type: String
Length Constraints: Maximum length of 256.
**
[name](#API_DescribeStateMachineAlias_ResponseSyntax)
**
The name of the state machine alias.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 80.
**
[routingConfiguration](#API_DescribeStateMachineAlias_ResponseSyntax)
**
The routing configuration of the alias.
Type: Array of [RoutingConfigurationListItem](./API_RoutingConfigurationListItem.html) objects
Array Members: Minimum number of 1 item. Maximum number of 2 items.
**
[stateMachineAliasArn](#API_DescribeStateMachineAlias_ResponseSyntax)
**
The Amazon Resource Name (ARN) of the state machine alias.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 256.
**
[updateDate](#API_DescribeStateMachineAlias_ResponseSyntax)
**
The date the state machine alias was last updated.
For a newly created state machine, this is the same as the creation date.
Type: Timestamp
## Errors
For information about the errors that are common to all actions, see [Common Errors](./CommonErrors.html).
**
InvalidArn
**
The provided Amazon Resource Name (ARN) is not valid.
HTTP Status Code: 400
**
ResourceNotFound
**
Could not find the referenced resource.
HTTP Status Code: 400
**
ValidationException
**
The input does not satisfy the constraints specified by an AWS service.
**
reason
**
The input does not satisfy the constraints specified by an AWS service.
HTTP Status Code: 400