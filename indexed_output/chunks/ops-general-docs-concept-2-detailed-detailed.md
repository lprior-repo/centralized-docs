---
doc_id: ops/general/docs-concept
chunk_id: ops/general/docs-concept#2-detailed
chunk_level: detailed
chunk_type: prose
heading: Introduction
token_count: 1065
summary:    HOW CUE ENABLES BOILERPLATE REMOVAL.    Boilerplate is noise that obscures the parts of a configuration that actually matter
---

   
   
   
   
   
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
