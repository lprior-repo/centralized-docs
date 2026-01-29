---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-routing-rules-use.html
title: How to use routing rules
word_count: 802
filtered: true
elements_removed: 0
density_score: 0.71
---

How to use routing rules - Amazon API Gateway
How to use routing rules - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#apigateway-routing-rules-use)
[Create a routing rule](#rest-api-routing-rules-create)[Change the priority of a routing rule](#rest-api-routing-rules-change-priority)
# How to use routing rules
You can create a routing rule using the AWS Management Console, AWS CLI, or any AWS SDK. After you create a rule, you can
change it's priority.
## Create a routing rule
The following procedure shows how to create a routing rule for a custom domain name with a routing mode set
to either `ROUTING\_RULE\_THEN\_API\_MAPPING` or `ROUTING\_RULE\_ONLY`.
AWS Management Console
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. Choose **Custom domain names** from the main navigation
pane.
3. Choose a custom domain name.
4. On the **Routing details** tab, choose
**Add routing rule**.
5. Choose **Add a new condition** to add a new condition.
You can add a header or base path condition. To match all incoming requests to your custom domain
name, don't add a condition.
6. For **Action**, use the dropdown to select your target API and target stage.
7. Choose **Next**.
8. In the priority field, enter a number for your priority.
API Gateway evaluates rules in priority order, from the lowest value to the highest value.
If you're creating a rule without a condition, we recommend that you use a high value
priority.
9. Choose **Create routing rule**.
AWS CLI
The following
[create-routing-rule](https://docs.aws.amazon.com/cli/latest/reference/apigatewayv2/create-routing-rule.html) command creates a routing rule with a priority of 50. In this example, API Gateway
routes any incoming requests that have the headers `Hello:World` and
`x-version:beta` and the base path `PetStoreShopper` to the target API
`a1b2c3`.
```
` aws apigatewayv2 create-routing-rule \\
--domain-name 'api.example.com' \\
--priority 50 \\
--conditions '[
{
"MatchHeaders": {
"AnyOf": [
{
"Header": "Hello",
"ValueGlob": "World"
}
]
}
},
{
"MatchHeaders": {
"AnyOf": [
{
"Header": "x-version",
"ValueGlob": "beta"
}
]
}
},
{
"MatchBasePaths": {
"AnyOf": [
"PetStoreShopper"
]
}
}
]'\\
--actions '[
{
"InvokeApi": {
"ApiId": "a1b2c3",
"Stage": "prod"
}
}
]'`
```
The output will look like the following.
```
`{
"Actions": [
{
"InvokeApi": {
"ApiId": "a1b2c3",
"Stage": "prod",
"StripBasePath": false
}
}
],
"Conditions": [
{
"MatchHeaders": {
"AnyOf": [
{
"Header": "Hello",
"ValueGlob": "World"
}
]
}
},
{
"MatchHeaders": {
"AnyOf": [
{
"Header": "x-version",
"ValueGlob": "beta"
}
]
}
},
{
"MatchBasePaths": {
"AnyOf": [
"PetStoreShopper"
]
}
}
],
"Priority": 50,
"RoutingRuleArn": "arn:aws:apigateway:us-west-2:111122223333:/domainnames/api.example.com/routingrules/abc123",
"RoutingRuleId": "abc123"
}`
```
## Change the priority of a routing rule
You can change the priority of a routing rule. This takes effect immediately and might impact how API
consumers invoke your custom domain names. We recommend that when you set the priorities of your routing rules, you
leave gaps between rules.
For example, consider two routing rules, rule `abc123` with a priority of 50 and rule `zzz000`
with a priority of 150. To change the priority of the rules so that API Gateway evaluates rule
`zzz000` first, you can change the priority of rule `zzz000` to 30.
AWS Management Console
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. Choose **Custom domain names** from the main navigation
pane.
3. Choose a custom domain name.
4. On the **Routing details** tab, choose your routing rule, and then choose **Edit**.
5. Choose **Next**.
6. For priority, enter the new priority.
7. Choose **Save changes**.
AWS CLI
The following [put-routing-rule](https://docs.aws.amazon.com/cli/latest/reference/apigatewayv2/put-routing-rule.html) command changes the priority of a routing rule `abc123`.
```
` aws apigatewayv2 put-routing-rule \\
--domain-name 'api.example.com' \\
--priority 30 \\
--routing-rule-id abc123 \\
--conditions '[
{
"MatchHeaders": {
"AnyOf": [
{
"Header": "Hello",
"ValueGlob": "World"
}
]
}
},
{
"MatchHeaders": {
"AnyOf": [
{
"Header": "x-version",
"ValueGlob": "beta"
}
]
}
},
{
"MatchBasePaths": {
"AnyOf": [
"PetStoreShopper"
]
}
}
]'\\
--actions '[
{
"InvokeApi": {
"ApiId": "a1b2c3",
"Stage": "prod"
}
}
]'`
```
The output will look like the following:
```
`{
"Actions": [
{
"InvokeApi": {
"ApiId": "a1b2c3",
"Stage": "prod",
"StripBasePath": false
}
}
],
"Conditions": [
{
"MatchHeaders": {
"AnyOf": [
{
"Header": "Hello",
"ValueGlob": "World"
}
]
}
},
{
"MatchHeaders": {
"AnyOf": [
{
"Header": "x-version",
"ValueGlob": "beta"
}
]
}
},
{
"MatchBasePaths": {
"AnyOf": [
"PetStoreShopper"
]
}
}
],
"Priority": 38,
"RoutingRuleArn": "arn:aws:apigateway:us-west-2:111122223333:/domainnames/api.example.com/routingrules/abc123",
"RoutingRuleId": "abc123"
}`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Examples of how API Gateway evaluates routing rules
Recreate an API mapping using routing rules
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.