---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/view-cloudwatch-log-events-in-cloudwatch-console.html
title: View API Gateway log
word_count: 421
filtered: true
elements_removed: 0
density_score: 0.70
---

View API Gateway log events in the CloudWatch console - Amazon API Gateway
View API Gateway log events in the CloudWatch console - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#view-cloudwatch-log-events-in-cloudwatch-console)
[Prerequisites](#view-cloudwatch-log-event-prerequisites)[To view logged API requests and responses using the CloudWatch console](#view-cloudwatch-log-event)
# View API Gateway log
events in the CloudWatch console
The following section explains the necessary prerequisites and how to view API Gateway log events in the CloudWatch
console.
## Prerequisites
1. You must have an API created in API Gateway. Follow the instructions in [Develop REST APIs in API Gateway](./rest-api-develop.html).
2. You must have the API deployed and invoked at least once. Follow the instructions in [Deploy REST APIs in API Gateway](./how-to-deploy-api.html) and [Invoke REST APIs in API Gateway](./how-to-call-api.html).
3. You must have CloudWatch Logs enabled for a stage. Follow the instructions in [Set up CloudWatch logging for REST APIs in API Gateway](./set-up-logging.html).
## To view logged API requests and responses using the CloudWatch console
1. Open the CloudWatch console at
[https://console.aws.amazon.com/cloudwatch/](https://console.aws.amazon.com/cloudwatch/).
2. If necessary, change the AWS Region. From the navigation bar, select the
Region where your AWS resources reside. For more information, see [Regions and
Endpoints](http://docs.aws.amazon.com/general/latest/gr/rande.html).
3. In the navigation pane, choose **Logs**, **Log groups**.
4. Under the **Log Groups** table, choose a log group of the
**API-Gateway-Execution-Logs\_{rest-api-id}/{stage-name}**
name.
5. Under the **Log Streams** table, choose a log stream.
You can use the timestamp to help locate the log stream of your interest.
6. Choose **Text** to view raw text or choose
**Row** to view the event row by row.
###### Important
CloudWatch lets you delete log groups or streams. Do not manually delete API Gateway API log
groups or streams; let API Gateway manage these resources. Manually deleting log
groups or streams may cause API requests and responses not to be logged. If that
happens, you can delete the entire log group for the API and redeploy the API.
This is because API Gateway creates log groups or log streams for an API stage
at the time when it is deployed.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
View metrics in the CloudWatch console
Monitoring tools in
AWS for API Gateway
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.