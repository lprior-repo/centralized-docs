---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/mcp-server.html
title: Add an API Gateway REST API as a target for Amazon Bedrock AgentCore Gateway
word_count: 753
filtered: true
elements_removed: 0
density_score: 0.85
---

Add an API Gateway REST API as a target for Amazon Bedrock AgentCore Gateway - Amazon API Gateway
Add an API Gateway REST API as a target for Amazon Bedrock AgentCore Gateway - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#mcp-server)
[Considerations](#w2aac15c11c11c34c11b7)[Add a stage of an API as a target for a AgentCore Gateway](#mcp-server-api-gateway)
# Add an API Gateway REST API as a target for Amazon Bedrock AgentCore Gateway
An Amazon Bedrock AgentCore Gateway provides AI agent developers a secure way to expose your API Gateway REST APIs as Model
Context Protocol (MCP)-compatible tools. AgentCore Gateway uses targets to define tools. When you add your stage as
a target, your Gateway becomes a single MCP URL that enables access to the tools for an agent. For more
information, see
[API Gateway
REST API stages as targets](https://docs.aws.amazon.com/bedrock-agentcore/latest/devguide/gateway-target-api-gateway.html) in the *Amazon Bedrock AgentCore
Gateway Developer Guide*.
API Gateway targets connect your AgentCore Gateway to stages of your REST APIs. You can include the entire stage as a
target, or select resources. After you create the API Gateway target, AgentCore Gateway translates incoming MCP
requests into HTTP requests and handles the response formatting. MCP clients can retrieve API documentation using
the `tools/list` method and invoke APIs using the `tools/call` method.
## Considerations
The following considerations might impact your use adding a stage as a target to a AgentCore Gateway:
* You must already have a AgentCore Gateway.
* Only public REST APIs are supported.
* The default endpoint of your API cannot be disabled.
* Every method of your API must either have an [operation name](https://docs.aws.amazon.com/apigateway/latest/api/API_PutMethod.html#apigw-PutMethod-request-operationName)
defined for it, or your need to create a name override when you add your stage as a target. This name is used
as the tool name that agents use to interact with your method.
* You can use `API\_KEY`, `NO\_AUTH`, or `GATEWAY\_IAM\_ROLE` credential
provider types for Outbound Auth to allow your Gateway to access your API. The `API\_KEY` credential
provider is defined by AgentCore Gateway. You can use your existing API Gateway API key. For more information, see
[Setting up
Outbound Auth](https://docs.aws.amazon.com/bedrock-agentcore/latest/devguide/gateway-outbound-auth.html).
* If you use a Amazon Cognito user pool or Lambda authorizer to control access to your
API, MCP clients cannot access it.
* Your API must be in the same account and Region as your AgentCore Gateway.
## Add a stage of an API as a target for a AgentCore Gateway
The following procedure shows how to add a stage of an API as a target for a AgentCore Gateway.
###### To add a stage of an API as a target for a AgentCore Gateway
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. Choose a REST API that's deployed to a stage.
3. In the main navigation pane, choose **Stages**.
4. Choose **Stage actions**, and then choose **Create MCP target**.
5. For **AgentCore Gateway**, select an AgentCore Gateway.
6. For **Target name**, enter a target name.
7. For **Target description**, enter a description.
8. Keep the provided API and stage.
9. For **Select API resources**, select the resources of your API that agents using your
AgentCore Gateway can access.
If you don't select a resource, an agent cannot view the documentation or invoke
the endpoint.
10. The combination of the resource and the method are the operations for the tool. If your operation does not
have a name, create a name override.
You can also define an operation name for a method when you create it.
11. For **Outbound Auth configuration**, choose either **IAM Role **,
**No authorization** or
**API key**.
12. Choose **Create target**.
To view all the AgentCore Gateways that have access to your APIs, choose the **MCP targets**
section in the main navigation pane. In this section, you can create a MCP target for any API in your Region deployed to a stage. Choose
**Create MCP target** and follow the previous steps.
You can also view the available tools for your target and edit your target in the AgentCore Gateway
console. For more information, see
[Add
targets to an existing AgentCore Gateway](https://docs.aws.amazon.com/bedrock-agentcore/latest/devguide/gateway-building-adding-targets.html).
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Set up a stage
Delete a stage
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.