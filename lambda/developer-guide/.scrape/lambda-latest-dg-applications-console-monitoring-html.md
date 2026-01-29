---
url: https://docs.aws.amazon.com/lambda/latest/dg/applications-console-monitoring.html
title: Monitoring Lambda applications
word_count: 378
filtered: true
elements_removed: 0
density_score: 0.68
---

Monitoring Lambda applications - AWS Lambda
Monitoring Lambda applications - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#applications-console-monitoring)
# Monitoring Lambda applications
The **Applications** section of the Lambda console includes a **Monitoring** tab where you can review an Amazon CloudWatch dashboard with aggregate metrics for the resources in your application.
###### To monitor a Lambda application
1. Open the Lambda console [Applications page](https://console.aws.amazon.com/lambda/home#/applications).
2. Choose **Monitoring**.
3. To see more details about the metrics in any graph, choose **View in metrics** from the drop-down menu.
![A monitoring widget.](https://docs.aws.amazon.com/images/lambda/latest/dg/images/applications-monitoring-widget.png)
The graph appears in a new tab, with the relevant metrics listed below the graph. You can customize your view of this graph, changing the metrics and resources shown, the statistic, the period, and other factors to get a better understanding of the current situation.
By default, the Lambda console shows a basic dashboard. You can customize this page by adding one or more Amazon CloudWatch dashboards to your application
template with the [AWS::CloudWatch::Dashboard](https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cw-dashboard.html)
resource type. When your template includes one or more dashboards, the page shows your
dashboards instead of the default dashboard. You can switch between dashboards with the drop-down menu on the top
right of the page. The following example creates a dashboard with a single widget that graphs the number of
invocations of a function named `my-function`.
###### Example function dashboard template
```
`Resources:
MyDashboard:
Type: AWS::CloudWatch::Dashboard
Properties:
DashboardName: my-dashboard
DashboardBody: |
{
"widgets": [
{
"type": "metric",
"width": 12,
"height": 6,
"properties": {
"metrics": [
[
"AWS/Lambda",
"Invocations",
"FunctionName",
"my-function",
{
"stat": "Sum",
"label": "MyFunction"
}
],
[
{
"expression": "SUM(METRICS())",
"label": "Total Invocations"
}
]
],
"region": "us-east-1",
"title": "Invocations",
"view": "timeSeries",
"stacked": false
}
}
]
}`
```
For more information about authoring CloudWatch dashboards and widgets, see [Dashboard body structure and syntax](https://docs.aws.amazon.com/AmazonCloudWatch/latest/APIReference/CloudWatch-Dashboard-Body-Structure.html) in the
*Amazon CloudWatch API Reference*.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Function insights
Application Signals
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.