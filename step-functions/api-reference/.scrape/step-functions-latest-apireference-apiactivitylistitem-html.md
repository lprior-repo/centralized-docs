---
url: https://docs.aws.amazon.com/step-functions/latest/apireference/API_ActivityListItem.html
title: API ActivityListItem.html
word_count: 145
filtered: true
elements_removed: 0
density_score: 0.93
---

ActivityListItem - AWS Step Functions
ActivityListItem - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/apireference/step-functions-api.pdf#API_ActivityListItem)
[Contents](#API_ActivityListItem_Contents)[See Also](#API_ActivityListItem_SeeAlso)
## Contents
**
activityArn
**
The Amazon Resource Name (ARN) that identifies the activity.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 256.
Required: Yes
**
creationDate
**
The date the activity is created.
Type: Timestamp
Required: Yes
**
name
**
The name of the activity.
A name must *not* contain:
* white space
* brackets `&lt; &gt; { } [ ]`
* wildcard characters `? \*`
* special characters `" # % \\ ^ | \~ ` $ &amp;&amp; , ; : /`
* control characters (`U+0000-001F`, `U+007F-009F`, `U+FFFE-FFFF`)
* surrogates (`U+D800-DFFF`)
* invalid characters (` U+10FFFF`)
To enable logging with CloudWatch Logs, the name should only contain 0-9, A-Z, a-z, - and \_.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 80.
Required: Yes