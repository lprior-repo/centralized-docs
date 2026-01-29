---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/how-to-api-dashboard.html
title: View CloudWatch metrics with the API dashboard in API Gateway
word_count: 323
filtered: true
elements_removed: 0
density_score: 0.93
---

View CloudWatch metrics with the API dashboard in API Gateway - Amazon API Gateway
View CloudWatch metrics with the API dashboard in API Gateway - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#how-to-api-dashboard)
[Prerequisites](#how-to-api-dashboard-prerequisites)[Examine API activities in the dashboard](#how-to-api-dashboard-console)
# View CloudWatch metrics with the API dashboard in API Gateway
You can use the API dashboard in the API Gateway Console to display the CloudWatch metrics of your deployed API in API Gateway. These are shown as a summary of API activity over time.
###### Topics
* [Prerequisites](#how-to-api-dashboard-prerequisites)
* [Examine API activities in the dashboard](#how-to-api-dashboard-console)
## Prerequisites
1. You must have an API created in API Gateway. Follow the instructions in [Develop REST APIs in API Gateway](./rest-api-develop.html).
2. You must have the API deployed at least once. Follow the instructions in [Deploy REST APIs in API Gateway](./how-to-deploy-api.html).
## Examine API activities in the dashboard
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. Choose an API.
3. In the main navigation pane, choose **Dashboard**.
4. For
**Stage**, choose the desired stage.
5. Choose **Date range** to specify a range of dates.
6. Refresh, if needed, and view individual metrics displayed in separate graphs titled **API
calls**, **Latency**, **Integration latency**,
**Latency**, **4xx error** and **5xx error**.
###### Tip
To examine method-level CloudWatch metrics, make sure that you have enabled CloudWatch Logs on a method level. For
more information about how to set up method-level logging, see [Override stage-level settings](./set-up-stages.html#how-to-method-override).
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Amazon API Gateway dimensions and
metrics
View metrics in the CloudWatch console
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.