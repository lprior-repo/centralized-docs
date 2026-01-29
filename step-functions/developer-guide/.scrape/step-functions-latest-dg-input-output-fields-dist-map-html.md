---
url: https://docs.aws.amazon.com/step-functions/latest/dg/input-output-fields-dist-map.html
title: Map state input and output fields in Step Functions
word_count: 329
filtered: true
elements_removed: 0
density_score: 0.88
---

Map state input and output fields in Step Functions - AWS Step Functions
Map state input and output fields in Step Functions - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/dg/step-functions-dg.pdf#input-output-fields-dist-map)
# Map state input and output fields in Step Functions
###### Managing state and transforming data
Learn about [Passing data between states with variables](./workflow-variables.html) and [Transforming data with JSONata](./transforming-data.html).
Map states iterate over a collection of items in a dataset. Examples of data sets include:
* JSON arrays and objects from previous states.
* Individual data files stored in Amazon S3 in formats such as: JSON, JSONL, CSV, Parquet files.
* References to multiple objects, such as: Athena manifests and Amazon S3 inventory files
A map repeats a set of steps for each item in the dataset. You can configure the input that the `Map
state` receives and the output the map generates using a variety of configuration options. Step Functions applies each option in your *Distributed Map state* in the order shown in the following list. Depending on your use case, you may not need to apply all of fields.
1. [ItemReader (Map)](./input-output-itemreader.html) - used to read your data items
2. [ItemsPath (Map, JSONPath only)](./input-output-itemspath.html) or **Items (JSONata)** - optional; used to specify items in your dataset
3. [ItemSelector (Map)](./input-output-itemselector.html) - optional; used to select and modify items in the data set
4. [ItemBatcher (Map)](./input-output-itembatcher.html) - used to process groups of items when processing large sets of items
5. [ResultWriter (Map)](./input-output-resultwriter.html) - provides options for output results from child workflows
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Specify state output with paths
ItemReader
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.