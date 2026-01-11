# Concept Guides | CUE

**Source:** https://cuelang.org/docs/concept/

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

 1. CONCEPT GUIDES

Concept guides explain the systems and ideas that drive CUE’s design.
They explore the foundations and implications of CUE’s theoretical basis in
depth, and provide background information to help the CUE user discover how
best to take advantage of its unique and powerful features.

The Popular guides [/docs/concept/popular-guides/] page contains a curated
list of concept guides.
You can also use the following index to browse through the full range of concept guides,
or to search inside them for specific content, titles, and tags:

Search in Concept Guides

34 documents found

Filters
ⓘ [/docs/howto/search-this-site/]

Tags


Filter by tags


Filter tags


Loading

 * 
   
   
   ecosystem
   
   
 * 
   
   
   encodings
   
   
 * 
   
   
   cue command
   
   
 * 
   
   
   language
   
   
 * 
   
   
   validation
   
   
 * 
   
   
   tooling
   
   
 * 
   
   
   commented cue
   
   
 * 
   
   
   user question
   
   
 * 
   
   
   modules
   
   
 * 
   
   
   go api
   
   
 * 
   
   
   java api
   
   
 * 
   
   
   workflow command
   
   

No tags found

 * 
   
   
   
   
   Documentation / Concept Guides
   
   
   
   
   
   
   HOW CUE WORKS WITH JSON
   
   
   
   Reading and writing JSON CUE is a superset of JSON. In other words: all valid JSON is CUE.
   The cue tool natively supports reading and writing JSON …
   
    * 
      encodings
      
    * 
      cue command
      
   
   
   
   Read more
   [/docs/concept/how-cue-works-with-json/]
   
   
 * 
   
   
   
   
   Documentation / Concept Guides
   
   
   
   
   
   
   QUERYING USE CASE
   
   
   
   CUE orders all values in a value lattice. A value more at the top of a hierarchy is what programming languages would refer to as a type. Concrete …
   
   
   
   Read more
   [/docs/concept/querying-use-case/]
   
   
 * 
   
   
   
   
   Documentation / Concept Guides
   
   
   
   
   
   
   POPULAR GUIDES
   
   
   
   The Logic of CUE CUE Modules Using the "cue export" command Common use cases How CUE enables configuration How CUE enables data validation How CUE enables boilerplate removal Schema …
   
   
   
   Read more
   [/docs/concept/popular-guides/]
   
   
 * 
   
   
   
   
   Documentation / Concept Guides
   
   
   
   
   
   
   CUE LANGUAGE VERSIONS
   
   
   
   The cue.mod/module.cue file has a mandatory language.version field that sets the language version for a given module. But what does it …
   
    * 
      modules
      
   
   
   
   Read more
   [/docs/concept/cue-language-version/]
   
   
 * 
   
   
   
   
   Documentation / Concept Guides
   
   
   
   
   
   
   HOW CUE WORKS WITH OPENAPI
   
   
   
   CUE has first class support for OpenAPI data schemas: the cue command automatically recognises OpenAPI by its signature fields, and the Go API has packages dedicated to the format. …
   
    * 
      encodings
      
    * 
      cue command
      
   
   
   
   Read more
   [/docs/concept/how-cue-works-with-openapi/]
   
   
 * 
   
   
   
   
   Documentation / Concept Guides
   
   
   
   
   
   
   HOW CUE WORKS WITH TOML
   
   
   
   Reading and writing TOML The cue command natively supports reading and writing TOML files and data. TOML can be processed by CUE’s wide range of data, schema …
   
    * 
      encodings
      
    * 
      cue command
      
   
   
   
   Read more
   [/docs/concept/how-cue-works-with-toml/]
   
   
 * 
   
   
   
   
   Documentation / Concept Guides / Frequently Asked Questions
   
   
   
   
   
   
   NEW MODULES VS OLD MODULES
   
   
   
   This guide answers some Frequently Asked Questions about the differences between CUE’s old modules implementation and its new modules implementation that became the default with CUE v0.9 …
   
    * 
      modules
      
   
   
   
   Read more
   [/docs/concept/faq/new-modules-vs-old-modules/]
   
   
 * 
   
   
   
   
   Documentation / Concept Guides / Frequently Asked Questions
   
   
   
   
   
   
   SYMBOLIC LINK SUPPORT IN CUE
   
   
   
   Both the cue command and the CUE Go API support symbolic links (“symlinks”) on operating systems where the feature is available. However, symlinks are ignored when they are part …
   
   
   
   Read more
   [/docs/concept/faq/symbolic-link-support/]
   
   
 * 
   
   
   
   
   Documentation / Concept Guides
   
   
   
   
   
   
   USING THE CUE EXPORT COMMAND
   
   
   
   The cue export command takes a configuration and turns some of it (or all of it) into validated, concrete data encoded in a format such as JSON or YAML. …
   
    * 
      cue command
      
   
   
   
   Read more
   [/docs/concept/using-the-cue-export-command/]
   
   
 * 
   
   
   
   
   Documentation / Concept Guides
   
   
   
   
   
   
   CODE GENERATION AND EXTRACTION USE CASE
   
   
   
   Code generation and extraction is a broad topic and, for instance, overlaps with the topics discussed in Schema Definition and Go.
   In this section we emphasize the role of …
   
   
   
   Read more
   [/docs/concept/code-generation-and-extraction-use-case/]
   
   
 * 
   
   
   
   
   Documentation / Concept Guides
   
   
   
   
   
   
   OLD MODULES, PACKAGES, AND INSTANCES
   
   
   
   This guide describes “old” modules, which were the previous implementation of dependencies in CUE. It’s preserved here for folks using previous versions of CUE, but if you …
   
    * 
      modules
      
   
   
   
   Read more
   [/docs/concept/old-modules-packages-instances/]
   
   
 * 
   
   
   
   
   Documentation / Concept Guides
   
   
   
   
   
   
   MODULES, PACKAGES, AND INSTANCES
   
   
   
   This guide describes “new” modules, which are the current implementation of dependencies in CUE.
   The previous version of this guide covers “old” modules, which were the previous way …
   
    * 
      modules
      
   
   
   
   Read more
   [/docs/concept/modules-packages-instances/]
   
   
 * 
   
   
   
   
   Documentation / Concept Guides
   
   
   
   
   
   
   HOW CUE WORKS WITH YAML
   
   
   
   Reading and writing YAML The cue tool natively supports reading and writing YAML files, including those containing multiple documents.
   This allows YAML files to be processed by CUE …
   
    * 
      encodings
      
    * 
      cue command
      
   
   
   
   Read more
   [/docs/concept/how-cue-works-with-yaml/]
   
   
 * 
   
   
   
   
   Documentation / Concept Guides
   
   
   
   
   
   
   CONFIGURATION USE CASE
   
   
   
   Arguably, validation should be the foremost task of any configuration language. Most configuration languages, however, focus on boilerplate removal. CUE is different in that it takes the validation …
   
   
   
   Read more
   [/docs/concept/configuration-use-case/]
   
   
 * 
   
   
   
   
   Documentation / Concept Guides
   
   
   
   
   
   
   ALIAS AND REFERENCE SCOPES
   
   
   
   Problem #1 A user asked for help with a problem they were having with their CUE:
   Can you help me with problem1.cue? I'm trying to get …
   
    * 
      user question
      
   
   
   
   Read more
   [/docs/concept/alias-and-reference-scopes/]
   
   
 * 
   
   
   
   
   Documentation / Concept Guides / Using the cue export command
   
   
   
   
   
   
   EVALUATION
   
   
   
   As described on the previous page, each cue export invocation first identifies and reads its inputs.
   
   
   
   
   Read more
   [/docs/concept/using-the-cue-export-command/evaluation/]
   
   
 * 
   
   
   
   
   Documentation / Concept Guides
   
   
   
   
   
   
   HOW CUE ENABLES BOILERPLATE REMOVAL
   
   
   
   Boilerplate is noise that obscures the parts of a configuration that actually matter. It includes things like repeated field definitions, redundant defaults, and copy-pasted fragments that only …
   
   
   
   Read more
   [/docs/concept/how-cue-enables-boilerplate-removal/]
   
   
 * 
   
   
   
   
   Documentation / Concept Guides
   
   
   
   
   
   
   DATA VALIDATION USE CASE
   
   
   
   By far the most straightforward approach to specify data is in plain JSON or YAML files. Every value can be looked up right where it needs to be defined …
   
   
   
   Read more
   [/docs/concept/data-validation-use-case/]
   
   
 * 
   
   
   
   
   Documentation / Concept Guides / Using the cue export command
   
   
   
   
   
   
   OUTPUT
   
   
   
   By default, a successful cue export displays the evaluation result on its standard output stream, encoded in JSON:
   
   
   
   
   Read more
   [/docs/concept/using-the-cue-export-command/output/]
   
   
 * 
   
   
   
   
   Documentation / Concept Guides
   
   
   
   
   
   
   MODULES
   
   
   
   Modules are how CUE manages native dependencies. They can be published to a registry, which allows users to fetch and work with their contents.
   Learn about modules using …
   
    * 
      modules
      
   
   
   
   Read more
   [/docs/concept/modules/]
   
   
 * 
   
   
   
   
   Documentation / Concept Guides
   
   
   
   
   
   
   UNDERSTANDING CUE'S GO MODULE DEPENDENCIES
   
   
   
   This guide explains why certain dependencies appear in a Go module graph when using the CUE API. It provides conceptual clarity around how Go’s module system interacts with …
   
    * 
      go api
      
   
   
   
   Read more
   [/docs/concept/understanding-cue-go-module-dependencies/]
   
   
 * 
   
   
   
   
   Documentation / Concept Guides
   
   
   
   
   
   
   HOW CUE WORKS WITH GO
   
   
   
   CUE is designed to complement and work with the Go programming language. It offers a powerful API that enables Go code to take advantage of CUE’s advanced capabilites …
   
    * 
      encodings
      
    * 
      go api
      
   
   
   
   Read more
   [/docs/concept/how-cue-works-with-go/]
   
   
 * 
   
   
   
   
   Documentation / Concept Guides
   
   
   
   
   
   
   THE LOGIC OF CUE
   
   
   
   This page explains the core concept on which pretty much everything that is CUE depends. It helps to get a top-down understanding and frame of reference, but …
   
   
   
   Read more
   [/docs/concept/the-logic-of-cue/]
   
   
 * 
   
   
   
   
   Documentation / Concept Guides / Frequently Asked Questions
   
   
   
   
   
   
   UPGRADING FROM EVALV2 TO EVALV3
   
   
   
   evalv3 is now on by default in v0.13.0 and later versions! What is evalv3? evalv3 is a new major version of the CUE evaluator which …
   
   
   
   Read more
   [/docs/concept/faq/upgrading-from-evalv2-to-evalv3/]
   
   
 * 
   
   
   
   
   Documentation / Concept Guides
   
   
   
   
   
   
   HOW CUE WORKS WITH JSON SCHEMA
   
   
   
   CUE has first class support for JSON Schema: both the cue command and the Go API understand the format.
   Constraints stored as JSON Schema are available for cue commands …
   
    * 
      encodings
      
    * 
      cue command
      
    * 
      go api
      
   
   
   
   Read more
   [/docs/concept/how-cue-works-with-json-schema/]
   
   
 * 
   
   
   
   
   Documentation / Concept Guides
   
   
   
   
   
   
   SCHEMA DEFINITION USE CASE
   
   
   
   A data definition language describes the structure of data. The structure defined by such a language can, in turn, be used to verify implementations, validate inputs, or generate code …
   
    * 
      go api
      
    * 
      validation
      
   
   
   
   Read more
   [/docs/concept/schema-definition-use-case/]
   
   
 * 
   
   
   
   
   Documentation / Concept Guides
   
   
   
   
   
   
   HOW CUE ENABLES CONFIGURATION
   
   
   
   Configuration is one of CUE’s core tasks. People often start using CUE because it makes it safer and easier to produce new configuration files and to validate existing …
   
    * 
      validation
      
    * 
      workflow command
      
   
   
   
   Read more
   [/docs/concept/how-cue-enables-configuration/]
   
   
 * 
   
   
   
   
   Documentation / Concept Guides
   
   
   
   
   
   
   FREQUENTLY ASKED QUESTIONS
   
   
   
   CUE’s FAQs collect common questions about specific topics together on a single page, along with their answers.
   The following FAQs are available:
   
   
   
   
   Read more
   [/docs/concept/faq/]
   
   
 * 
   
   
   
   
   Documentation / Concept Guides / Frequently Asked Questions
   
   
   
   
   
   
   REMOVING LIST ARITHMETIC OPERATORS IN CUE 0.11
   
   
   
   This guide answers some questions about a language change that happened in CUE version 0.11 – including how to tell if the change affects your CUE, and how to …
   
    * 
      language
      
   
   
   
   Read more
   [/docs/concept/faq/removing-list-arithmetic-operators-v0.11/]
   
   
 * 
   
   
   
   
   Documentation / Concept Guides
   
   
   
   
   
   
   HOW CUE ENABLES DATA VALIDATION
   
   
   
   CUE is designed to make data validation simple, powerful, and flexible.
   To achieve this, the project publishes the cue command line tool, which allows a wide range of data …
   
    * 
      validation
      
   
   
   
   Read more
   [/docs/concept/how-cue-enables-data-validation/]
   
   
 * 
   
   
   
   
   Documentation
   
   
   
   
   
   
   CONCEPT GUIDES
   
   
   
   Concept guides explain the systems and ideas that drive CUE’s design. They explore the foundations and implications of CUE’s theoretical basis in depth, and provide background information …
   
   
   
   Read more
   [/docs/concept/]
   
   
 * 
   
   
   
   
   Documentation / Concept Guides / Using the cue export command
   
   
   
   
   
   
   INPUTS
   
   
   
   The cue export command can be given any number of inputs to evaluate via file or package arguments. This page explains how the command interprets, assembles, and combines …
   
   
   
   Read more
   [/docs/concept/using-the-cue-export-command/inputs/]
   
   
 * 
   
   
   
   
   Documentation / Concept Guides
   
   
   
   
   
   
   WORKING WITH INCOMPLETE CUE
   
   
   
   In general, CUE can handle references to fields that don’t yet exist, or where a value can’t be calculated because there’s insufficient information. CUE that contains …
   
    * 
      go api
      
   
   
   
   Read more
   [/docs/concept/working-with-incomplete-cue/]
   
   
 * 
   
   
   
   
   Documentation / Concept Guides
   
   
   
   
   
   
   HOW CUE WORKS WITH PROTOCOL BUFFERS
   
   
   
   Protocol Buffers, also known as Protobuf, is a language-neutral, platform-neutral, and extensible mechanism for serializing structured data, initially developed and released by Google.
   Protobuf definitions …
   
    * 
      encodings
      
    * 
      go api
      
   
   
   
   Read more
   [/docs/concept/how-cue-works-with-protocol-buffers/]
   
   

