---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/websocket-api-disable-default-endpoint.html
title: Disable the default endpoint for
word_count: 308
filtered: true
elements_removed: 0
density_score: 0.90
---

Disable the default endpoint for WebSocket APIs - Amazon API Gateway
Disable the default endpoint for WebSocket APIs - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#websocket-api-disable-default-endpoint)
# Disable the default endpoint for
WebSocket APIs
By default, clients can invoke your API by using the `execute-api` endpoint that API Gateway generates for
your API. To ensure that clients can access your API only by using a custom domain name, disable the default
`execute-api` endpoint. When you disable the default endpoint, it affects all stages of an API.
The following procedure shows how to disable the default endpoint for a WebSocket API.
AWS Management Console
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. Choose an WebSocket API.
3. Choose **API settings**.
4. On **API details**, choose **Edit**.
5. For **Default endpoint**, select **Inactive**.
6. Choose **Save changes**.
7. On the main navigation pane, choose **Routes**.
8. Choose **Deploy**, and then redeploy your API or create a new stage for
the change to take effect.
AWS CLI
The following [update-api](https://docs.aws.amazon.com/cli/latest/reference/apigatewayv2/update-api.html) command disables
the default endpoint for an WebSocket API:
```
`aws apigatewayv2 update-api \\
--api-id `abcdef123` \\
--disable-execute-api-endpoint`
```
After you disable the default endpoint, you must deploy your API for the change to take effect.
The following AWS CLI command creates a deployment.
```
`aws apigatewayv2 create-deployment \\
--api-id `abcdef123` \\
--stage-name `dev``
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
IP address types for custom domain names for WebSocket APIs
Protect
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.