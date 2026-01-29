---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/cloudtrail.html
title: Logging Amazon API Gateway API calls using
word_count: 945
filtered: true
elements_removed: 0
density_score: 0.84
---

Logging Amazon API Gateway API calls using AWS CloudTrail - Amazon API Gateway
Logging Amazon API Gateway API calls using AWS CloudTrail - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#cloudtrail)
[API Gateway management events in CloudTrail](#cloudtrail-management-events)[API Gateway event example](#cloudtrail-event-examples)
# Logging Amazon API Gateway API calls using
AWS CloudTrail
Amazon API Gateway is integrated with [AWS CloudTrail](https://docs.aws.amazon.com/awscloudtrail/latest/userguide/cloudtrail-user-guide.html), a service that provides a record of actions taken by a user, role, or an
AWS service. CloudTrail captures all REST API calls for API Gateway
service as events. The calls captured include calls from the API Gateway console
and code calls to the API Gateway service APIs. Using the information collected by CloudTrail, you can
determine the request that was made to API Gateway, the IP address from which the request was
made, when it was made, and additional details.
###### Note
[TestInvokeAuthorizer](https://docs.aws.amazon.com/cli/latest/reference/apigateway/test-invoke-authorizer.html) and [TestInvokeMethod](https://docs.aws.amazon.com/cli/latest/reference/apigateway/test-invoke-method.html) are not logged in CloudTrail.
Every event or log entry contains information about who generated the request. The identity
information helps you determine the following:
* Whether the request was made with root user or user credentials.
* Whether the request was made on behalf of an IAM Identity Center user.
* Whether the request was made with temporary security credentials for a role or federated
user.
* Whether the request was made by another AWS service.
CloudTrail is active in your AWS account when you create the account and you automatically have
access to the CloudTrail **Event history**. The CloudTrail **Event
history** provides a viewable, searchable, downloadable, and immutable record of the
past 90 days of recorded management events in an AWS Region. For more information, see [Working
with CloudTrail Event history](https://docs.aws.amazon.com/awscloudtrail/latest/userguide/view-cloudtrail-events.html) in the *AWS CloudTrail User Guide*. There are no CloudTrail
charges for viewing the **Event history**.
For an ongoing record of events in your AWS account past 90 days, create a trail or a
[CloudTrail
Lake](https://docs.aws.amazon.com/awscloudtrail/latest/userguide/cloudtrail-lake.html) event data store.
**CloudTrail trails**
A *trail* enables CloudTrail to deliver log files to an Amazon S3 bucket. All trails created using the AWS Management Console are multi-Region. You can create a single-Region or a multi-Region trail by using the AWS CLI. Creating a multi-Region trail is recommended because you capture activity in all AWS Regions in your account. If you create a single-Region trail, you can view only the events logged in the trail's AWS Region. For more information about trails, see [Creating a trail for your AWS account](https://docs.aws.amazon.com/awscloudtrail/latest/userguide/cloudtrail-create-and-update-a-trail.html) and [Creating a trail for an organization](https://docs.aws.amazon.com/awscloudtrail/latest/userguide/creating-trail-organization.html) in the *AWS CloudTrail User Guide*.
You can deliver one copy of your ongoing management events to your Amazon S3 bucket at no charge from CloudTrail by creating a trail, however, there are Amazon S3 storage charges. For more information about CloudTrail pricing, see [AWS CloudTrail Pricing](https://aws.amazon.com/cloudtrail/pricing/). For information about Amazon S3 pricing, see [Amazon S3 Pricing](https://aws.amazon.com/s3/pricing/).
**CloudTrail Lake event data stores**
*CloudTrail Lake* lets you run SQL-based queries on your events. CloudTrail Lake converts existing events in row-based JSON format to [ Apache ORC](https://orc.apache.org/) format. ORC is a columnar storage format that is optimized for fast retrieval of data. Events are aggregated into *event data stores*, which are immutable collections of events based on criteria that you select by applying [advanced event selectors](https://docs.aws.amazon.com/awscloudtrail/latest/userguide/cloudtrail-lake-concepts.html#adv-event-selectors). The selectors that you apply to an event data store control which events persist and are available for you to query. For more information about CloudTrail Lake, see [Working with AWS CloudTrail Lake](https://docs.aws.amazon.com/awscloudtrail/latest/userguide/cloudtrail-lake.html) in the *AWS CloudTrail User Guide*.
CloudTrail Lake event data stores and queries incur costs. When you create an event data store, you choose the [pricing option](https://docs.aws.amazon.com/awscloudtrail/latest/userguide/cloudtrail-lake-manage-costs.html#cloudtrail-lake-manage-costs-pricing-option) you want to use for the event data store. The pricing option determines the cost for ingesting and storing events, and the default and maximum retention period for the event data store. For more information about CloudTrail pricing, see [AWS CloudTrail Pricing](https://aws.amazon.com/cloudtrail/pricing/).
## API Gateway management events in CloudTrail
[Management events](https://docs.aws.amazon.com/awscloudtrail/latest/userguide/logging-management-events-with-cloudtrail.html#logging-management-events) provide information about management operations that are performed on resources in your AWS account. These are also known as control plane operations. By default, CloudTrail logs management events.
Amazon API Gateway logs all API Gateway actions as management events, except for [TestInvokeAuthorizer](https://docs.aws.amazon.com/cli/latest/reference/apigateway/test-invoke-authorizer.html) and [TestInvokeMethod](https://docs.aws.amazon.com/cli/latest/reference/apigateway/test-invoke-method.html). For a list
of the Amazon API Gateway actions that API Gateway logs to CloudTrail, see the
[Amazon API Gateway API Reference](https://docs.aws.amazon.com/apigateway/latest/api/API_Operations.html).
## API Gateway event example
An event represents a single request from any source and includes information about the requested API operation, the date and time of the operation, request parameters, and so on. CloudTrail log files aren't an ordered stack trace of the public API calls, so events don't appear in any specific order.
The following example shows a CloudTrail event that demonstrates the API Gateway
`GetResource` action:
```
`{
Records: [
{
eventVersion: "1.03",
userIdentity: {
type: "Root",
principalId: "AKIAI44QH8DHBEXAMPLE",
arn: "arn:aws:iam::123456789012:root",
accountId: "123456789012",
accessKeyId: "AKIAIOSFODNN7EXAMPLE",
sessionContext: {
attributes: {
mfaAuthenticated: "false",
creationDate: "2015-06-16T23:37:58Z"
}
}
},
eventTime: "2015-06-17T00:47:28Z",
eventSource: "apigateway.amazonaws.com",
eventName: "GetResource",
awsRegion: "us-east-1",
sourceIPAddress: "203.0.113.11",
userAgent: "example-user-agent-string",
requestParameters: {
restApiId: "3rbEXAMPLE",
resourceId: "5tfEXAMPLE",
template: false
},
responseElements: null,
requestID: "6d9c4bfc-148a-11e5-81b6-7577cEXAMPLE",
eventID: "4d293154-a15b-4c33-9e0a-ff5eeEXAMPLE",
readOnly: true,
eventType: "AwsApiCall",
recipientAccountId: "123456789012"
},
... additional entries ...
]
}`
```
For information about CloudTrail record contents, see [CloudTrail
record contents](https://docs.aws.amazon.com/awscloudtrail/latest/userguide/cloudtrail-event-reference-record-contents.html) in the *AWS CloudTrail User Guide*.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Logging and monitoring
Working with AWS Config
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.