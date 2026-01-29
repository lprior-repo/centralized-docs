---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/set-up-api-with-vpclink-cli.html
title: Set up an API Gateway API with private
word_count: 1046
filtered: true
elements_removed: 0
density_score: 0.82
---

Set up an API Gateway API with private integrations using the AWS CLI (legacy) - Amazon API Gateway
Set up an API Gateway API with private integrations using the AWS CLI (legacy) - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#set-up-api-with-vpclink-cli)
# Set up an API Gateway API with private
integrations using the AWS CLI (legacy)
###### Note
The following implementation of private integrations uses VPC links V1. VPC links V1 are legacy resources. We recommend
that you use [VPC links V2 for REST APIs](./apigateway-vpc-links-v2.html).
The following tutorial shows how to use the AWS CLI to create a VPC link and a private integration. The following prerequisites are required:
* You need an Network Load Balancer created and configured with your VPC source as the target. For more information, see
[Set up a Network Load Balancer
for API Gateway private integrations (legacy)](./set-up-nlb-for-vpclink-using-console.html).
This must be in the same AWS account as your API. You need the Network Load Balancer ARN to create your VPC link.
* To create and manage a `VpcLink`, you need the permissions to create a `VpcLink` in your API. You don't
need the permissions to use the `VpcLink`. For more information, see [Grant permissions for API Gateway to create a VPC
link (legacy)](./grant-permissions-to-create-vpclink.html).
###### To set up an API with the private integration using AWS CLI
1. Use the following [create-vpc-link](https://docs.aws.amazon.com/cli/latest/reference/apigateway/create-vpc-link.html)
command to create a `VpcLink` targeting the specified Network Load Balancer:
```
`aws apigateway create-vpc-link \\
--name my-test-vpc-link \\
--target-arns arn:aws:elasticloadbalancing:us-east-2:`123456789012`:loadbalancer/net/`my-vpclink-test-nlb/1234567890abcdef``
```
The output of this command acknowledges the receipt of the request and shows the
`PENDING` status for the `VpcLink` being created.
```
`{
"status": "PENDING",
"targetArns": [
"arn:aws:elasticloadbalancing:us-east-2:123456789012:loadbalancer/net/my-vpclink-test-nlb/1234567890abcdef"
],
"id": "gim7c3",
"name": "my-test-vpc-link"
}`
```
It takes 2-4 minutes for API Gateway to finish creating the `VpcLink`.
When the operation finishes successfully, the `status` is
`AVAILABLE`. You can verify this by using the following
[get-vpc-link](https://docs.aws.amazon.com/cli/latest/reference/apigateway/get-vpc-link.html)
command:
```
`aws apigateway get-vpc-link --vpc-link-id `gim7c3``
```
If the operation fails, you get a `FAILED` status, with the
`statusMessage` containing the error message. For example, if you
attempt to create a `VpcLink` with a Network Load Balancer that is
already associated with a VPC endpoint, you get the following on the
`statusMessage` property:
```
`"NLB is already associated with another VPC Endpoint Service"`
```
After the `VpcLink` is created successfully, you can create an API and integrate it with the
VPC resource through the `VpcLink`.
Note the `id` value of the newly created `VpcLink`. In this example output, it's
`gim7c3`. You need it to set up the
private integration.
2. Use the following [create-rest-api](https://docs.aws.amazon.com/cli/latest/reference/apigateway/create-rest-api.html)
command to create an API Gateway [`RestApi`](https://docs.aws.amazon.com/apigateway/latest/api/API_RestApi.html)
resource:
```
`aws apigateway create-rest-api --name 'My VPC Link Test'`
```
Note the `RestApi`'s `id` value and the `RestApi`'s
`rootResourceId` value in the returned result. You need this value to perform further operations
on the API.
Next, you create an API with only a `GET`
method on the root resource (`/`) and integrate the method with the
`VpcLink`.
3. Use the following [put-method](https://docs.aws.amazon.com/cli/latest/reference/apigateway/put-method.html) command to create the `GET /`
method:
```
`aws apigateway put-method \\
--rest-api-id `abcdef123` \\
--resource-id `skpp60rab7` \\
--http-method GET \\
--authorization-type "NONE" `
```
If you don't use the proxy integration with the `VpcLink`, you
must also set up at least a method response of the `200` status code.
You use the proxy integration here.
4. After you create the `GET /` method, you set up the integration. For a private integration, you
use the `connection-id` parameter to provide the `VpcLink` ID. You can
use either a stage variable or directly enter the `VpcLink` ID. The
`uri` parameter is not used for routing requests to your endpoint, but is used for setting the
`Host` header and for certificate validation.
Use the VPC link ID
Use the following [put-integration](https://docs.aws.amazon.com/cli/latest/reference/apigateway/put-integration.html)
command to use the `VpcLink` ID directly in the integration:
```
`aws apigateway put-integration \\
--rest-api-id `abcdef123` \\
--resource-id `skpp60rab7` \\
--uri 'http://`my-vpclink-test-nlb-1234567890abcdef`.us-east-2.amazonaws.com' \\
--http-method GET \\
--type HTTP\_PROXY \\
--integration-http-method GET \\
--connection-type VPC\_LINK \\
--connection-id `gim7c3``
```
Use a stage variable
Use the following [put-integration](https://docs.aws.amazon.com/cli/latest/reference/apigateway/put-integration.html)
command to use a stage variable to reference the VPC link ID. When you deploy your API to a stage, you
set the VPC link ID.
```
`aws apigateway put-integration \\
--rest-api-id `abcdef123` \\
--resource-id `skpp60rab7` \\
--uri 'http://`my-vpclink-test-nlb-1234567890abcdef`.us-east-2.amazonaws.com' \\
--http-method GET \\
--type HTTP\_PROXY \\
--integration-http-method GET \\
--connection-type VPC\_LINK \\
--connection-id "\\${stageVariables.vpcLinkId}"`
```
Make sure to double-quote the stage variable expression
(`${stageVariables.vpcLinkId}`) and escape the `$`
character.
At any point, you can also update the integration to change the `connection-id`. Use the following
[update-integration](https://docs.aws.amazon.com/cli/latest/reference/apigateway/update-integration.html) command to
update your integration:
```
` aws apigateway update-integration \\
--rest-api-id `abcdef123` \\
--resource-id `skpp60rab7` \\
--http-method GET \\
--patch-operations '[{"op":"replace","path":"/connectionId","value":"${stageVariables.vpcLinkId}"}]' `
```
Make sure to use a stringified JSON list as the `patch-operations`
parameter value.
Because you used the private proxy integration, your API is now ready for
deployment and for test runs.
5. If you used the stage variable to define your `connection-id`, you need to deploy your API to
test it. Use the following [create-deployment](https://docs.aws.amazon.com/cli/latest/reference/apigateway/create-deployment.html)
command to deploy your API with a stage variable:
```
`aws apigateway create-deployment \\
--rest-api-id `abcdef123` \\
--stage-name test \\
--variables vpcLinkId=`gim7c3``
```
To update the stage variable with a different `VpcLink` ID, such as
``asf9d7``, use the following [update-stage](https://docs.aws.amazon.com/cli/latest/reference/apigateway/update-stage.html) command:
```
`aws apigateway update-stage \\
--rest-api-id `abcdef123` \\
--stage-name test \\
--patch-operations op=replace,path='/variables/vpcLinkId',value='`asf9d7`'`
```
When you hardcode the `connection-id` property with the
`VpcLink` ID literal, you don't need to deploy your API to test it. Use the [test-invoke-method](https://docs.aws.amazon.com/cli/latest/reference/apigateway/test-invoke-method.html) command
to test your API before it is
deployed.
6. Use the following command to invoke your API:
```
`curl -X GET https://`abcdef123`.execute-api.us-east-2.amazonaws.com/test`
```
Alternatively, you can enter your API's invoke-URL in a web browser to view the
result.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Grant permissions for API Gateway to create a VPC
link (legacy)
API Gateway accounts used for private integrations (legacy)
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.