---
url: https://docs.aws.amazon.com/step-functions/latest/dg/input-output-itemspath.html
title: input output itemspath.html
word_count: 571
filtered: true
elements_removed: 0
density_score: 0.73
---

ItemsPath (Map, JSONPath only) - AWS Step Functions
ItemsPath (Map, JSONPath only) - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/dg/step-functions-dg.pdf#input-output-itemspath)
###### Managing state and transforming data
This page refers to JSONPath. Step Functions recently added variables and JSONata to manage state and transform data.
Learn about [Passing data with variables](./workflow-variables.html) and [Transforming data with JSONata](./transforming-data.html).
In JSONPath-based states, use the `ItemsPath` field to select an array or object within a JSON input provided to a `Map` state.
By default, the `Map` state sets `ItemsPath` to `$`, which selects the entire input.
*
If the input to the `Map` state is a JSON array, it runs
an iteration for each item in the array, passing that item to the iteration as input
*
If the input to the `Map` state is a JSON object, it runs
an iteration for each key-value pair in the object, passing the pair to the iteration as input
###### Note
You can use `ItemsPath` in the *Distributed Map state*
only
if you use a JSON input passed from a previous state in the
workflow.
The value of `ItemsPath` must be a [Reference Path](./amazon-states-language-paths.html#amazon-states-language-reference-paths), and
that path must evaluate to a JSON array or object. For instance, consider input to a
`Map` state that includes two arrays, like the following example.
```
`{
"ThingsPiratesSay": [
{
"say": "Avast!"
},
{
"say": "Yar!"
},
{
"say": "Walk the Plank!"
}
],
"ThingsGiantsSay": [
{
"say": "Fee!"
},
{
"say": "Fi!"
},
{
"say": "Fo!"
},
{
"say": "Fum!"
}
]
}`
```
In this case, you could specify which array to use for `Map` state
iterations by selecting
it
with `ItemsPath`. The following state machine definition specifies the
`ThingsPiratesSay` array in the input using
`ItemsPath`.It
then
runs
an iteration of the `SayWord` pass state for each item in the
`ThingsPiratesSay` array.
```
`{
"StartAt": "PiratesSay",
"States": {
"PiratesSay": {
"Type": "Map",
"ItemsPath": "$.ThingsPiratesSay",
"ItemProcessor": {
"StartAt": "SayWord",
"States": {
"SayWord": {
"Type": "Pass",
"End": true
}
}
},
"End": true
}
}
} `
```
For nested JSON objects, you can use `ItemsPath` to select a specific object within the input. Consider the following input with nested configuration data:
```
`{
"environment": "production",
"servers": {
"web": {
"server1": {"port": 80, "status": "active"},
"server2": {"port": 8080, "status": "inactive"}
},
"database": {
"primary": {"host": "db1.example.com", "port": 5432},
"replica": {"host": "db2.example.com", "port": 5432}
}
}
}`
```
To iterate over the web servers object, you would set `ItemsPath` to `$.servers.web`:
```
`{
"StartAt": "ProcessWebServers",
"States": {
"ProcessWebServers": {
"Type": "Map",
"ItemsPath": "$.servers.web",
"ItemProcessor": {
"StartAt": "CheckServer",
"States": {
"CheckServer": {
"Type": "Pass",
"End": true
}
}
},
"End": true
}
}
}`
```
When processing input,
the
`Map` state applies
`ItemsPath`
after [InputPath](./input-output-inputpath-params.html#input-output-inputpath). It operates
on the effective input to the
state after
`InputPath`
filters
the input.
For more information on `Map` states, see the following:
* [Map state](./state-map.html)
* [Map state processing modes](./state-map.html#concepts-map-process-modes)
* [Repeat actions with Inline Map](./tutorial-map-inline.html)
* [Inline Map state
input and output processing](./state-map-inline.html#inline-map-state-output)
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
ItemReader
ItemSelector
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.