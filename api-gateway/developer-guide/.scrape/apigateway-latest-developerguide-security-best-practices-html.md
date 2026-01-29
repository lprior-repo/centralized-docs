---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/security-best-practices.html
title: Security best practices in Amazon API Gateway
word_count: 617
filtered: true
elements_removed: 0
density_score: 0.81
---

Security best practices in Amazon API Gateway - Amazon API Gateway
Security best practices in Amazon API Gateway - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#security-best-practices)
# Security best practices in Amazon API Gateway
API Gateway provides a number of security features to consider as you develop and implement your own security policies. The following best
practices are general guidelines and don’t represent a complete security solution. Because these best practices might not be appropriate or sufficient
for your environment, treat them as helpful considerations rather than prescriptions.
**Implement least privilege access**
Use IAM policies to implement least privilege access for creating, reading,
updating, or deleting API Gateway APIs. To learn more, see [Identity and access management for Amazon API Gateway](./security-iam.html). API Gateway offers several
options to control access to APIs that you create. To learn more, see [Control and manage access to
REST APIs in API Gateway](./apigateway-control-access-to-api.html), [Control and manage access to
WebSocket APIs in API Gateway](./apigateway-websocket-api-control-access.html), and [Control access to HTTP APIs with JWT authorizers in API Gateway](./http-api-jwt-authorizer.html).
**Implement logging**
Use CloudWatch Logs or Amazon Data Firehose to log requests to your APIs. To learn more, see [Monitor REST APIs in API Gateway](./rest-api-monitor.html),
[Configure logging for WebSocket APIs in API Gateway](./websocket-api-logging.html), and
[Configure logging for HTTP APIs in API Gateway](./http-api-logging.html).
**Implement Amazon CloudWatch alarms**
Using CloudWatch alarms, you watch a single metric over a time period that you specify.
If the metric exceeds a given threshold, a notification is sent to an Amazon Simple Notification Service
topic or AWS Auto Scaling policy. CloudWatch alarms do not invoke actions when a metric is in
a particular state. Rather, the state must have changed and been maintained for
a specified number of periods. For more information, see [Monitor REST API execution with Amazon CloudWatch metrics](./monitoring-cloudwatch.html).
**Enable AWS CloudTrail**
CloudTrail provides a record of actions taken by a user, role, or an AWS service in API Gateway. Using the information collected by
CloudTrail, you can determine the request that was made to API Gateway, the IP address from which the request was made, who made the request,
when it was made, and additional details. For more information, see [Logging Amazon API Gateway API calls using
AWS CloudTrail](./cloudtrail.html).
**Enable AWS Config**
AWS Config provides a detailed view of the configuration of AWS resources in your account. You can see how resources are related, get a
history of configuration changes, and see how relationships and configurations change over time. You can use AWS Config to define rules that
evaluate resource configurations for data compliance. AWS Config rules represent the ideal configuration settings for your API Gateway resources. If a
resource violates a rule and is flagged as noncompliant, AWS Config can alert you using an Amazon Simple Notification Service (Amazon SNS) topic.
For details, see [Monitoring API Gateway API configuration with AWS Config](./apigateway-config.html).
**Use AWS Security Hub CSPM**
Monitor your usage of API Gateway as it relates to security best practices by using [AWS Security Hub CSPM](https://docs.aws.amazon.com/securityhub/latest/userguide/what-is-securityhub.html). Security Hub CSPM uses *security controls*
to evaluate resource configurations and *security standards* to help you comply with various compliance
frameworks. For more information about using Security Hub CSPM to evaluate API Gateway resources, see [Amazon API Gateway controls](https://docs.aws.amazon.com/securityhub/latest/userguide/apigateway-controls.html)
in the *AWS Security Hub User Guide*.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Configuration and vulnerability analysis
Tagging
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.