Last modified May 23, 2025 [https://github.com/cue-lang/cuelang.org/commit/2bb11f637a7a1cfe18b3bed5c0717cf07f5ea21d]


Open share options
 * 
   
   Copy link
   Copied!

 * Share on X (Twitter)
   
   [https://twitter.com/intent/tweet?url=https://cuelang.org/docs/concept/&text=Concept%20guides%20explain%20the%20systems%20and%20ideas%20that%20drive%20CUE&rsquo;s%20design.%20They%20explore%20the%20foundations%20and%20implications%20of%20CUE&rsquo;s%20theoretical%20basis%20in%20depth,%20and%20provide%20background%20information%20to%20help%20the%20CUE%20user%20discover%20how%20best%20to%20take%20advantage%20of%20its%20unique%20and%20powerful%20features.%0aThe%20Popular%20guides%20page%20contains%20a%20curated%20list%20of%20concept%20guides.%20You%20can%20also%20use%20the%20following%20index%20to%20browse%20through%20the%20full%20range%20of%20concept%20guides,%20or%20to%20search%20inside%20them%20for%20specific%20content,%20titles,%20and%20tags:%0a]

 * Share on Linkedin
   
   [https://www.linkedin.com/shareArticle?mini=true&url=https://cuelang.org/docs/concept/&summary=Concept%20guides%20explain%20the%20systems%20and%20ideas%20that%20drive%20CUE&rsquo;s%20design.%20They%20explore%20the%20foundations%20and%20implications%20of%20CUE&rsquo;s%20theoretical%20basis%20in%20depth,%20and%20provide%20background%20information%20to%20help%20the%20CUE%20user%20discover%20how%20best%20to%20take%20advantage%20of%20its%20unique%20and%20powerful%20features.%0aThe%20Popular%20guides%20page%20contains%20a%20curated%20list%20of%20concept%20guides.%20You%20can%20also%20use%20the%20following%20index%20to%20browse%20through%20the%20full%20range%20of%20concept%20guides,%20or%20to%20search%20inside%20them%20for%20specific%20content,%20titles,%20and%20tags:%0a]


Previous
Next

 * Introduction [/docs/introduction/]
 * Tour [/docs/tour/]
 * Integrations [/docs/integration/]
 * Tutorials [/docs/tutorial/]
 * How-to Guides [/docs/howto/]
 * Concept Guides [/docs/concept/]
   
   * Popular guides [/docs/concept/popular-guides/]
   * The Logic of CUE [/docs/concept/the-logic-of-cue/]
   * Modules [/docs/concept/modules/]
   * Frequently Asked Questions [/docs/concept/faq/]
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
 * Report an Issue [https://github.com/cue-lang/cue/issues/new?labels=Triage,NeedsInvestigation,cuelang.org&title=cuelang.org:%20&template=bug_report.md&body=%23%23%23+What+page+were+you+looking+at%3F%0A%0Ahttps%3A%2F%2Fcuelang.org%2Fdocs%2Fconcept%2F%0A%0A%23%23%23+What+version+of+the+site+were+you+looking+at%3F%0A%0Ahttps%3A%2F%2Fgithub.com%2Fcue-lang%2Fcuelang.org%2Fcommit%2F6215397cbbdb765fedde5348b4c1f2d7e119dff1%0A%0A%23%23%23+What+did+you+do%3F%0A%0A%0A%0A%23%23%23+What+did+you+expect%3F%0A%0A%0A%0A%23%23%23+What+did+you+see+instead%3F%0A%0A]


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