---
url: https://docs.aws.amazon.com/lambda/latest/dg/snapstart-runtime-hooks.html
title: Implement code before or after Lambda function snapshots
word_count: 307
filtered: true
elements_removed: 0
density_score: 0.88
---

Implement code before or after Lambda function snapshots - AWS Lambda
Implement code before or after Lambda function snapshots - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#snapstart-runtime-hooks)
# Implement code before or after Lambda function snapshots
You can use runtime hooks to implement code before Lambda creates a snapshot or after Lambda resumes a function
from a snapshot. Runtime hooks are useful for a variety of purposes, such as:
* **Cleanup and initialization:** Before a snapshot is created, you can use a runtime hook to perform cleanup or resource release operations. After a snapshot is restored, you can use a runtime hook to re-initialize any resources or state that were not captured in the snapshot.
* **Dynamic configuration:** You can use runtime hooks to dynamically update configuration or other metadata before a snapshot is created or after it is restored. This can be useful if your function needs to adapt to changes in the runtime environment.
* **External integrations:** You can use runtime hooks to integrate with external services or systems, such as sending notifications or updating external state, as part of the checkpointing and restoration process.
* **Performance tuning:** You can use runtime hooks to fine-tune your function's startup sequence, such as by preloading dependencies. For more information, see [Performance tuning](./snapstart-best-practices.html#snapstart-tuning).
The following pages explain how to implement runtime hooks for your preferred runtime.
###### Topics
* [Java](./snapstart-runtime-hooks-java.html)
* [Python](./snapstart-runtime-hooks-python.html)
* [.NET](./snapstart-runtime-hooks-dotnet.html)
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Handling uniqueness
Java
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.