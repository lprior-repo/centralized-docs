---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/rest-api-routing-rules-recreate-api-mapping.html
title: Recreate an API mapping using routing rules
word_count: 813
filtered: true
elements_removed: 0
density_score: 0.85
---

Recreate an API mapping using routing rules - Amazon API Gateway
Recreate an API mapping using routing rules - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#rest-api-routing-rules-recreate-api-mapping)
# Recreate an API mapping using routing rules
You can recreate an API mapping using routing rules. To recreate an API mapping, make sure to turn on base
path striping. This preserves the behavior of API mappings. For more information, see
[Strip the base path with base path conditions](./rest-api-routing-rules.html#rest-api-routing-rules-condition-path-split).
The following tutorial shows how to recreate the API mapping
`https://
`api.example.com`/orders/v2/items/categories/5` as a routing rule and how to update your
access logs to log the routing rule ID API Gateway uses to send traffic to your API.
AWS Management Console
###### To set the routing mode to ROUTING\_RULE\_THEN\_API\_MAPPING
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. Choose **Custom domain names** from the main navigation
pane.
3. Choose your custom domain name.
4. For **Domain details**, choose
**Edit**.
5. For **Routing mode**, choose
**ROUTING\_RULE\_THEN\_API\_MAPPING**.
6. Choose **Save**
After you set the routing mode, you create the routing rule.
###### To create the routing rule
1. On the **Routing details** tab, choose
**Add routing rule**.
2. Choose **Add new condition** and then choose **Path**.
3. For **Path**, enter `orders/v2/items/categories/5`.
4. For **Strip base path**, choose **Active**.
5. For **Target API**, choose your target API.
6. For **Target stage**, choose your target stage.
7. Choose **Next**.
8. For priority, enter a priority.
Even if you keep your existing API mapping, API Gateway will always use the new routing rule as routing
rules always take priority over API mappings.
9. Choose **Save changes**.
After you create the routing rule, update the access log format for your stage or create a new log to
confirm that API Gateway uses your routing rule to send traffic to your API.
###### To update your access logs
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. Choose your API.
3. In the main navigation pane, choose **Stages**.
4. For **Logs and tracing**, choose **Edit**.
If you don't have a log group, see [Set up CloudWatch logging for REST APIs in API Gateway](./set-up-logging.html).
5. Add `$context.customDomain.routingRuleIdMatched` to your log format.
This log group records the routing rule ID that API Gateway used to send traffic to your API. For more
information, see [I can't tell how API Gateway sent traffic to my APIs](./rest-api-routing-rules-troubleshoot.html#rest-api-routing-rules-logging).
6. Choose **Save**.
After you update your access logs, invoke your custom domain name. The following is an example curl
command to invoke the custom domain name `https://`api.example.com`` with the base path
`orders/v2/items/categories/5`.
```
`curl "https://`api.example.com`/orders/v2/items/categories/5"`
```
After you have successfully invoked your custom domain name, confirm that CloudWatch Logs shows the
`routingRuleIdMatched`. To learn how to use the CloudWatch Logs console to view a log group, see [View API Gateway log
events in the CloudWatch console](./view-cloudwatch-log-events-in-cloudwatch-console.html).
AWS CLI
1. Use the following [update-domain-name](https://docs.aws.amazon.com/cli/latest/reference/apigatewayv2/update-domain-name.html) command to update the domain name
``api.example.com`` to use the routing mode
`ROUTING\_RULE\_THEN\_API\_MAPPING`.
```
`aws apigatewayv2 update-domain-name \\
--domain-name '`api.example.com`' \\
--routing-mode ROUTING\_RULE\_THEN\_API\_MAPPING`
```
2. Use the following
[create-routing-rule](https://docs.aws.amazon.com/cli/latest/reference/apigatewayv2/create-routing-rule.html) command to create a new routing rule to recreate the API mapping
`https://`api.example.com`/orders/v2/items/categories/5`.
```
`aws apigatewayv2 create-routing-rule \\
--domain-name '`api.example.com`' \\
--priority 50 \\
--conditions '[
{
"MatchBasePaths": {
"AnyOf": [
"orders/v2/items/categories/5"
]
}
}
]' \\
--actions '[
{
"InvokeApi": {
"ApiId": "a1b2c3",
"Stage": "prod",
"StripBasePath": true
}
}
]'`
```
3. Use the following [update-stage](https://docs.aws.amazon.com/cli/latest/reference/apigateway/update-stage.html) command to update the access logs format to include the
`$context.customDomain.routingRuleIdMatched` variable. This variable records the routing
rule ID that API Gateway used to send traffic to your API. You use this log to confirm that API Gateway uses your
routing rule to send traffic to your API. For more information, see [I can't tell how API Gateway sent traffic to my APIs](./rest-api-routing-rules-troubleshoot.html#rest-api-routing-rules-logging).
```
`aws apigateway update-stage \\
--rest-api-id `a1bc2c3` \\
--stage-name prod \\
--patch-operations "op=replace,path=/accessLogSettings/format,value='\\$context.path \\$context.customDomain.routingRuleIdMatched \\$context.requestId \\$context.extendedRequestId'"`
```
If you don't have a log group, see [Set up CloudWatch logging for REST APIs in API Gateway](./set-up-logging.html).
4. Use the following example curl command to invoke your custom domain name with the base path
`orders/v2/items/categories/5`.
```
`curl "https://`api.example.com`/orders/v2/items/categories/5`
```
5. Use the following
[filter-log-events](https://docs.aws.amazon.com/cli/latest/reference/logs/filter-log-events.html) command to get the log events from the log group
`access-log-group-orders` that contain routing rule ID `abc123`.
```
`aws logs filter-log-events --log-group-name access-log-group-orders --filter-pattern abc123`
```
This confirms that API Gateway used the routing rule to send traffic to your API.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
How to use routing rules
Troubleshooting issues with routing rules
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.