---
url: https://docs.aws.amazon.com/lambda/latest/dg/lambda-managed-instances-networking.html
title: Networking for Lambda Managed Instances
word_count: 1006
filtered: true
elements_removed: 0
density_score: 0.89
---

Networking for Lambda Managed Instances - AWS Lambda
Networking for Lambda Managed Instances - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#lambda-managed-instances-networking)
[Connectivity options](#lambda-managed-instances-connectivity-options)[Public subnet with an internet gateway](#lambda-managed-instances-public-subnet-igw)[VPC endpoints](#lambda-managed-instances-vpc-endpoints)[Private subnet with NAT gateway](#lambda-managed-instances-private-subnet-nat)[Choosing a connectivity option](#lambda-managed-instances-choosing-connectivity)[Next steps](#lambda-managed-instances-networking-next-steps)
# Networking for Lambda Managed Instances
When running Lambda Managed Instances functions, you need to configure network connectivity to enable your functions to access resources outside the VPC. This includes AWS services such as Amazon S3 and DynamoDB. The connectivity is also needed for transmitting telemetry data to CloudWatch Logs and X-Ray.
## Connectivity options
There are three primary approaches for configuring VPC connectivity, each with different trade-offs for cost, security, and complexity.
## Public subnet with an internet gateway
This option uses a public subnet with direct internet access through an internet gateway. You can choose between IPv4 and IPv6 configurations.
### IPv4 with internet gateway
**To configure IPv4 connectivity with an internet gateway**
1. Create or use an existing public subnet with an IPv4 CIDR block.
2. Attach an internet gateway to your VPC.
3. Update the route table to route `0.0.0.0/0` traffic to the internet gateway.
4. Ensure resources have public IPv4 addresses or Elastic IP addresses assigned.
5. Configure security groups to allow outbound traffic on the required ports.
This configuration provides bidirectional connectivity, allowing both outbound connections from your functions and inbound connections from the internet.
### IPv6 with internet gateway
**To configure IPv6 connectivity with an internet gateway**
1. Enable IPv6 on your VPC.
2. Create or use an existing public subnet with an IPv6 CIDR block assigned.
3. Attach an internet gateway to your VPC (the same internet gateway can handle both IPv4 and IPv6).
4. Update the route table to route `::/0` traffic to the internet gateway.
5. Verify that the AWS services you need to access support IPv6 in your Region.
6. Configure security groups to allow outbound traffic on the required ports.
This configuration provides bidirectional connectivity using IPv6 addressing.
### IPv6 with egress-only internet gateway
**To configure IPv6 connectivity with an egress-only internet gateway**
1. Enable IPv6 on your VPC.
2. Create or use an existing public subnet with an IPv6 CIDR block assigned.
3. Attach an egress-only internet gateway to your VPC.
4. Update the route table to route `::/0` traffic to the egress-only internet gateway.
5. Verify that the AWS services you need to access support IPv6 in your Region.
6. Configure security groups to allow outbound traffic on the required ports.
This configuration provides outbound-only connectivity, preventing inbound connections from the internet while allowing your functions to initiate outbound connections.
## VPC endpoints
VPC endpoints enable you to privately connect your VPC to supported AWS services without requiring an internet gateway, NAT device, VPN connection, or AWS Direct Connect connection. Traffic between your VPC and the AWS service does not leave the Amazon network.
**To configure VPC endpoints**
1. Open the Amazon VPC console at [console.aws.amazon.com/vpc/](http://console.aws.amazon.com/vpc/).
2. In the navigation pane, choose **Endpoints**.
3. Choose **Create endpoint**.
4. For **Service category**, choose **AWS services**.
5. For **Service name**, select the service endpoint you need (for example, `com.amazonaws.region.s3` for Amazon S3).
6. For **VPC**, select your VPC.
7. For **Subnets**, select the subnets where you want to create endpoint network interfaces. For high availability, select subnets in multiple Availability Zones.
8. For **Security groups**, select the security groups to associate with the endpoint network interfaces. The security groups must allow inbound traffic from your function's security group on the required ports.
9. Choose **Create endpoint**.
Repeat these steps for each AWS service that your functions need to access.
## Private subnet with NAT gateway
This option uses a NAT gateway to provide internet access for resources in private subnets while keeping the resources private.
**To configure a private subnet with NAT gateway**
1. Create a public subnet (if one doesn't already exist) with a CIDR block.
2. Attach an internet gateway to your VPC.
3. Create a NAT gateway in the public subnet and assign an Elastic IP address.
4. Update the public subnet route table to add a route: `0.0.0.0/0` → internet gateway.
5. Create or use an existing private subnet with a CIDR block.
6. Update the private subnet route table to add a route: `0.0.0.0/0` → NAT gateway.
7. Configure security groups to allow outbound traffic on the required ports.
For high availability, deploy one NAT gateway in each Availability Zone and configure route tables per Availability Zone to use the local NAT gateway. This prevents cross-AZ data transfer charges and improves resilience.
## Choosing a connectivity option
Consider the following factors when choosing a connectivity option:
**Public subnet with internet gateway**
* Simplest configuration with lowest cost
* Suitable for development and testing environments
* Resources can receive inbound connections from the internet (security consideration)
* Supports both IPv4 and IPv6
**VPC endpoints**
* Highest security, traffic stays within the AWS network
* Lower latency compared to internet routing
* Recommended for production environments with strict security requirements
* Higher cost per endpoint, per Availability Zone, and per GB processed
* Requires an endpoint in each Availability Zone for high availability
**Private subnet with NAT gateway**
* Resources remain private with no inbound internet access
* Standard enterprise architecture pattern
* Supports all IPv4 internet traffic
* Moderate cost with NAT gateway hourly and data processing charges
* Supports IPv4 only
## Next steps
* Learn about [capacity providers for Lambda Managed Instances](./lambda-managed-instances-capacity-providers.html)
* Understand [scaling for Lambda Managed Instances](./lambda-managed-instances-scaling.html)
* Review runtime-specific guides for [Java](./lambda-managed-instances-java-runtime.html), [Node.js](./lambda-managed-instances-nodejs-runtime.html), and [Python](./lambda-managed-instances-python-runtime.html)
* Understand [security and permissions for Lambda Managed Instances](./lambda-managed-instances-security.html)
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
.NET runtime
Monitoring
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.