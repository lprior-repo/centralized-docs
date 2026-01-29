---
url: https://docs.aws.amazon.com/lambda/latest/dg/configuration-vpc-internet.html
title: Enable internet access for VPC-connected Lambda functions
word_count: 3174
filtered: true
elements_removed: 0
density_score: 0.86
---

Enable internet access for VPC-connected Lambda functions - AWS Lambda
Enable internet access for VPC-connected Lambda functions - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#configuration-vpc-internet)
# Enable internet access for VPC-connected Lambda functions
By default, Lambda functions run in a Lambda-managed VPC that has internet access. To access resources in a VPC in your account, you can add a VPC configuration to a function. This restricts the function to resources within that VPC, unless the VPC has internet access. This page explains how to provide internet access to VPC-connected Lambda functions.
### Create the VPC
The **Create VPC workflow** creates all VPC resources required for a
Lambda function to access the public internet from a private subnet, including subnets,
NAT gateway, internet gateway, and route table entries.
###### To create the VPC
1. Open the Amazon VPC console at
[https://console.aws.amazon.com/vpc/](https://console.aws.amazon.com/vpc/).
2. On the dashboard, choose **Create VPC**.
3. For **Resources to create**, choose **VPC
and more**.
4. **Configure the VPC**
1. For **Name tag auto-generation**, enter a name for the VPC.
2. For **IPv4 CIDR block**, you can keep the default suggestion, or
alternatively you can enter the CIDR block required by your application or
network.
3. If your application communicates by using IPv6 addresses, choose **IPv6
CIDR block**, **Amazon-provided IPv6 CIDR block**.
4. **Configure the subnets**
1. For **Number of Availability Zones**, choose
**2**. We recommend at least two AZs for high availability.
2. For **Number of public subnets**, choose
**2**.
3. For **Number of private subnets**, choose
**2**.
4. You can keep the default CIDR block for the public subnet, or alternatively you
can expand **Customize subnet CIDR blocks** and enter a CIDR block.
For more information, see [Subnet CIDR blocks](https://docs.aws.amazon.com/vpc/latest/userguide/subnet-sizing.html) .
5. For **NAT gateways**, choose **1 per AZ** to improve
resiliency.
6. For **Egress only internet gateway**, choose
**Yes** if you opted to include an IPv6 CIDR block.
7. For **VPC endpoints**, keep the default (**S3 Gateway**). There is no cost for this option. For more information, see [Types of VPC endpoints for Amazon S3](https://docs.aws.amazon.com/AmazonS3/latest/userguide/privatelink-interface-endpoints.html#types-of-vpc-endpoints-for-s3).
8. For **DNS options**, keep the default settings.
9. Choose **Create VPC**.
###### To configure a VPC when you create a function
1. Open the [Functions page](https://console.aws.amazon.com/lambda/home#/functions) of the Lambda console.
2. Choose **Create function**.
3. Under **Basic information**, for **Function name**, enter a name for
your function.
4. Expand **Advanced settings**.
5. Select **Enable VPC**, and then choose a VPC.
6. (Optional) To allow [outbound IPv6 traffic](./configuration-vpc.html#configuration-vpc-ipv6), select **Allow IPv6 traffic for dual-stack subnets**.
7. For **Subnets**, select all private subnets. The private subnets can access the internet through the NAT gateway. Connecting a function to a public subnet doesn't give it internet access.
###### Note
If you selected **Allow IPv6 traffic for dual-stack subnets**, all selected subnets must have an IPv4 CIDR block and an IPv6 CIDR block.
8. For **Security groups**, select a security group that allows outbound traffic.
9. Choose **Create function**.
Lambda automatically creates an execution role with the [AWSLambdaVPCAccessExecutionRole](https://docs.aws.amazon.com/aws-managed-policy/latest/reference/AWSLambdaVPCAccessExecutionRole.html) AWS managed policy. The permissions in this policy are required only to create elastic network interfaces for the VPC configuration, not to invoke your function. To apply least-privilege permissions, you can remove the **AWSLambdaVPCAccessExecutionRole** policy from your execution role after you create the function and VPC configuration. For more information, see [Required IAM permissions](./configuration-vpc.html#configuration-vpc-permissions).
###### To configure a VPC for an existing function
To add a VPC configuration to an existing function, the function's execution role must have [ permission to create and manage elastic network interfaces](./configuration-vpc.html#configuration-vpc-permissions). The [AWSLambdaVPCAccessExecutionRole](https://docs.aws.amazon.com/aws-managed-policy/latest/reference/AWSLambdaVPCAccessExecutionRole.html) AWS managed policy includes
the required permissions. To apply least-privilege permissions, you can remove the **AWSLambdaVPCAccessExecutionRole** policy from your execution role after you create the VPC configuration.
1. Open the [Functions page](https://console.aws.amazon.com/lambda/home#/functions) of the Lambda console.
2. Choose a function.
3. Choose the **Configuration** tab, and then choose **VPC**.
4. Under **VPC**, choose **Edit**.
5. Select the VPC.
6. (Optional) To allow [outbound IPv6 traffic](./configuration-vpc.html#configuration-vpc-ipv6), select **Allow IPv6 traffic for dual-stack subnets**.
7. For **Subnets**, select all private subnets. The private subnets can access the internet through the NAT gateway. Connecting a function to a public subnet doesn't give it internet access.
###### Note
If you selected **Allow IPv6 traffic for dual-stack subnets**, all selected subnets must have an IPv4 CIDR block and an IPv6 CIDR block.
8. For **Security groups**, select a security group that allows outbound traffic.
9. Choose **Save**.
### Test the function
Use the following sample code to confirm that your VPC-connected function can reach the public internet. If successful, the code returns a `200` status code. If unsuccessful, the function times out.
Node.js
1. In the **Code source** pane on the Lambda console, paste the following code into the **index.mjs** file. The function makes an HTTP GET request to a public endpoint and returns the HTTP response code to test if the function has access to the public internet.
![Lambda console code editor.](https://docs.aws.amazon.com/images/lambda/latest/dg/images/code-source-nodejs.png)
###### Example — HTTP request with async/await
```
`const url = "https://aws.amazon.com/";
export const handler = async(event) =&gt; {
try {
const res = await fetch(url);
console.info("status", res.status);
return res.status;
}
catch (e) {
console.error(e);
return 500;
}
};`
```
2. In the **DEPLOY** section, choose **Deploy** to update your function's code:
![Deploy button in the Lambda console code editor](https://docs.aws.amazon.com/images/lambda/latest/dg/images/getting-started-tutorial/deploy-console.png)
3. Choose the **Test** tab.
![Lambda console Test tab.](https://docs.aws.amazon.com/images/lambda/latest/dg/images/test-tab.png)
4. Choose **Test**.
5. The function returns a `200` status code. This means that the function has outbound internet access.
![Lambda console Test tab.](https://docs.aws.amazon.com/images/lambda/latest/dg/images/test-successful-200.png)
If the function can't reach the public internet, you get an error message like this:
```
`{
"errorMessage": "2024-04-11T17:22:20.857Z abe12jlc-640a-8157-0249-9be825c2y110 Task timed out after 3.01 seconds"
}`
```
Python
1. In the **Code source** pane on the Lambda console, paste the following code into the **lambda\_function.py** file. The function makes an HTTP GET request to a public endpoint and returns the HTTP response code to test if the function has access to the public internet.
![Lambda console code editor.](https://docs.aws.amazon.com/images/lambda/latest/dg/images/code-source-python.png)
```
`import urllib.request
def lambda\_handler(event, context):
try:
response = urllib.request.urlopen('https://aws.amazon.com')
status\_code = response.getcode()
print('Response Code:', status\_code)
return status\_code
except Exception as e:
print('Error:', e)
raise e`
```
2. In the **DEPLOY** section, choose **Deploy** to update your function's code:
![Deploy button in the Lambda console code editor](https://docs.aws.amazon.com/images/lambda/latest/dg/images/getting-started-tutorial/deploy-console.png)
3. Choose the **Test** tab.
![Lambda console Test tab.](https://docs.aws.amazon.com/images/lambda/latest/dg/images/test-tab.png)
4. Choose **Test**.
5. The function returns a `200` status code. This means that the function has outbound internet access.
![Lambda console Test tab.](https://docs.aws.amazon.com/images/lambda/latest/dg/images/test-successful-200.png)
If the function can't reach the public internet, you get an error message like this:
```
`{
"errorMessage": "2024-04-11T17:22:20.857Z abe12jlc-640a-8157-0249-9be825c2y110 Task timed out after 3.01 seconds"
}`
```
If you already have a VPC but you need to configure public internet access for a Lambda function, follow these steps. This procedure assumes that your VPC has at least two subnets. If you don't have two subnets, see [Create a subnet](https://docs.aws.amazon.com/vpc/latest/userguide/create-subnets.html) in the *Amazon VPC User Guide*.
### Verify the route table configuration
1. Open the Amazon VPC console at
[https://console.aws.amazon.com/vpc/](https://console.aws.amazon.com/vpc/).
2. Choose the **VPC ID**.
![VPC console list of VPCs.](https://docs.aws.amazon.com/images/lambda/latest/dg/images/vpc-id.png)
3. Scroll down to the **Resource map** section. Note the route table mappings. Open each route table that is mapped to a subnet.
![VPC console resource map section](https://docs.aws.amazon.com/images/lambda/latest/dg/images/route-table-associations.png)
4. Scroll down to the **Routes** tab. Review the routes to determine if your VPC has both of the following route tables. Each of these requirements must be satisfied by a separate route table.
* Internet-bound traffic (`0.0.0.0/0` for IPv4, `::/0` for IPv6) is routed to an internet gateway (`igw-xxxxxxxxxx`). This means that the subnet associated with the route table is a public subnet.
###### Note
If your subnet doesn't have an IPv6 CIDR block, you will only see the IPv4 route (`0.0.0.0/0`).
###### Example public subnet route table
![Public subnet route table with route to internet gateway](https://docs.aws.amazon.com/images/lambda/latest/dg/images/routes-public.png)
* Internet-bound traffic for IPv4 (`0.0.0.0/0`) is routed to a NAT gateway (`nat-xxxxxxxxxx`) that is associated with a public subnet. This means that the subnet is a private subnet that can access the internet through the NAT gateway.
###### Note
If your subnet has an IPv6 CIDR block, the route table must also route internet-bound IPv6 traffic (`::/0`) to an egress-only internet gateway (`eigw-xxxxxxxxxx`). If your subnet doesn't have an IPv6 CIDR block, you will only see the IPv4 route (`0.0.0.0/0`).
###### Example private subnet route table
![Private subnet route table with route to NAT gateway](https://docs.aws.amazon.com/images/lambda/latest/dg/images/routes-private.png)
* Repeat the previous step until you have reviewed each route table associated with a subnet in your VPC and confirmed that you have a route table with an internet gateway and a route table with a NAT gateway.
If you don't have two route tables, one with a route to an internet gateway and one with a route to a NAT gateway, follow these steps to create the missing resources and route table entries.
Follow these steps to create a route table and associate it with a subnet.
###### To create a custom route table using the Amazon VPC console
1. Open the Amazon VPC console at
[https://console.aws.amazon.com/vpc/](https://console.aws.amazon.com/vpc/).
2. In the navigation pane, choose **Route tables**.
3. Choose **Create route table**.
4. (Optional) For **Name**, enter a name for your route
table.
5. For **VPC**, choose your VPC.
6. (Optional) To add a tag, choose **Add new tag** and enter
the tag key and tag value.
7. Choose **Create route table**.
8. On the **Subnet associations** tab, choose **Edit
subnet associations**.
![Attach internet gateway to VPC](https://docs.aws.amazon.com/images/lambda/latest/dg/images/route-table-subnet.png)
9. Select the check box for the subnet to associate with the route table.
10. Choose **Save associations**.
Follow these steps to create an internet gateway, attach it to your VPC, and add it to your public subnet's route table.
###### To create an internet gateway
1. Open the Amazon VPC console at
[https://console.aws.amazon.com/vpc/](https://console.aws.amazon.com/vpc/).
2. In the navigation pane, choose **Internet gateways**.
3. Choose **Create internet gateway**.
4. (Optional) Enter a name for your internet gateway.
5. (Optional) To add a tag, choose **Add new tag** and enter the tag key and value.
6. Choose **Create internet gateway**.
7. Choose **Attach to a VPC** from the banner at the top of the screen, select an
available VPC, and then choose **Attach internet gateway**.
![Attach internet gateway to VPC](https://docs.aws.amazon.com/images/lambda/latest/dg/images/igw-attach-vpc.png)
8. Choose the **VPC ID**.
![Internet gateway details page](https://docs.aws.amazon.com/images/lambda/latest/dg/images/igw-subnet-1.png)
9. Choose the **VPC ID** again to open the VPC details page.
![Filtered VPC list in Amazon VPC Console](https://docs.aws.amazon.com/images/lambda/latest/dg/images/igw-your-vpcs.png)
10. Scroll down to the **Resource map** section and then choose a subnet. The subnet details are displayed in a new tab.
![VPC console Resource map with list of subnets.](https://docs.aws.amazon.com/images/lambda/latest/dg/images/vpc-subnets.png)
11. Choose the link under **Route table**.
![Link to route table on subnet details page](https://docs.aws.amazon.com/images/lambda/latest/dg/images/subnet-route-table.png)
12. Choose the **Route table ID** to open the route table details page.
![Filtered route table list](https://docs.aws.amazon.com/images/lambda/latest/dg/images/route-table-id.png)
13. Under **Routes**, choose **Edit routes**.
![Routes list with Edit routes button](https://docs.aws.amazon.com/images/lambda/latest/dg/images/edit-routes.png)
14. Choose **Add route**, and then enter `0.0.0.0/0` in the **Destination** box.
![Add destination for new route](https://docs.aws.amazon.com/images/lambda/latest/dg/images/create-route-1.png)
15. For **Target**, select **Internet gateway**, and then choose the internet gateway that you created earlier. If your subnet has an IPv6 CIDR block, you must also add a route for `::/0` to the same internet gateway.
![Add target for new route](https://docs.aws.amazon.com/images/lambda/latest/dg/images/create-route-2.png)
16. Choose **Save changes**.
Follow these steps to create a NAT gateway, associate it with a public subnet, and then add it to your private subnet's route table.
###### To create a NAT gateway and associate it with a public subnet
1. In the navigation pane, choose **NAT gateways**.
2. Choose **Create NAT gateway**.
3. (Optional) Enter a name for your NAT gateway.
4. For **Subnet**, select a public subnet in your VPC. (A public subnet is a subnet that has a direct route to an internet gateway in its route table.)
###### Note
NAT gateways are associated with a public subnet, but the route table entry is in the private subnet.
5. For **Elastic IP allocation ID**, select an elastic IP address or choose **Allocate Elastic IP**.
6. Choose **Create NAT gateway**.
###### To add a route to the NAT gateway in the private subnet's route table
1. In the navigation pane, choose **Subnets**.
2. Select a private subnet in your VPC. (A private subnet is a subnet that doesn't have a route to an internet gateway in its route table.)
3. Choose the link under **Route table**.
![Link to route table on subnet details page](https://docs.aws.amazon.com/images/lambda/latest/dg/images/subnet-route-table.png)
4. Choose the **Route table ID** to open the route table details page.
![Filtered route table list](https://docs.aws.amazon.com/images/lambda/latest/dg/images/route-table-id.png)
5. Scroll down and choose the **Routes** tab, then choose **Edit routes**
![Routes tab on route table details page](https://docs.aws.amazon.com/images/lambda/latest/dg/images/route-table-edit-routes.png)
6. Choose **Add route**, and then enter `0.0.0.0/0` in the **Destination** box.
![Add destination for new route](https://docs.aws.amazon.com/images/lambda/latest/dg/images/create-route-1.png)
7. For **Target**, select **NAT gateway**, and then choose the NAT gateway that you created earlier.
![Add target for new route](https://docs.aws.amazon.com/images/lambda/latest/dg/images/create-route-nat.png)
8. Choose **Save changes**.
Follow these steps to create an egress-only internet gateway and add it to your private subnet's route table.
###### To create an egress-only internet gateway
1. In the navigation pane, choose **Egress-only internet gateways**.
2. Choose **Create egress only internet gateway**.
3. (Optional) Enter a name.
4. Select the VPC in which to create the egress-only internet gateway.
5. Choose **Create egress only internet gateway**.
6. Choose the link under **Attached VPC ID**.
![Egress-only internet gateway details page](https://docs.aws.amazon.com/images/lambda/latest/dg/images/eigw-details.png)
7. Choose the link under **VPC ID** to open the VPC details page.
8. Scroll down to the **Resource map** section and then choose a private subnet. (A private subnet is a subnet that doesn't have a route to an internet gateway in its route table.) The subnet details are displayed in a new tab.
![VPC console Resource map with list of subnets.](https://docs.aws.amazon.com/images/lambda/latest/dg/images/vpc-subnet-private.png)
9. Choose the link under **Route table**.
![Link to route table on subnet details page](https://docs.aws.amazon.com/images/lambda/latest/dg/images/private-subnet-route-table.png)
10. Choose the **Route table ID** to open the route table details page.
![Filtered route table list](https://docs.aws.amazon.com/images/lambda/latest/dg/images/route-table-id.png)
11. Under **Routes**, choose **Edit routes**.
![Routes list with Edit routes button](https://docs.aws.amazon.com/images/lambda/latest/dg/images/edit-routes.png)
12. Choose **Add route**, and then enter `::/0` in the **Destination** box.
![Add destination for new route](https://docs.aws.amazon.com/images/lambda/latest/dg/images/create-route-1.png)
13. For **Target**, select **Egress Only Internet Gateway**, and then choose the gateway that you created earlier.
![Add target for new route](https://docs.aws.amazon.com/images/lambda/latest/dg/images/eigw-route.png)
14. Choose **Save changes**.
###### To configure a VPC when you create a function
1. Open the [Functions page](https://console.aws.amazon.com/lambda/home#/functions) of the Lambda console.
2. Choose **Create function**.
3. Under **Basic information**, for **Function name**, enter a name for
your function.
4. Expand **Advanced settings**.
5. Select **Enable VPC**, and then choose a VPC.
6. (Optional) To allow [outbound IPv6 traffic](./configuration-vpc.html#configuration-vpc-ipv6), select **Allow IPv6 traffic for dual-stack subnets**.
7. For **Subnets**, select all private subnets. The private subnets can access the internet through the NAT gateway. Connecting a function to a public subnet doesn't give it internet access.
###### Note
If you selected **Allow IPv6 traffic for dual-stack subnets**, all selected subnets must have an IPv4 CIDR block and an IPv6 CIDR block.
8. For **Security groups**, select a security group that allows outbound traffic.
9. Choose **Create function**.
Lambda automatically creates an execution role with the [AWSLambdaVPCAccessExecutionRole](https://docs.aws.amazon.com/aws-managed-policy/latest/reference/AWSLambdaVPCAccessExecutionRole.html) AWS managed policy. The permissions in this policy are required only to create elastic network interfaces for the VPC configuration, not to invoke your function. To apply least-privilege permissions, you can remove the **AWSLambdaVPCAccessExecutionRole** policy from your execution role after you create the function and VPC configuration. For more information, see [Required IAM permissions](./configuration-vpc.html#configuration-vpc-permissions).
###### To configure a VPC for an existing function
To add a VPC configuration to an existing function, the function's execution role must have [ permission to create and manage elastic network interfaces](./configuration-vpc.html#configuration-vpc-permissions). The [AWSLambdaVPCAccessExecutionRole](https://docs.aws.amazon.com/aws-managed-policy/latest/reference/AWSLambdaVPCAccessExecutionRole.html) AWS managed policy includes
the required permissions. To apply least-privilege permissions, you can remove the **AWSLambdaVPCAccessExecutionRole** policy from your execution role after you create the VPC configuration.
1. Open the [Functions page](https://console.aws.amazon.com/lambda/home#/functions) of the Lambda console.
2. Choose a function.
3. Choose the **Configuration** tab, and then choose **VPC**.
4. Under **VPC**, choose **Edit**.
5. Select the VPC.
6. (Optional) To allow [outbound IPv6 traffic](./configuration-vpc.html#configuration-vpc-ipv6), select **Allow IPv6 traffic for dual-stack subnets**.
7. For **Subnets**, select all private subnets. The private subnets can access the internet through the NAT gateway. Connecting a function to a public subnet doesn't give it internet access.
###### Note
If you selected **Allow IPv6 traffic for dual-stack subnets**, all selected subnets must have an IPv4 CIDR block and an IPv6 CIDR block.
8. For **Security groups**, select a security group that allows outbound traffic.
9. Choose **Save**.
### Test the function
Use the following sample code to confirm that your VPC-connected function can reach the public internet. If successful, the code returns a `200` status code. If unsuccessful, the function times out.
Node.js
1. In the **Code source** pane on the Lambda console, paste the following code into the **index.mjs** file. The function makes an HTTP GET request to a public endpoint and returns the HTTP response code to test if the function has access to the public internet.
![Lambda console code editor.](https://docs.aws.amazon.com/images/lambda/latest/dg/images/code-source-nodejs.png)
###### Example — HTTP request with async/await
```
`const url = "https://aws.amazon.com/";
export const handler = async(event) =&gt; {
try {
const res = await fetch(url);
console.info("status", res.status);
return res.status;
}
catch (e) {
console.error(e);
return 500;
}
};`
```
2. In the **DEPLOY** section, choose **Deploy** to update your function's code:
![Deploy button in the Lambda console code editor](https://docs.aws.amazon.com/images/lambda/latest/dg/images/getting-started-tutorial/deploy-console.png)
3. Choose the **Test** tab.
![Lambda console Test tab.](https://docs.aws.amazon.com/images/lambda/latest/dg/images/test-tab.png)
4. Choose **Test**.
5. The function returns a `200` status code. This means that the function has outbound internet access.
![Lambda console Test tab.](https://docs.aws.amazon.com/images/lambda/latest/dg/images/test-successful-200.png)
If the function can't reach the public internet, you get an error message like this:
```
`{
"errorMessage": "2024-04-11T17:22:20.857Z abe12jlc-640a-8157-0249-9be825c2y110 Task timed out after 3.01 seconds"
}`
```
Python
1. In the **Code source** pane on the Lambda console, paste the following code into the **lambda\_function.py** file. The function makes an HTTP GET request to a public endpoint and returns the HTTP response code to test if the function has access to the public internet.
![Lambda console code editor.](https://docs.aws.amazon.com/images/lambda/latest/dg/images/code-source-python.png)
```
`import urllib.request
def lambda\_handler(event, context):
try:
response = urllib.request.urlopen('https://aws.amazon.com')
status\_code = response.getcode()
print('Response Code:', status\_code)
return status\_code
except Exception as e:
print('Error:', e)
raise e`
```
2. In the **DEPLOY** section, choose **Deploy** to update your function's code:
![Deploy button in the Lambda console code editor](https://docs.aws.amazon.com/images/lambda/latest/dg/images/getting-started-tutorial/deploy-console.png)
3. Choose the **Test** tab.
![Lambda console Test tab.](https://docs.aws.amazon.com/images/lambda/latest/dg/images/test-tab.png)
4. Choose **Test**.
5. The function returns a `200` status code. This means that the function has outbound internet access.
![Lambda console Test tab.](https://docs.aws.amazon.com/images/lambda/latest/dg/images/test-successful-200.png)
If the function can't reach the public internet, you get an error message like this:
```
`{
"errorMessage": "2024-04-11T17:22:20.857Z abe12jlc-640a-8157-0249-9be825c2y110 Task timed out after 3.01 seconds"
}`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Attaching functions to resources in another account
Inbound networking
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.