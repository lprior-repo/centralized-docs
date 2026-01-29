---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/rest-api-disable-default-endpoint.html
title: Disable the default endpoint for REST APIs
word_count: 353
filtered: true
elements_removed: 0
density_score: 0.90
---

Disable the default endpoint for REST APIs - Amazon API Gateway
Disable the default endpoint for REST APIs - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#rest-api-disable-default-endpoint)
# Disable the default endpoint for REST APIs
By default, clients can invoke your API by using the `execute-api` endpoint that API Gateway generates for
your API. To ensure that clients can access your API only by using a custom domain name, disable the default
`execute-api` endpoint. Clients can still connect to your default endpoint, but they will receive a
`403 Forbidden` status code. Disabling the default endpoint affects all stages of the API. This setting
takes affect when you update any setting on any stage, such as updating the deployment on the stage.
The following procedure shows how to disable the default endpoint for a REST API.
AWS Management Console
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. Choose a REST API.
3. On the main navigation pane, choose **API settings**.
4. Choose an API.
5. On **API details**, choose **Edit**.
6. For **Default endpoint**, select **Inactive**.
7. Choose **Save changes**.
8. On the main navigation pane, choose **Resources**.
9. Choose **Deploy API**.
10. Redeploy your API to a stage or update any setting on a stage for the update to take effect.
AWS CLI
The following [update-rest-api](https://docs.aws.amazon.com/cli/latest/reference/apigateway/update-rest-api.html) command
disables the default endpoint:
```
`aws apigateway update-rest-api \\
--rest-api-id `abcdef123` \\
--patch-operations op=replace,path=/disableExecuteApiEndpoint,value='True'`
```
After you disable the default endpoint, you must deploy your API for the change to take effect.
The following [create-deployment](https://docs.aws.amazon.com/cli/latest/reference/apigateway/create-deployment.html)
command creates a deployment and associates it with a stage:
```
`aws apigateway create-deployment \\
--rest-api-id `abcdef123` \\
--stage-name `dev``
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Choose a security policy
DNS failover
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.