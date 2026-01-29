---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/monitoring_automated_manual.html
title: Monitoring tools in
word_count: 710
filtered: true
elements_removed: 0
density_score: 0.76
---

Monitoring tools in AWS for API Gateway - Amazon API Gateway
Monitoring tools in AWS for API Gateway - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#monitoring_automated_manual)
[Automated monitoring tools](#monitoring_automated_tools)[Manual tools](#monitoring_manual_tools)[Creating alarms](#creating_alarms)
# Monitoring tools in
AWS for API Gateway
AWS provides various tools that you can use to monitor API Gateway. You can
configure some of these tools to do the monitoring for you automatically, while
other tools require manual intervention. We recommend that you automate monitoring
tasks as much as possible.
## Automated monitoring tools in
AWS
You can use the following automated monitoring tools to watch API Gateway
and report when something is wrong:
* **Amazon CloudWatch Alarms** – Watch a single metric over a time period
that you specify, and perform one or more actions based on the value of the metric relative
to a given threshold over a number of time periods. The action is a notification sent to an
Amazon Simple Notification Service (Amazon SNS) topic or Amazon EC2 Auto Scaling policy. CloudWatch alarms do not invoke actions simply because
they are in a particular state; the state must have changed and been maintained for a specified
number of periods. For more information, see [Monitor REST API execution with Amazon CloudWatch metrics](./monitoring-cloudwatch.html).
* **Amazon CloudWatch Logs** – Monitor, store, and access your log files from AWS CloudTrail
or other sources. For more information, see [What is CloudWatch Logs?](https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/WhatIsCloudWatchLogs.html)
in the *Amazon CloudWatch User Guide*.
* **Amazon EventBridge (formerly called CloudWatch Events)** – Match events and route them to one
or more target functions or streams to make changes, capture state
information, and take corrective action. For more information, see
[What Is Amazon EventBridge?](https://docs.aws.amazon.com/eventbridge/latest/userguide/eb-what-is.html)
in
the *EventBridge User Guide*.
* **AWS CloudTrail Log Monitoring** – Share log files
between accounts, monitor CloudTrail log files in real time by sending them to
CloudWatch Logs, write log processing applications in Java, and validate that your log
files have not changed after delivery by CloudTrail. For more information, see
[Working with
CloudTrail Log Files](https://docs.aws.amazon.com/awscloudtrail/latest/userguide/cloudtrail-working-with-log-files.html) in the *AWS CloudTrail User Guide*.
## Manual monitoring tools
Another important part of monitoring API Gateway involves manually
monitoring those items that the CloudWatch alarms don't cover. The API Gateway,
CloudWatch, and other AWS console dashboards provide an at-a-glance view of the state
of your AWS environment. We recommend that you also check the log files on
API execution.
* API Gateway dashboard shows the following statistics for a given API
stage during a specified period of time:
* **API Calls**
* **Cache Hit**, only when API caching is
enabled.
* **Cache Miss**, only when API caching is
enabled.
* **Latency**
* **Integration Latency**
* **4XX Error**
* **5XX Error**
* The CloudWatch home page shows:
* Current alarms and status
* Graphs of alarms and resources
* Service health status
In addition, you can use CloudWatch to do the following:
* Create [customized dashboards](https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/CloudWatch_Dashboards.html) to monitor the services you
care about
* Graph metric data to troubleshoot issues and discover
trends
* Search and browse all your AWS resource metrics
* Create and edit alarms to be notified of problems
## Creating CloudWatch alarms to monitor
API Gateway
You can create a CloudWatch alarm that sends an Amazon SNS message when the alarm changes
state. An alarm watches a single metric over a time period you specify, and
performs one or more actions based on the value of the metric relative to a
given threshold over a number of time periods. The action is a notification sent
to an Amazon SNS topic or Auto Scaling policy. Alarms invoke actions for sustained state
changes only. CloudWatch alarms do not invoke actions simply because they are in a
particular state; the state must have changed and been maintained for a
specified number of periods.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
View log event in the CloudWatch console
CloudWatch logs
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.