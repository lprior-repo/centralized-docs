---
url: https://docs.aws.amazon.com/lambda/latest/dg/kafka-consumer-group-id.html
title: Customizable consumer group ID in Lambda
word_count: 323
filtered: true
elements_removed: 0
density_score: 0.84
---

Customizable consumer group ID in Lambda - AWS Lambda
Customizable consumer group ID in Lambda - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#kafka-consumer-group-id)
# Customizable consumer group ID in Lambda
When setting up Amazon MSK or self-managed Apache Kafka as an event source, you can specify a
[consumer group](https://developer.confluent.io/learn-more/kafka-on-the-go/consumer-groups/) ID.
This consumer group ID is an existing identifier for the Kafka consumer group that you want your Lambda function to
join. You can use this feature to seamlessly migrate any ongoing Kafka record processing setups from other
consumers to Lambda.
Kafka distributes messages across all consumers in a consumer group. If you specify a consumer group ID that
has other active consumers, Lambda receives only a portion of the messages from the Kafka topic. If you want Lambda
to handle all messages in the topic, turn off any other consumers in that consumer group.
Additionally, if you specify a consumer group ID, and Kafka finds a valid existing consumer group with the same
ID, Lambda ignores the [StartingPosition](./kafka-starting-positions.html) for your event source mapping.
Instead, Lambda begins processing records according to the committed offset of the consumer group. If you specify
a consumer group ID, and Kafka cannot find an existing consumer group, then Lambda configures your event source
with the specified `StartingPosition`.
The consumer group ID that you specify must be unique among all your Kafka event sources. After creating a
Kafka event source mapping with the consumer group ID specified, you cannot update this value.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Polling and stream positions
Event filtering
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.