---
url: https://docs.aws.amazon.com/lambda/latest/api/API_WaitStartedDetails.html
title: WaitStartedDetails
word_count: 65
filtered: true
elements_removed: 0
density_score: 0.93
---

WaitStartedDetails - AWS Lambda
WaitStartedDetails - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/api/lambda-api.pdf#API_WaitStartedDetails)
[Contents](#API_WaitStartedDetails_Contents)[See Also](#API_WaitStartedDetails_SeeAlso)
# WaitStartedDetails
Details about a wait operation that has started.
## Contents
**
Duration
**
The duration to wait, in seconds.
Type: Integer
Valid Range: Minimum value of 0.
Required: Yes
**
ScheduledEndTimestamp
**
The date and time when the wait operation is scheduled to complete, in [ISO-8601 format](https://www.w3.org/TR/NOTE-datetime) (YYYY-MM-DDThh:mm:ss.sTZD).
Type: Timestamp
Required: Yes