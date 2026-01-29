---
url: https://docs.aws.amazon.com/step-functions/latest/dg/tag-based-policies.html
title: Creating tag-based IAM policies in Step Functions
word_count: 301
filtered: true
elements_removed: 0
density_score: 0.82
---

Creating tag-based IAM policies in Step Functions - AWS Step Functions
Creating tag-based IAM policies in Step Functions - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/dg/step-functions-dg.pdf#tag-based-policies)
# Creating tag-based IAM policies in Step Functions
Step Functions supports policies based on tags. For example, you could restrict access to all
Step Functions resources that include a tag with the key `environment` and the value
`production`.
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Effect": "Deny",
"Action": [
"states:TagResource",
"states:UntagResource",
"states:DeleteActivity",
"states:DeleteStateMachine",
"states:StopExecution"
],
"Resource": "\*",
"Condition": {
"StringEquals": {"aws:ResourceTag/environment": "production"}
}
}
]
}`
`
```
This policy will `Deny` the ability to delete state machines or activities,
stop executions, and add or delete new tags for all resources that have been tagged as
`environment/production`.
For tag-based authorization, state machine execution resources as shown in the following example inherit the tags associated with a state machine.
```
`arn:`partition`:states:`region`:`account-id`:execution:`&lt;StateMachineName&gt;:&lt;ExecutionId&gt;``
```
When you call [DescribeExecution](https://docs.aws.amazon.com/step-functions/latest/apireference/API_DescribeExecution.html) or other APIs in which you specify the execution resource ARN, Step Functions uses tags associated with the state machine to accept or deny the request while performing tag-based authorization. This helps you allow or deny access to state machine executions at the state machine level.
For more information about tagging, see the following:
* [Tagging state machines and activities in Step Functions](./sfn-best-practices.html#concepts-tagging)
* [Controlling Access Using IAM
Tags](https://docs.aws.amazon.com/IAM/latest/UserGuide/access_iam-tags.html)
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
IAM policies for Distributed Maps
Troubleshooting identity and access
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.