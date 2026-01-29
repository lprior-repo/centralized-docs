---
url: https://docs.aws.amazon.com/lambda/latest/dg/snapstart-runtime-hooks-python.html
title: Lambda SnapStart runtime hooks for Python
word_count: 500
filtered: true
elements_removed: 0
density_score: 0.89
---

Lambda SnapStart runtime hooks for Python - AWS Lambda
Lambda SnapStart runtime hooks for Python - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#snapstart-runtime-hooks-python)
[Registration and execution](#runtime-hooks-registration-python)[Example](#runtime-hooks-python-code-sample)
# Lambda SnapStart runtime hooks for Python
You can use runtime hooks to implement code before Lambda creates a snapshot or after Lambda resumes a function
from a snapshot. Python runtime hooks are available as part of the open-source [Snapshot Restore for Python library](https://pypi.org/project/snapshot-restore-py/), which is included in Python managed runtimes. This library provides two decorators that you can use to define your runtime hooks:
* `@register\_before\_snapshot`: For functions you want to run before Lambda creates a snapshot.
* `@register\_after\_restore`: For functions you want to run when Lambda resumes a function from a snapshot.
Alternatively, you can use the following methods to register callables for runtime hooks:
* `register\_before\_snapshot(func, \*args, \*\*kwargs)`
* `register\_after\_restore(func, \*args, \*\*kwargs)`
## Runtime hook registration and execution
The order that Lambda executes your runtime hooks is determined by the order of registration:
* Before snapshot: Executed in the reverse order of registration
* After snapshot: Executed in the order of registration
The order of runtime hook registration depends on how you define the hooks. When using decorators (`@register\_before\_snapshot` and `@register\_after\_restore`), the registration order follows the order of import, definition, or execution in your code. If you need more control over the registration order, use the `register\_before\_snapshot()` and `register\_after\_restore()` methods instead of decorators.
Make sure that all registered hooks are properly imported and included in your function's code. If you register runtime hooks in a separate file or module, you must ensure that the module is imported, either directly or as part of a larger package, in your function's handler file. If the file or module is not imported in the function handler, Lambda ignores the runtime hooks.
###### Note
When Lambda creates a snapshot, your initialization code can run for up to 15 minutes. The time limit is 130 seconds or the [configured function timeout](./configuration-timeout.html) (maximum 900 seconds), whichever is higher. Your `@register\_before\_snapshot` runtime hooks count towards the initialization code time limit. When Lambda restores a snapshot, the runtime must load and `@register\_after\_restore` runtime hooks must complete within the timeout limit (10 seconds). Otherwise, you'll get a SnapStartTimeoutException.
## Example
The following example handler shows how to run code before checkpointing (`@register\_before\_snapshot`) and
after restoring (`@register\_after\_restore`).
```
`from snapshot\_restore\_py import register\_before\_snapshot, register\_after\_restore
def lambda\_handler(event, context):
# Logic to be executed before taking snapshots
@register\_after\_restore
def after\_restore():
# Logic to be executed after restore`
```
For more examples, see [Snapshot Restore for Python](https://github.com/aws/snapshot-restore-py/tree/main/examples) in the AWS GitHub repository.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Java
.NET
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.