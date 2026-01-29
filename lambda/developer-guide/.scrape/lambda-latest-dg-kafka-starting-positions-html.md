---
url: https://docs.aws.amazon.com/lambda/latest/dg/kafka-starting-positions.html
title: Apache Kafka polling and stream starting positions in Lambda
word_count: 303
filtered: true
elements_removed: 0
density_score: 0.84
---

Apache Kafka polling and stream starting positions in Lambda - AWS Lambda
Apache Kafka polling and stream starting positions in Lambda - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#kafka-starting-positions)
# Apache Kafka polling and stream starting positions in Lambda
The [
StartingPosition parameter](https://docs.aws.amazon.com/lambda/latest/api/API_CreateEventSourceMapping.html#lambda-CreateEventSourceMapping-request-StartingPosition) tells Lambda when to start reading messages from your Amazon MSK or self-managed Apache Kafka stream. There are
three options to choose from:
* **Latest** – Lambda starts reading just after the most recent
record in the Kafka topic.
* **Trim horizon** – Lambda starts reading from the last untrimmed
record in the Kafka topic. This is also the oldest record in the topic.
* **At timestamp** – Lambda starts reading from a position defined
by a timestamp, in Unix time seconds. Use the [
StartingPositionTimestamp parameter](https://docs.aws.amazon.com/lambda/latest/api/API_CreateEventSourceMapping.html#lambda-CreateEventSourceMapping-request-StartingPositionTimestamp) to specify the timestamp.
Stream polling during an event source mapping create or update is eventually consistent:
* During event source mapping creation, it may take several minutes to start polling events
from the stream.
* During event source mapping updates, it may take up to 90 seconds to stop and restart polling
events from the stream.
This behavior means that if you specify `LATEST` as the starting position for the stream, the event
source mapping could miss events during a create or update. To ensure that no events are missed, specify either
`TRIM\_HORIZON` or `AT\_TIMESTAMP`.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Event poller scaling
Consumer group ID
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.