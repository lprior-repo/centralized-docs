---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-control-access-aws-waf.html
title: Use AWS WAF to protect your REST APIs in API Gateway
word_count: 906
filtered: true
elements_removed: 0
density_score: 0.85
---

Use AWS WAF to protect your REST APIs in API Gateway - Amazon API Gateway
Use AWS WAF to protect your REST APIs in API Gateway - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#apigateway-control-access-aws-waf)
[To associate an AWS WAF
web ACL with an API Gateway API stage using the API Gateway console](#apigateway-control-access-aws-waf-console)[Associate an AWS WAF web ACL with an API Gateway API stage using the
AWS CLI](#apigateway-control-access-aws-waf-awscli)[Associate an AWS WAF web
ACL with an API stage using the AWS WAF REST API](#apigateway-control-access-aws-waf-api)
# Use AWS WAF to protect your REST APIs in API Gateway
AWS WAF is a web application firewall that helps protect web applications and APIs from
attacks. It enables you to configure a set of rules called a web access control list (web
ACL) that allow, block, or count web requests based on customizable web security rules and
conditions that you define. For more information, see [How AWS WAF Works](https://docs.aws.amazon.com/waf/latest/developerguide/how-aws-waf-works.html).
You can use AWS WAF to protect your API Gateway REST API from common web exploits, such as SQL
injection and cross-site scripting (XSS) attacks. These could affect API availability and
performance, compromise security, or consume excessive resources. For example, you can
create rules to allow or block requests from specified IP address ranges, requests from CIDR
blocks, requests that originate from a specific country or region, requests that contain
malicious SQL code, or requests that contain malicious script.
You can also create rules that match a specified string or a regular expression pattern in
HTTP headers, method, query string, URI, and the request body (limited to the first 64 KB).
Additionally, you can create rules to block attacks from specific user agents, bad bots, and
content scrapers. For example, you can use rate-based rules to specify the number of web
requests that are allowed by each client IP in a trailing, continuously updated, 5-minute
period.
###### Important
AWS WAF is your first line of defense against web exploits. When AWS WAF is enabled on an
API, AWS WAF rules are evaluated before other access control features, such as [resource policies](./apigateway-resource-policies.html), [IAM policies](./permissions.html), [Lambda authorizers](./apigateway-use-lambda-authorizer.html), and [Amazon Cognito authorizers](./apigateway-integrate-with-cognito.html). For example,
if AWS WAF blocks access from a CIDR block that a resource policy allows, AWS WAF takes
precedence and the resource policy isn't evaluated.
To enable AWS WAF for your API, you need to do the following:
1. Use the AWS WAF console, AWS SDK, or CLI to create a web ACL that contains
the desired combination of AWS WAF managed rules and your own custom rules. For more
information, see [Getting Started with
AWS WAF](https://docs.aws.amazon.com/waf/latest/developerguide/getting-started.html) and [Web access control lists (web ACLs)](https://docs.aws.amazon.com/waf/latest/developerguide/web-acl.html).
###### Important
API Gateway requires an AWS WAFV2 web ACL for a Regional application or an AWS WAF Classic Regional web ACL.
2. Associate the AWS WAF web ACL with an API stage. You can do this by using
the AWS WAF console, AWS SDK, CLI, or by using the API Gateway console.
## To associate an AWS WAF
web ACL with an API Gateway API stage using the API Gateway console
To use the API Gateway console to associate an AWS WAF web ACL with an existing API Gateway
API stage, use the following steps:
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. Choose an existing API or create a new one.
3. In the main navigation pane, choose **Stages**, and then choose a stage.
4. In the **Stage details** section, choose **Edit**.
5. Under **Web application firewall (AWS WAF)**, select your web ACL.
If you are using AWS WAFV2, select an AWS WAFV2
web ACL for a Regional application. The web ACL and any other AWS WAFV2 resources that it uses must be located
in the same Region as your API.
If you are using AWS WAF Classic Regional, select
a Regional web ACL.
6. Choose **Save changes**.
## Associate an AWS WAF web ACL with an API Gateway API stage using the
AWS CLI
The following [associate-web-acl](https://docs.aws.amazon.com/cli/latest/reference/wafv2/associate-web-acl.html) command
associates an AWS WAFV2 web ACL for a Regional application with an existing API Gateway API stage:
```
`aws wafv2 associate-web-acl \\
--web-acl-arn arn:aws:wafv2:`{region}`:`111122223333`:regional/webacl/test-cli/a1b2c3d4-5678-90ab-cdef-EXAMPLE11111 \\
--resource-arn arn:aws:apigateway:`{region}`::/restapis/4wk1k4onj3/stages/prod`
```
The following [associate-web-acl](https://docs.aws.amazon.com/cli/latest/reference/waf-regional/associate-web-acl.html) command associates an AWS WAF Classic Regional web ACL with an existing API Gateway API
stage:
```
`aws waf-regional associate-web-acl \\
--web-acl-id 'aabc123a-fb4f-4fc6-becb-2b00831cadcf' \\
--resource-arn 'arn:aws:apigateway:`{region}`::/restapis/4wk1k4onj3/stages/prod'`
```
## Associate an AWS WAF web
ACL with an API stage using the AWS WAF REST API
To use the AWS WAFV2 REST API to associate an AWS WAFV2 web ACL for a Regional application with an existing
API Gateway API stage, use the [AssociateWebACL](https://docs.aws.amazon.com/waf/latest/APIReference/API_AssociateWebACL.html) command, as in the following
example:
```
`import boto3
wafv2 = boto3.client('wafv2')
wafv2.associate\_web\_acl(
WebACLArn='arn:aws:wafv2:`{region}`:`111122223333`:regional/webacl/test/abc6aa3b-fc33-4841-b3db-0ef3d3825b25',
ResourceArn='arn:aws:apigateway:`{region}`::/restapis/4wk1k4onj3/stages/prod'
)`
```
To use the AWS WAF REST API to associate an AWS WAF Classic Regional web ACL with an existing
API Gateway API stage, use the [AssociateWebACL](https://docs.aws.amazon.com/waf/latest/APIReference/API_wafRegional_AssociateWebACL.html) command, as in the following
example:
```
`import boto3
waf = boto3.client('waf-regional')
waf.associate\_web\_acl(
WebACLId='aabc123a-fb4f-4fc6-becb-2b00831cadcf',
ResourceArn='arn:aws:apigateway:`{region}`::/restapis/4wk1k4onj3/stages/prod'
)`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Supported certificate authorities for HTTP and HTTP proxy integration
Throttling
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.