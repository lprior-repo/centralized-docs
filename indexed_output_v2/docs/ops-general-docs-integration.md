---
id: ops/general/docs-integration
title: Docs Integration
category: ops
tags: ["integrations", "ops"]
---

# Integrations | CUE

> **Context**: **Source:** https://cuelang.org/docs/integration/


**Source:** https://cuelang.org/docs/integration/

Skip to content

Homepage of CUE [/]
 * Documentation [/docs/]
 * Play [/play/]
 * Community [/community/]

 * 
   GitHub [https://github.com/cue-lang/cue]
 * 
   Slack [/s/slack]
 * 
   Discord [/s/discord]
 * 
   X (Twitter) [https://twitter.com/cue_lang]
 * 
   Bluesky [https://bsky.app/profile/cuelang.org]
 * 
   YouTube [https://www.youtube.com/@cuelang/videos]

Install
[/docs/introduction/installation/]

Search [/search]

What are you looking for?

Menu

 1. INTEGRATIONS

CUE has first-class support for a growing range of formats and ecosystems:


JSON

CUE reads, writes, and validates JSON data, regardless of its source:
discrete JSON files, I/O streams, or JSON contained in data fields inside
other formats. The cue command emits JSON by default.
Learn more

Learn more [/docs/concept/how-cue-works-with-json/]


YAML

CUE’s support for YAML mirrors its first class JSON support: it reads, writes
and validates YAML data anywhere it can be found: discrete YAML files, I/O
streams, or YAML contained in data fields inside other formats.
Read more

Read more [/docs/concept/how-cue-works-with-yaml/]


GO

The cue command converts Go types to CUE, enabling their first class use as
schema and data constraints. CUE’s extensive Go API allows code to have
fine grained control over CUE’s capabilities and operations, including
export to any encoding supported by CUE.
Learn more

Learn more [/docs/concept/how-cue-works-with-go/]


TOML

CUE supports reading and writing TOML data wherever it’s found: discrete
TOML data files, I/O streams, or encoded in string fields inside other formats.
Find out more

Find out more [/docs/concept/how-cue-works-with-toml/]


OPENAPI

CUE reads and writes OpenAPI data schemas through its Go API and the cue
command, enabling schemas to be used to constrain and validate data directly,
and to be expressed in other formats - including CUE.
Read more

Read more [/docs/concept/how-cue-works-with-openapi/]


PROTOCOL BUFFERS

CUE’s Go API and the cue command read Protobuf definitions, enabling them
to be used to constrain and validate data directly and to be expressed in other
formats. CUE constraints can be extracted from Protobuf options, allowing
richer data validation than Protobuf’s type-based defaults.
Learn more

Learn more [/docs/concept/how-cue-works-with-protocol-buffers/]


JSON SCHEMA

CUE understands JSON Schema constraints through its Go API and the cue
command, enabling schemas to be used to constrain and validate data directly,
and to be expressed in other formats - including CUE.
Read more

Read more [/docs/concept/how-cue-works-with-json-schema/]


JAVA

Technology preview

Support for using CUE in Java is available through an experimental Java library.

Follow this introductory tutorial to get started with CUE in Java.

Learn more

Learn more [/docs/tutorial/get-started-cue-java/]

TECHNOLOGIES

CUE is independent of the technologies it can be used alongside.
Some examples of its use with specific tools, systems, and providers are collected in
CUE By Example [https://github.com/cue-labs/cue-by-example/]:


CONTROLLING KUBERNETES

A worked example of converting a set of Kubernetes configuration files
for a collection of microservices into smaller, validated CUE configurations by
automatically removing boilerplate;
automating commands that don’t know CUE yet (such as kubectl);
and extracting schema definitions from Kubernetes source code.
Read on CUE By Example

Read on CUE By Example [https://github.com/cue-labs/cue-by-example/tree/main/003_kubernetes_tutorial/README.md]


DRIVING GITHUB ACTIONS WORKFLOWS

A guide explaining how to convert GitHub Actions workflow files from YAML to
CUE, check those workflows are valid, and then use CUE’s tooling layer to
regenerate YAML - allowing safer and more predictable changes.
Read on CUE By Example

Read on CUE By Example [https://github.com/cue-labs/cue-by-example/blob/main/001_github_actions_importing_workflows/README.md]


WRITING TERRAFORM PLAN POLICIES

A pair of guides showing how to validate the JSON output from terraform plan using CUE as a policy language.
Read on CUE By Example

Read on CUE By Example [https://github.com/cue-labs/cue-by-example/blob/main/002_terraform_plan/README.md]


MANAGING MYTHIC BEASTS DNS

A guide that demonstrates how to use CUE to keep DNS data in a compact format,
using CUE templating to DRY out configurations, and use CUE’s constraints to
enforce policies on the data to guard against mistakes.
Read on CUE By Example

Read on CUE By Example [https://github.com/cue-labs/cue-by-example/blob/main/004_mythic_beasts_dns/README.md]


DRIVING GITLAB CI/CD PIPELINES

A guide illustrating how to convert a GitLab CI/CD pipeline file from YAML to
CUE, check its contents are valid, and then use CUE’s tooling layer to
regenerate YAML - allowing safer and more predictable changes.
Read on CUE By Example

Read on CUE By Example [https://github.com/cue-labs/cue-by-example/blob/main/005_gitlab_ci/README.md]


DRIVING BUILDKITE PIPELINES

A guide demonstrating how to convert static Buildkite pipelines files from YAML to
CUE, check the pipelines are valid, and then use CUE’s tooling layer to
regenerate YAML - allowing safer and more predictable changes by switching to CUE as a source of truth.
Read on CUE By Example

Read on CUE By Example [https://github.com/cue-labs/cue-by-example/blob/main/006_buildkite_importing_pipelines/README.md]


SUPERCHARGING BUILDKITE PIPELINES

A guide that builds on an official Buildkite blog post, showing how to use CUE
to define and validate CI pipelines as they’re initiated and as they’re
executing, so that their steps can vary dynamically, based on the pipeline’s
execution context.
Read on CUE By Example

Read on CUE By Example [https://github.com/cue-labs/cue-by-example/blob/main/007_buildkite_dynamic_pipelines/README.md]

Last modified May 23, 2025 [https://github.com/cue-lang/cuelang.org/commit/c276a64d08b83d12621aabfb8fc349e49e47e693]


Open share options
 * 
   
   Copy link
   Copied!

 * Share on X (Twitter)
   
   [https://twitter.com/intent/tweet?url=https://cuelang.org/docs/integration/&text=CUE%20has%20first-class%20support%20for%20a%20growing%20range%20of%20formats%20and%20ecosystems:%0aJSON%20CUE%20reads,%20writes,%20and%20validates%20JSON%20data,%20regardless%20of%20its%20source:%20discrete%20JSON%20files,%20I/O%20streams,%20or%20JSON%20contained%20in%20data%20fields%20inside%20other%20formats.%20The%20cue%20command%20emits%20JSON%20by%20default.%20Learn%20more%20Learn%20more%20YAML%20CUE&rsquo;s%20support%20for%20YAML%20mirrors%20its%20first%20class%20JSON%20support:%20it%20reads,%20writes%20and%20validates%20YAML%20data%20anywhere%20it%20can%20be%20found:%20discrete%20YAML%20files,%20I/O%20streams,%20or%20YAML%20contained%20in%20data%20fields%20inside%20other%20formats.%20Read%20more%20Read%20more%20Go%20The%20cue%20command%20converts%20Go%20types%20to%20CUE,%20enabling%20their%20first%20class%20use%20as%20schema%20and%20data%20constraints.%20CUE&rsquo;s%20extensive%20Go%20API%20allows%20code%20to%20have%20fine%20grained%20control%20over%20CUE&rsquo;s%20capabilities%20and%20operations,%20including%20export%20to%20any%20encoding%20supported%20by%20CUE.%20Learn%20more%20Learn%20more%20TOML%20CUE%20supports%20reading%20and%20writing%20TOML%20data%20wherever%20it&rsquo;s%20found:%20discrete%20TOML%20data%20files,%20I/O%20streams,%20or%20encoded%20in%20string%20fields%20inside%20other%20formats.%20Find%20out%20more%20Find%20out%20more%20OpenAPI%20CUE%20reads%20and%20writes%20OpenAPI%20data%20schemas%20through%20its%20Go%20API%20and%20the%20cue%20command,%20enabling%20schemas%20to%20be%20used%20to%20constrain%20and%20validate%20data%20directly,%20and%20to%20be%20expressed%20in%20other%20formats%20-%20including%20CUE.%20Read%20more%20Read%20more%20Protocol%20Buffers%20CUE&rsquo;s%20Go%20API%20and%20the%20cue%20command%20read%20Protobuf%20definitions,%20enabling%20them%20to%20be%20used%20to%20constrain%20and%20validate%20data%20directly%20and%20to%20be%20expressed%20in%20other%20formats.%20CUE%20constraints%20can%20be%20extracted%20from%20Protobuf%20options,%20allowing%20richer%20data%20validation%20than%20Protobuf&rsquo;s%20type-based%20defaults.%20Learn%20more%20Learn%20more%20JSON%20Schema%20CUE%20understands%20JSON%20Schema%20constraints%20through%20its%20Go%20API%20and%20the%20cue%20command,%20enabling%20schemas%20to%20be%20used%20to%20constrain%20and%20validate%20data%20directly,%20and%20to%20be%20expressed%20in%20other%20formats%20-%20including%20CUE.%20Read%20more%20Read%20more%20Java%20Technology%20preview%0a]

 * Share on Linkedin
   
   [https://www.linkedin.com/shareArticle?mini=true&url=https://cuelang.org/docs/integration/&summary=CUE%20has%20first-class%20support%20for%20a%20growing%20range%20of%20formats%20and%20ecosystems:%0aJSON%20CUE%20reads,%20writes,%20and%20validates%20JSON%20data,%20regardless%20of%20its%20source:%20discrete%20JSON%20files,%20I/O%20streams,%20or%20JSON%20contained%20in%20data%20fields%20inside%20other%20formats.%20The%20cue%20command%20emits%20JSON%20by%20default.%20Learn%20more%20Learn%20more%20YAML%20CUE&rsquo;s%20support%20for%20YAML%20mirrors%20its%20first%20class%20JSON%20support:%20it%20reads,%20writes%20and%20validates%20YAML%20data%20anywhere%20it%20can%20be%20found:%20discrete%20YAML%20files,%20I/O%20streams,%20or%20YAML%20contained%20in%20data%20fields%20inside%20other%20formats.%20Read%20more%20Read%20more%20Go%20The%20cue%20command%20converts%20Go%20types%20to%20CUE,%20enabling%20their%20first%20class%20use%20as%20schema%20and%20data%20constraints.%20CUE&rsquo;s%20extensive%20Go%20API%20allows%20code%20to%20have%20fine%20grained%20control%20over%20CUE&rsquo;s%20capabilities%20and%20operations,%20including%20export%20to%20any%20encoding%20supported%20by%20CUE.%20Learn%20more%20Learn%20more%20TOML%20CUE%20supports%20reading%20and%20writing%20TOML%20data%20wherever%20it&rsquo;s%20found:%20discrete%20TOML%20data%20files,%20I/O%20streams,%20or%20encoded%20in%20string%20fields%20inside%20other%20formats.%20Find%20out%20more%20Find%20out%20more%20OpenAPI%20CUE%20reads%20and%20writes%20OpenAPI%20data%20schemas%20through%20its%20Go%20API%20and%20the%20cue%20command,%20enabling%20schemas%20to%20be%20used%20to%20constrain%20and%20validate%20data%20directly,%20and%20to%20be%20expressed%20in%20other%20formats%20-%20including%20CUE.%20Read%20more%20Read%20more%20Protocol%20Buffers%20CUE&rsquo;s%20Go%20API%20and%20the%20cue%20command%20read%20Protobuf%20definitions,%20enabling%20them%20to%20be%20used%20to%20constrain%20and%20validate%20data%20directly%20and%20to%20be%20expressed%20in%20other%20formats.%20CUE%20constraints%20can%20be%20extracted%20from%20Protobuf%20options,%20allowing%20richer%20data%20validation%20than%20Protobuf&rsquo;s%20type-based%20defaults.%20Learn%20more%20Learn%20more%20JSON%20Schema%20CUE%20understands%20JSON%20Schema%20constraints%20through%20its%20Go%20API%20and%20the%20cue%20command,%20enabling%20schemas%20to%20be%20used%20to%20constrain%20and%20validate%20data%20directly,%20and%20to%20be%20expressed%20in%20other%20formats%20-%20including%20CUE.%20Read%20more%20Read%20more%20Java%20Technology%20preview%0a]


Previous
Next

 * Introduction [/docs/introduction/]
 * Tour [/docs/tour/]
 * Integrations [/docs/integration/]
    1. Technologies
   
   * JSON [/docs/integration/json/]
   * YAML [/docs/integration/yaml/]
   * Go [/docs/integration/go/]
   * OpenAPI [/docs/integration/openapi/]
   * Protocol Buffers [/docs/integration/protobuf/]
 * Tutorials [/docs/tutorial/]
 * How-to Guides [/docs/howto/]
 * Concept Guides [/docs/concept/]
 * References [/docs/reference/]

Hide side navigation


Show side navigation

Get Started

 * Documentation [/docs/]
 * Language Tour [/docs/tour/]
 * Playground [/play/]
 * Install CUE [/docs/introduction/installation/]

Community

 * The CUE Community [/community]
 * Contributing [https://github.com/cue-lang/cue/blob/master/CONTRIBUTING.md#contribution-guide]
 * Code of Conduct [/docs/reference/code-of-conduct/]
 * Slack Workspace [/s/slack]
 * Discord Server [/s/discord]

Connect

 * GitHub [https://github.com/cue-lang/cue]
 * X (Twitter) [https://twitter.com/cue_lang]
 * Bluesky [https://bsky.app/profile/cuelang.org]
 * YouTube [https://www.youtube.com/@cuelang/videos]

 * © 2025 CUE
 * Privacy policy [/privacy-policy/]
 * Report an Issue [https://github.com/cue-lang/cue/issues/new?labels=Triage,NeedsInvestigation,cuelang.org&title=cuelang.org:%20&template=bug_report.md&body=%23%23%23+What+page+were+you+looking+at%3F%0A%0Ahttps%3A%2F%2Fcuelang.org%2Fdocs%2Fintegration%2F%0A%0A%23%23%23+What+version+of+the+site+were+you+looking+at%3F%0A%0Ahttps%3A%2F%2Fgithub.com%2Fcue-lang%2Fcuelang.org%2Fcommit%2F6215397cbbdb765fedde5348b4c1f2d7e119dff1%0A%0A%23%23%23+What+did+you+do%3F%0A%0A%0A%0A%23%23%23+What+did+you+expect%3F%0A%0A%0A%0A%23%23%23+What+did+you+see+instead%3F%0A%0A]


Homepage of CUE [/]
CUE v0.15 is now available – learn more about its new features and improvements [https://github.com/cue-lang/cue/releases/tag/v0.15.0]
Install CUE

[/docs/introduction/installation/]

Close

Homepage of CUE [/]


Hide menu
 * Documentation [/docs/]
 * Play [/play/]
 * Community [/community/]
 * Install [/docs/introduction/installation/]
 * 

 * 
   GitHub [https://github.com/cue-lang/cue]
 * 
   Slack [/s/slack]
 * 
   Discord [/s/discord]
 * 
   X (Twitter) [https://twitter.com/cue_lang]
 * 
   Bluesky [https://bsky.app/profile/cuelang.org]
 * 
   YouTube [https://www.youtube.com/@cuelang/videos]
## See Also

- [Documentation Index](./COMPASS.md)
