---
url: https://docs.aws.amazon.com/lambda/latest/dg/troubleshooting-networking.html
title: Troubleshoot networking issues in Lambda
word_count: 928
filtered: true
elements_removed: 0
density_score: 0.86
---

Troubleshoot networking issues in Lambda - AWS Lambda
Troubleshoot networking issues in Lambda - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#troubleshooting-networking)
[VPC: Function loses internet access or times out](#troubleshooting-networking-cfn)[VPC: TCP or UDP connection intermittently fails](#troubleshooting-networking-tcp-udp)[VPC: Function needs access to AWS services without using the internet](#troubleshooting-networking-access)[VPC: Elastic network interface limit reached](#troubleshooting-networking-limit)[EC2: Elastic network interface with type of "lambda"](#troubleshooting-networking-eni)[DNS: Fail to connect to hosts with UNKNOWNHOSTEXCEPTION](#troubleshooting-networking-dns-tcp)
# Troubleshoot networking issues in Lambda
By default, Lambda runs your functions in an internal virtual private cloud (VPC) with connectivity to AWS
services and the internet. To access local network resources, you can [configure
your function to connect to a VPC in your account](./configuration-vpc.html). When you use this feature, you manage the function's
internet access and network connectivity with Amazon Virtual Private Cloud (Amazon VPC) resources.
Network connectivity errors can result from issues with your VPC's routing configuration, security group
rules, AWS Identity and Access Management (IAM) role permissions, or network address translation (NAT), or from the availability of
resources such as IP addresses or network interfaces. Depending on the issue, you might see a specific error or
timeout if a request can't reach its destination.
###### Topics
* [VPC: Function loses internet access or times out](#troubleshooting-networking-cfn)
* [VPC: TCP or UDP connection intermittently fails](#troubleshooting-networking-tcp-udp)
* [VPC: Function needs access to AWS services without using the internet](#troubleshooting-networking-access)
* [VPC: Elastic network interface limit reached](#troubleshooting-networking-limit)
* [EC2: Elastic network interface with type of "lambda"](#troubleshooting-networking-eni)
* [DNS: Fail to connect to hosts with UNKNOWNHOSTEXCEPTION](#troubleshooting-networking-dns-tcp)
## VPC: Function loses internet access or times out
**Issue:**
*Your Lambda function loses internet access after connecting to a VPC.*
**Error:**
*Error: connect ETIMEDOUT 176.32.98.189:443*
**Error:**
*Error: Task timed out after 10.00 seconds*
**Error:**
*ReadTimeoutError: Read timed out. (read timeout=15)*
When you connect a function to a VPC, all outbound requests go through the VPC. To connect to the internet,
configure your VPC to send outbound traffic from the function's subnet to a NAT gateway in a public subnet. For
more information and sample VPC configurations, see [Enable internet access for VPC-connected Lambda functions](./configuration-vpc-internet.html).
If some of your TCP connections are timing out, see [VPC: TCP or UDP connection intermittently fails](#troubleshooting-networking-tcp-udp) if
your subnet is using a network access control list (NACL). Otherwise, this is likely due to packet fragmentation.
Lambda functions cannot handle incoming fragmented TCP requests, since Lambda does not support IP fragmentation
for TCP or ICMP.
###### Note
This issue applies only if your subnet uses a [network access control list (ACL)](https://docs.aws.amazon.com//vpc/latest/userguide/vpc-network-acls.html#nacl-basics).
Network ACLs aren't required for Lambda to connect to your subnets.
**Issue:**
*Lambda intermittently loses connection to your VPC subnets, which you have configured a
network access control list (ACL) for.*
For VPC-enabled Lambda functions, AWS creates [hyperplane ENIs](./configuration-vpc.html#configuration-vpc-enis)
in the customer's account, and uses ephemeral ports `1024` to `65535` to connect Lambda
to the customer's VPC. If you use network ACLs in the target subnet, you must allow the port range
`1024` to `65535` for both TCP and UDP. Not allowing this full port range can cause
intermittent connection failures.
## VPC: Function needs access to AWS services without using the internet
**Issue:**
*Your Lambda function needs access to AWS services without using the internet.*
To connect a function to AWS services from a private subnet with no internet access, use VPC endpoints.
## VPC: Elastic network interface limit reached
**Error:**
*ENILimitReachedException: The elastic network interface limit was reached for the function's
VPC.*
When you connect a Lambda function to a VPC, Lambda creates an elastic network interface for each combination of
subnet and security group attached to the function. The default service quota is 250 network interfaces per VPC.
To request a quota increase, use the [Service Quotas console](https://console.aws.amazon.com/servicequotas/home/services/lambda/quotas/L-9FEE3D26).
## EC2: Elastic network interface with type of "lambda"
**Error Code:**
*Client.OperationNotPermitted*
**Error message:**
*The security group can not be modified for this type of interface*
You will receive this error if you attempt to modify an elastic network interface (ENI) that is managed by Lambda. The `ModifyNetworkInterfaceAttribute` is not included in the Lambda API for update operations on elastic network interfaces created by Lambda.
## DNS: Fail to connect to hosts with UNKNOWNHOSTEXCEPTION
**Error Message:**
*UNKNOWNHOSTEXCEPTION*
Lambda functions support a maximum of 20 concurrent TCP connections for DNS resolution. Your function may be
exhausting that limit. Most common DNS requests are done over UDP. If your function is only making UDP DNS
connections, this is unlikely to be your issue. This error is commonly thrown due to misconfiguration or degraded
infrastructure, so before examining your DNS traffic in depth, confirm that your DNS infrastructure is properly
configured and healthy and that your Lambda function is referring to a host specified in DNS.
If you diagnose your issue as related to the TCP connection maximum, note that you cannot request an increase
to this limit. If your Lambda function is falling back to TCP DNS because of large DNS payloads, confirm that your
solution is using libraries that support EDNS. For more information about EDNS, see [the RFC 6891 standard](https://datatracker.ietf.org/doc/html/rfc6891). If your DNS payloads
consistently exceed EDNS max sizes, your solution may still exhaust the TCP DNS limit.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Event source mapping
Sample applications
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.