---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-private-api-create.html
title: Create a private API
word_count: 2710
filtered: true
elements_removed: 0
density_score: 0.84
---

Create a private API - Amazon API Gateway
Create a private API - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#apigateway-private-api-create)
[Prerequisites](#apigateway-private-api-create-interface-vpc-prerequisites)[Step 1: Create a VPC endpoint for API Gateway in your VPC](#apigateway-private-api-create-interface-vpc-endpoint)[Step 2: Create a private API](#apigateway-private-api-create-using-console)[Step 3: Set up a resource policy
for a private API](#apigateway-private-api-set-up-resource-policy)[(Optional) Associate or disassociate a VPC endpoint
with a private API](#associate-private-api-with-vpc-endpoint)[Step 4: Deploy a private API](#apigateway-private-api-deploy-using-console)[Troubleshoot your private API](#apigateway-private-api-troubleshooting)
# Create a private API
Before you create a private API, you first create a VPC endpoint for API Gateway. Next you create your private API and attach
a resource policy to it. Optionally, you can associate your VPC endpoint with your private API to simplify how you
invoke your API. Finally, you deploy your API.
The following procedures describe how to accomplish this. You can
create a private REST API using the AWS Management Console, AWS CLI or an AWS SDK.
## Prerequisites
To follow these steps, you must have a fully configured VPC. To learn how to create a VPC, see [Create a VPC only](https://docs.aws.amazon.com/vpc/latest/userguide/create-vpc.html#create-vpc-only) in
the *Amazon VPC User Guide*. To follow all recommend steps when you create your VPC, enable
private DNS. This way you can invoke your API within a VPC without having to pass the Host or `x-apigw-api-id`
header.
To enable private DNS, the `enableDnsSupport` and
`enableDnsHostnames` attributes of your VPC must be set
to `true`. For more information, see [DNS Support in Your
VPC](https://docs.aws.amazon.com/vpc/latest/userguide/vpc-dns.html#vpc-dns-support) and [Updating DNS Support for Your VPC](https://docs.aws.amazon.com/vpc/latest/userguide/vpc-dns.html#vpc-dns-updating).
## Step 1: Create a VPC endpoint for API Gateway in your VPC
The following procedure shows how to create a VPC endpoint for API Gateway. To create a VPC endpoint for API Gateway,
you specify the `execute-api` domain for the AWS Region where you create your private API. The
`execute-api` domain is the API Gateway component service for API execution.
When you create your VPC endpoint for API Gateway, you specify the DNS settings. If you turn off private DNS, you
can only access your API using public DNS. For more information, see [Issue: I can't
connect to my public API from an API Gateway VPC endpoint](#apigateway-private-api-troubleshooting-public-access).
AWS Management Console
###### To create an interface VPC endpoint for API Gateway
1. Sign in to the AWS Management Console and open the Amazon VPC console at
[https://console.aws.amazon.com/vpc/](https://console.aws.amazon.com/vpc/).
2. In the navigation pane, under **Virtual private cloud**, choose
**Endpoints**.
3. Choose **Create endpoint**.
4. (Optional) For **Name tag**, enter a name to help identify your VPC endpoint.
5. For **Service category**, choose **AWS
services**.
6. Under **Services**, in the search bar, enter `execute-api`.
Then, choose the API Gateway service endpoint in the AWS Region where you will create your API. The service name
should look like `com.amazonaws.us-east-1.execute-api` and the **Type**
should be **Interface**.
7. For **VPC**, choose the VPC that you want to create
the endpoint in.
8. (Optional) To turn off **Enable Private DNS
Name**, choose **Additional settings** and then clear **Enable Private DNS
Name**.
9. For **Subnets**, choose the Availability Zones where you created the
endpoint network interfaces. To improve the availability of your API, choose multiple subnets.
10. For **Security group**, select the security group to
associate with the VPC endpoint network interfaces.
The security group you choose must be set to allow TCP Port 443
inbound HTTPS traffic from either an IP range in your VPC or another
security group in your VPC.
11. For **Policy**, do one of the following:
* If you have not created your private API or you don't want to configure a custom VPC endpoint policy, choose **Full access**.
* If you have already created a private API and want to configure a custom VPC endpoint policy,
you can enter a custom VPC endpoint policy. For more information, see [Use VPC endpoint policies for private
APIs in API Gateway](./apigateway-vpc-endpoint-policies.html).
You can update the VPC endpoint policy after you create your VPC endpoint. For more information,
see [Update a VPC
endpoint policy](https://docs.aws.amazon.com/vpc/latest/privatelink/vpc-endpoints-access.html#update-vpc-endpoint-policy).
* Choose **Create endpoint**.
* Copy the resulting VPC endpoint ID, as you might use it in future steps.
AWS CLI
The following [create-vpc-endpoint](https://docs.aws.amazon.com/cli/latest/reference/ec2/create-vpc-endpoint.html) command creates a VPC endpoint:
```
`aws ec2 create-vpc-endpoint \\
--vpc-id vpc-1a2b3c4d \\
--vpc-endpoint-type Interface \\
--service-name com.amazonaws.us-east-1.execute-api \\
--subnet-ids subnet-7b16de0c \\
--security-group-id sg-1a2b3c4d`
```
Copy the resulting VPC endpoint ID, as you might use it in future steps.
## Step 2: Create a private API
After you create your VPC endpoint, you create a private REST API. The following procedure shows how to
create a private API.
AWS Management Console
###### To create a private API
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. Choose **Create API**.
3. Under **REST API**, choose **Build**.
4. For **Name**, enter a name.
5. (Optional) For **Description**, enter a description.
6. For **API endpoint type**, select **Private**.
7. (Optional) For **VPC endpoint IDs**, enter a VPC endpoint ID.
If you associate a VPC endpoint ID with your private API, you can invoke your API from within your VPC without overriding a `Host` header or passing an
`x-apigw-api-id header` For more information, see [(Optional) Associate or disassociate a VPC endpoint
with a private API](#associate-private-api-with-vpc-endpoint).
8. For **IP address type**, choose **Dualstack**.
9. Choose **Create API**.
After completing the preceding steps, you can follow the instructions in [Get started with the REST API console](./getting-started-rest-new-console.html) to set up methods and integrations for this API, but you
can't deploy your API. To deploy your API, follow step 3 and attach a resource policy to your API.
AWS CLI
The following [create-rest-api](https://docs.aws.amazon.com/cli/latest/reference/apigateway/create-rest-api.html) command creates a private API:
```
`aws apigateway create-rest-api \\
--name 'Simple PetStore (AWS CLI, Private)' \\
--description 'Simple private PetStore API' \\
--region us-west-2 \\
--endpoint-configuration '{ "types": ["PRIVATE"], "ipAddressType": "dualstack" }'`
```
A successful call returns output similar to the following:
```
`{
"createdDate": "2017-10-13T18:41:39Z",
"description": "Simple private PetStore API",
"endpointConfiguration": {
"types": [
"PRIVATE"
],
"ipAddressType": "dualstack"
},
"id": "0qzs2sy7bh",
"name": "Simple PetStore (AWS CLI, Private)"
}`
```
After completing the preceding steps, you can follow the instructions in [Tutorial: Create a REST API using
AWS SDKs or the AWS CLI](./api-gateway-create-api-cli-sdk.html) to set up methods and integrations for this API, but you
can't deploy your API. To deploy your API, follow step 3 and attach a resource policy to your API.
SDK JavaScript v3
The following example shows how to create a private API by using the AWS SDK for JavaScript v3:
```
`import {APIGatewayClient, CreateRestApiCommand} from "@aws-sdk/client-api-gateway";
const apig = new APIGatewayClient({region:"us-east-1"});
const input = { // CreateRestApiRequest
name: "Simple PetStore (JavaScript v3 SDK, private)", // required
description: "Demo private API created using the AWS SDK for JavaScript v3",
version: "0.00.001",
endpointConfiguration: { // EndpointConfiguration
types: [ "PRIVATE"],
},
};
export const handler = async (event) =&gt; {
const command = new CreateRestApiCommand(input);
try {
const result = await apig.send(command);
console.log(result);
} catch (err){
console.error(err)
}
};`
```
A successful call returns output similar to the following:
```
`{
apiKeySource: 'HEADER',
createdDate: 2024-04-03T17:56:36.000Z,
description: 'Demo private API created using the AWS SDK for JavaScript v3',
disableExecuteApiEndpoint: false,
endpointConfiguration: { types: [ 'PRIVATE' ] },
id: 'abcd1234',
name: 'Simple PetStore (JavaScript v3 SDK, private)',
rootResourceId: 'efg567',
version: '0.00.001'
}`
```
After completing the preceding steps, you can follow the instructions in [Tutorial: Create a REST API using
AWS SDKs or the AWS CLI](./api-gateway-create-api-cli-sdk.html) to set up methods and integrations for this API, but you
can't deploy your API. To deploy your API, follow step 3 and attach a resource policy to your API.
Python SDK
The following example shows how to create a private API by using the AWS SDK for Python:
```
`import json
import boto3
import logging
logger = logging.getLogger()
apig = boto3.client('apigateway')
def lambda\_handler(event, context):
try:
result = apig.create\_rest\_api(
name='Simple PetStore (Python SDK, private)',
description='Demo private API created using the AWS SDK for Python',
version='0.00.001',
endpointConfiguration={
'types': [
'PRIVATE',
],
},
)
except botocore.exceptions.ClientError as error:
logger.exception("Couldn't create private API %s.", error)
raise
attribute=["id", "name", "description", "createdDate", "version", "apiKeySource", "endpointConfiguration"]
filtered\_data ={key:result[key] for key in attribute}
result = json.dumps(filtered\_data, default=str, sort\_keys='true')
return result`
```
A successful call returns output similar to the following:
```
`"{\\"apiKeySource\\": \\"HEADER\\", \\"createdDate\\": \\"2024-04-03 17:27:05+00:00\\", \\"description\\": \\"Demo private API created using the AWS SDK for \\", \\"endpointConfiguration\\": {\\"types\\": [\\"PRIVATE\\"]}, \\"id\\": \\"abcd1234\\", \\"name\\": \\"Simple PetStore (Python SDK, private)\\", \\"version\\": \\"0.00.001\\"}"`
```
After completing the preceding steps, you can follow the instructions in [Tutorial: Create a REST API using
AWS SDKs or the AWS CLI](./api-gateway-create-api-cli-sdk.html) to set up methods and integrations for this API, but you
can't deploy your API. To deploy your API, follow step 3 and attach a resource policy to your API.
## Step 3: Set up a resource policy
for a private API
Your current private API is inaccessible to all VPCs. Use a resource policy to grant your VPCs and VPC endpoints access
to your private APIs. You can grant access to a VPC endpoint in any AWS account.
Your resource policy should contain `aws:SourceVpc` or `aws:SourceVpce` conditions to
restrict access. We recommend that you identify specific VPCs and VPC endpoints and don't create a resource
policy that allows access for all VPCs and VPC endpoints.
The following procedure shows how to attach a resource policy to your API.
AWS Management Console
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. Choose a REST API.
3. In the main navigation pane, choose **Resource policy**.
4. Choose **Create policy**.
5. Choose **Select a template** and then choose **Source VPC**.
6. Replace `{{vpcID}}`
(including the curly braces) with your VPC ID.
7. Choose **Save changes**.
AWS CLI
The following [update-rest-api](https://docs.aws.amazon.com/cli/latest/reference/apigateway/update-rest-api.html) command attaches a resource policy to an existing API:
```
`aws apigateway update-rest-api \\
--rest-api-id a1b2c3 \\
--patch-operations op=replace,path=/policy,value='"{\\"`jsonEscapedPolicyDocument`\\"}"'`
```
You might also want to control which resources have access to your VPC endpoint. To control which resources
have access to your VPC endpoint, attach an endpoint policy to your VPC endpoint. For more information, see
[Use VPC endpoint policies for private
APIs in API Gateway](./apigateway-vpc-endpoint-policies.html).
## (Optional) Associate or disassociate a VPC endpoint
with a private API
When you associate a VPC endpoint with your private API, API Gateway generates a new Route53 alias DNS record. You
can use this record to invoke your private APIs just as you do your public APIs without overriding a
`Host` header or passing an `x-apigw-api-id` header.
The generated base URL is in the following format:
```
`https://`{rest-api-id}`-`{vpce-id}`.execute-api.`{region}`.amazonaws.com/`{stage}``
```
Associate a VPC endpoint (AWS Management Console)
You can associate a VPC endpoint with your private API when you create it, or after it's created. The
following procedure shows how to associate a VPC endpoint with a previously created API.
###### To associate a VPC endpoint with a private API
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. Choose your private API.
3. In the main navigation pane, choose **Resource policy**.
4. Edit your resource policy to allow calls from your additional VPC endpoint.
5. In the main navigation pane, choose **API settings**.
6. In the **API details** section, choose **Edit**.
7. For **VPC endpoint IDs**, select additional VPC endpoint IDs.
8. Choose **Save**.
9. Redeploy your API for the changes to take effect.
Dissociate a VPC endpoint (AWS Management Console)
###### To disassociate a VPC endpoint from a private REST API
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. Choose your private API.
3. In the main navigation pane, choose **Resource policy**.
4. Edit your resource policy to remove mentions of the VPC endpoint you want to dissociate from your
private API.
5. In the main navigation pane, choose **API settings**.
6. In the **API details** section, choose **Edit**.
7. For **VPC endpoint IDs**, choose the **X** to dissociate the VPC
endpoint.
8. Choose **Save**.
9. Redeploy your API for the changes to take effect.
Associate a VPC endpoint (AWS CLI)
The following [create-rest-api](https://docs.aws.amazon.com/cli/latest/reference/apigateway/create-rest-api.html)
command associates VPC endpoints at the time of API creation:
```
`aws apigateway create-rest-api \\
--name Petstore \\
--endpoint-configuration '{ "types": ["PRIVATE"], "vpcEndpointIds" : ["vpce-0212a4ababd5b8c3e", "vpce-0393a628149c867ee"] }' \\
--region us-west-2`
```
The output will look like the following:
```
`{
"apiKeySource": "HEADER",
"endpointConfiguration": {
"types": [
"PRIVATE"
],
"vpcEndpointIds": [
"vpce-0212a4ababd5b8c3e",
"vpce-0393a628149c867ee"
]
},
"id": "u67n3ov968",
"createdDate": 1565718256,
"name": "Petstore"
}
`
```
The following [update-rest-api](https://docs.aws.amazon.com/cli/latest/reference/apigateway/update-rest-api.html)
command associates VPC endpoints to an API that you already created:
```
`aws apigateway update-rest-api \\
--rest-api-id u67n3ov968 \\
--patch-operations "op='add',path='/endpointConfiguration/vpcEndpointIds',value='vpce-01d622316a7df47f9'" \\
--region us-west-2`
```
The output will look like the following:
```
`{
"name": "Petstore",
"apiKeySource": "1565718256",
"tags": {},
"createdDate": 1565718256,
"endpointConfiguration": {
"vpcEndpointIds": [
"vpce-0212a4ababd5b8c3e",
"vpce-0393a628149c867ee",
"vpce-01d622316a7df47f9"
],
"types": [
"PRIVATE"
]
},
"id": "u67n3ov968"
}`
```
Redeploy your API for the changes to take effect.
Disassociate a VPC endpoint (AWS CLI)
The following [update-rest-api](https://docs.aws.amazon.com/cli/latest/reference/apigateway/update-rest-api.html)
command dissociates a VPC endpoint from a private API:
```
`aws apigateway update-rest-api \\
--rest-api-id u67n3ov968 \\
--patch-operations "op='remove',path='/endpointConfiguration/vpcEndpointIds',value='vpce-0393a628149c867ee'" \\
--region us-west-2
`
```
The output will look like the following:
```
`{
"name": "Petstore",
"apiKeySource": "1565718256",
"tags": {},
"createdDate": 1565718256,
"endpointConfiguration": {
"vpcEndpointIds": [
"vpce-0212a4ababd5b8c3e",
"vpce-01d622316a7df47f9"
],
"types": [
"PRIVATE"
]
},
"id": "u67n3ov968"
}
`
```
Redeploy your API for the changes to take effect.
## Step 4: Deploy a private API
To deploy your API, you create an API deployment and associate it with a stage. The following procedure
shows how to deploy your private API.
AWS Management Console
###### To deploy a private API
1. Choose your API.
2. Choose **Deploy API**.
3. For **Stage**, select **New stage**.
4. For **Stage name**, enter a stage name.
5. (Optional) For **Description**, enter a description.
6. Choose **Deploy**.
AWS CLI
The following [create-deployment](https://docs.aws.amazon.com/cli/latest/reference/apigateway/create-deployment.html) command deploys a private API:
```
`aws apigateway create-deployment --rest-api-id a1b2c3 \\
--stage-name test \\
--stage-description 'Private API test stage' \\
--description 'First deployment'`
```
## Troubleshoot your private API
The following provides troubleshooting advice for errors and issues that you might encounter when
creating a private API.
### Issue: I can't
connect to my public API from an API Gateway VPC endpoint
When you create your VPC, you can configure the DNS settings. We recommend that you turn on private DNS
for your VPC. If you choose turn off private DNS, you're only able to access your API via public
DNS.
If you enable private DNS, you can't access the default endpoint of a public API Gateway API from your VPC
endpoint. You can access an API with a custom domain name.
If you create a Regional custom domain name, use an A type alias record, if you create an
edge-optimized custom domain name, there are no restrictions for your record type. You can access
these public APIs with private DNS enabled. For more information, see [Issue: I
connect to my public API from an API Gateway VPC endpoint](https://repost.aws/knowledge-center/api-gateway-vpc-connections).
### Issue: My API returns `{"Message":"User:
anonymous is not authorized to perform: execute-api:Invoke on resource:
arn:aws:execute-api:us-east-1:\*\*\*\*\*\*\*\*/\*\*\*\*/\*\*\*\*/"}`
In your resource policy, if you set the Principal to an AWS principal, such as the following:
JSON
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Principal": {
"AWS": [
"arn:aws:iam::`111122223333`:role/developer",
"arn:aws:iam::`111122223333`:role/Admin"
]
},
"Action": "execute-api:Invoke",
"Resource": [
"execute-api:/`stage`/GET/pets"
]
}
]
}`
`
```
You must use `AWS\_IAM` authorization for every method in your API, or else your API returns
the previous error message. For more instructions on how to turn on `AWS\_IAM` authorization for a
method, see [Methods for REST APIs in API Gateway](./how-to-method-settings.html).
### Issue: I can't tell if my VPC endpoint is associated with my API
If you associate or dissociate a VPC endpoint with your private API, you need to redeploy your API. The
update operation might take few minutes to complete due to DNS propagation. During this time, your API is
available, but DNS propagation for the newly generated DNS URLs may still be in progress. If after several
minutes, your new URLs are not resolving in DNS, we recommend that you redeploy your API.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Private REST APIs
Custom domain names for private APIs
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.