---
url: https://docs.aws.amazon.com/lambda/latest/dg/services-mq-errors.html
title: Troubleshoot Amazon MQ event source mapping errors
word_count: 288
filtered: true
elements_removed: 0
density_score: 0.89
---

Troubleshoot Amazon MQ event source mapping errors - AWS Lambda
Troubleshoot Amazon MQ event source mapping errors - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#services-mq-errors)
# Troubleshoot Amazon MQ event source mapping errors
When a Lambda function encounters an unrecoverable error, your Amazon MQ consumer stops processing records. Any
other consumers can continue processing, provided that they do not encounter the same error. To determine the
potential cause of a stopped consumer, check the `StateTransitionReason` field in the return details of
your `EventSourceMapping` for one of the following codes:
**`ESM\_CONFIG\_NOT\_VALID`**
The event source mapping configuration is not valid.
**`EVENT\_SOURCE\_AUTHN\_ERROR`**
Lambda failed to authenticate the event source.
**`EVENT\_SOURCE\_AUTHZ\_ERROR`**
Lambda does not have the required permissions to access the event source.
**`FUNCTION\_CONFIG\_NOT\_VALID`**
The function's configuration is not valid.
Records also go unprocessed if Lambda drops
them due to their size. The size limit for Lambda records is 6 MB. To
redeliver messages upon function error, you can use a dead-letter queue (DLQ). For more information, see [Message Redelivery and
DLQ Handling](https://activemq.apache.org/message-redelivery-and-dlq-handling) on the Apache ActiveMQ website and [Reliability Guide](https://www.rabbitmq.com/reliability.html) on the RabbitMQ
website.
###### Note
Lambda does not support custom redelivery policies. Instead, Lambda uses a policy with the default values from the [Redelivery Policy](https://activemq.apache.org/redelivery-policy) page on the Apache ActiveMQ website, with `maximumRedeliveries` set to 6.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Event filtering
RDS
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.