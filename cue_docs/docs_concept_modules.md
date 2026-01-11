# Modules | CUE

**Source:** https://cuelang.org/docs/concept/modules/

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

 1. Concept Guides [https://cuelang.org/docs/concept/]


 2. MODULES

 * modules [/search?q=tag:modules]

Modules are how CUE manages native dependencies.
They can be published to a registry, which allows users to fetch and work with their contents.

Learn about modules using these recommended guides, or
browse through all the modules
[/search?q=tag:modules]pages listed below:

RECOMMENDED GUIDES


REFERENCE MANUAL

The CUE modules reference manual and glossary
reference/manual

reference/manual [/docs/reference/modules/]


CONCEPT GUIDE

CUE’s “new” versus “old” modules – your questions: answered
concept/faq

concept/faq [/docs/concept/faq/new-modules-vs-old-modules/]


THE CUE COMMAND

The built-in help text for
cue help modules
reference/command/cue

reference/command/cue [/docs/reference/command/cue-help-modules]


GETTING STARTED

Working with the CUE Central Registry
tutorial

tutorial [/docs/tutorial/working-with-the-central-registry/]


COLLABORATE

Publishing modules to the Central Registry
tutorial

tutorial [/docs/tutorial/publishing-modules-to-the-central-registry/]


PRIVATE INFRASTRUCTURE

Working with a custom module registry
tutorial

tutorial [/docs/tutorial/working-with-a-custom-module-registry/]

ALL PAGES TAGGED “MODULES”

Search in cuelang.org
tag:modules
22 documents found

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
   
   
 * 
   
   Clear all items
   
   

No tags found

Content Type


Filter by Content Type


Filter Content Type


Loading

 * 
   
   
   Community
   
   
 * 
   
   
   Introduction
   
   
 * 
   
   
   Tour
   
   
 * 
   
   
   Integrations
   
   
 * 
   
   
   Tutorials
   
   
 * 
   
   
   How-to Guides
   
   
 * 
   
   
   Concept Guides
   
   
 * 
   
   
   References
   
   
 * 
   
   
   Privacy policy
   
   

No Content Type's found

 * 
   
   
   
   
   Documentation / References / The cue command
   
   
   
   
   
   
   CUE HELP MOD PUBLISH
   
   
   
   TERMINAL Copy code Copied! $ cue help mod publish Publish the current module to an OCI registry. It consults $CUE_REGISTRY to determine where the module should …
   
    * 
      cue command
      
    * 
      modules
      
   
   
   
   Read more
   [/docs/reference/command/cue-help-mod-publish/]
   
   
 * 
   
   
   
   
   Documentation / References / The cue command
   
   
   
   
   
   
   CUE HELP REGISTRYCONFIG
   
   
   
   TERMINAL Copy code Copied! $ cue help registryconfig The registry configuration determines how CUE maps modules to their locations in OCI registries. Given a particular registry configuration, it …
   
    * 
      cue command
      
    * 
      modules
      
   
   
   
   Read more
   [/docs/reference/command/cue-help-registryconfig/]
   
   
 * 
   
   
   
   
   Documentation / Concept Guides
   
   
   
   
   
   
   CUE LANGUAGE VERSIONS
   
   
   
   The cue.mod/module.cue file has a mandatory language.version field that sets the language version for a given module. But what does it …
   
    * 
      modules
      
   
   
   
   Read more
   [/docs/concept/cue-language-version/]
   
   
 * 
   
   
   
   
   Documentation / Concept Guides / Frequently Asked Questions
   
   
   
   
   
   
   NEW MODULES VS OLD MODULES
   
   
   
   This guide answers some Frequently Asked Questions about the differences between CUE’s old modules implementation and its new modules implementation that became the default with CUE v0.9 …
   
    * 
      modules
      
   
   
   
   Read more
   [/docs/concept/faq/new-modules-vs-old-modules/]
   
   
 * 
   
   
   
   
   Documentation / Tutorials
   
   
   
   
   
   
   PUBLISHING MODULES TO THE CENTRAL REGISTRY
   
   
   
   Introduction In this tutorial you will publish a module to the Central Registry and then create a second module that depends on the first.
   
   
    * 
      modules
      
    * 
      tooling
      
    * 
      cue command
      
   
   
   
   Read more
   [/docs/tutorial/publishing-modules-to-the-central-registry/]
   
   
 * 
   
   
   
   
   Documentation / References / The cue command
   
   
   
   
   
   
   CUE HELP MOD
   
   
   
   TERMINAL Copy code Copied! $ cue help mod Mod groups commands which operate on CUE modules. Note that support for modules is built into all the cue commands …
   
    * 
      cue command
      
    * 
      modules
      
   
   
   
   Read more
   [/docs/reference/command/cue-help-mod/]
   
   
 * 
   
   
   
   
   Documentation / References / The cue command
   
   
   
   
   
   
   CUE HELP MOD RESOLVE
   
   
   
   TERMINAL Copy code Copied! $ cue help mod resolve This command prints information about how a given module path will resolve to an actual registry in the form of …
   
    * 
      cue command
      
    * 
      modules
      
   
   
   
   Read more
   [/docs/reference/command/cue-help-mod-resolve/]
   
   
 * 
   
   
   
   
   Documentation / References / The cue command
   
   
   
   
   
   
   CUE HELP MOD TIDY
   
   
   
   TERMINAL Copy code Copied! $ cue help mod tidy Tidy resolves all module dependencies in the current module and updates the cue.mod/module.cue file …
   
    * 
      cue command
      
    * 
      modules
      
   
   
   
   Read more
   [/docs/reference/command/cue-help-mod-tidy/]
   
   
 * 
   
   
   
   
   Documentation / Concept Guides
   
   
   
   
   
   
   OLD MODULES, PACKAGES, AND INSTANCES
   
   
   
   This guide describes “old” modules, which were the previous implementation of dependencies in CUE. It’s preserved here for folks using previous versions of CUE, but if you …
   
    * 
      modules
      
   
   
   
   Read more
   [/docs/concept/old-modules-packages-instances/]
   
   
 * 
   
   
   
   
   Documentation / References / The cue command
   
   
   
   
   
   
   CUE HELP MODULES
   
   
   
   TERMINAL Copy code Copied! $ cue help modules Modules are how CUE publishes packages and manages dependencies. A module is a collection of packages that are released, versioned …
   
    * 
      cue command
      
    * 
      modules
      
   
   
   
   Read more
   [/docs/reference/command/cue-help-modules/]
   
   
 * 
   
   
   
   
   Documentation / References / The cue command
   
   
   
   
   
   
   CUE HELP LOGIN
   
   
   
   TERMINAL Copy code Copied! $ cue help login Log into a CUE registry via the OAuth 2.0 Device Authorization Grant. Without an argument, CUE_REGISTRY …
   
    * 
      cue command
      
    * 
      modules
      
   
   
   
   Read more
   [/docs/reference/command/cue-help-login/]
   
   
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
   
   
   
   
   Documentation / Tutorials
   
   
   
   
   
   
   WORKING WITH MODULES AND THE CENTRAL REGISTRY
   
   
   
   Introduction In this tutorial you will learn how to create and work with CUE modules, using the Central Registry.
   Along the way you will:
   Create a module that …
   
    * 
      modules
      
   
   
   
   Read more
   [/docs/tutorial/working-with-the-central-registry/]
   
   
 * 
   
   
   
   
   Documentation / References / The cue command
   
   
   
   
   
   
   CUE HELP MOD EDIT
   
   
   
   TERMINAL Copy code Copied! $ cue help mod edit Edit provides a command-line interface for editing cue.mod/module.cue. It reads only …
   
    * 
      cue command
      
    * 
      modules
      
   
   
   
   Read more
   [/docs/reference/command/cue-help-mod-edit/]
   
   
 * 
   
   
   
   
   Documentation / References
   
   
   
   
   
   
   CUE MODULES
   
   
   
   Introduction Modules are how CUE manages dependencies. This document is a detailed reference manual for CUE’s module system. CUE’s modules support has a lot in …
   
    * 
      modules
      
   
   
   
   Read more
   [/docs/reference/modules/]
   
   
 * 
   
   
   
   
   Documentation / How-to Guides
   
   
   
   
   
   
   MIRRORING MODULES BETWEEN REGISTRIES
   
   
   
   Requires CUE v0.13.0 or later This guide demonstrates how to use the cue mod mirror command to copy CUE modules between registries.
   
   
    * 
      modules
      
   
   
   
   Read more
   [/docs/howto/mirror-modules-between-registries/]
   
   
 * 
   
   
   
   
   Documentation / Tutorials
   
   
   
   
   
   
   USING MODULES WITH THE GO API
   
   
   
   Introduction In this tutorial you will use CUE’s Go API to work with a CUE module dependency fetched from the Central Registry.
   Along the way you will …
   
    * 
      go api
      
    * 
      modules
      
   
   
   
   Read more
   [/docs/tutorial/using-modules-with-go-api/]
   
   
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
   
   
   
   
   Documentation / References / The cue command
   
   
   
   
   
   
   CUE HELP MOD GET
   
   
   
   TERMINAL Copy code Copied! $ cue help mod get Get updates module dependencies, fetching new dependencies if needed and changing versions to specified versions. It can downgrade a …
   
    * 
      cue command
      
    * 
      modules
      
   
   
   
   Read more
   [/docs/reference/command/cue-help-mod-get/]
   
   
 * 
   
   
   
   
   Documentation / References / The cue command
   
   
   
   
   
   
   CUE HELP MOD INIT
   
   
   
   TERMINAL Copy code Copied! $ cue help mod init Init initializes a cue.mod directory in the current directory, in effect creating a new module rooted at the …
   
    * 
      cue command
      
    * 
      modules
      
   
   
   
   Read more
   [/docs/reference/command/cue-help-mod-init/]
   
   
 * 
   
   
   
   
   Documentation / Tutorials
   
   
   
   
   
   
   WORKING WITH A CUSTOM MODULE REGISTRY
   
   
   
   Introduction In this tutorial you will learn how to create and work with CUE modules, using a custom module registry.
   Along the way you will:
   Define a module …
   
    * 
      modules
      
    * 
      tooling
      
    * 
      cue command
      
   
   
   
   Read more
   [/docs/tutorial/working-with-a-custom-module-registry/]
   
   
 * 
   
   
   
   
   Documentation / References / The cue command
   
   
   
   
   
   
   CUE HELP MOD FIX
   
   
   
   TERMINAL Copy code Copied! $ cue help mod fix Fix provides a way to migrate from a legacy module.cue file to the new standard syntax. It …
   
    * 
      cue command
      
    * 
      modules
      
   
   
   
   Read more
   [/docs/reference/command/cue-help-mod-fix/]
   
   

Last modified May 23, 2025 [https://github.com/cue-lang/cuelang.org/commit/2bb11f637a7a1cfe18b3bed5c0717cf07f5ea21d]

 * modules [/search?q=tag:modules]


Open share options
 * 
   
   Copy link
   Copied!

 * Share on X (Twitter)
   
   [https://twitter.com/intent/tweet?url=https://cuelang.org/docs/concept/modules/&text=Modules%20are%20how%20CUE%20manages%20native%20dependencies.%20They%20can%20be%20published%20to%20a%20registry,%20which%20allows%20users%20to%20fetch%20and%20work%20with%20their%20contents.%0aLearn%20about%20modules%20using%20these%20recommended%20guides,%20or%20browse%20through%20all%20the%20modules%20pages%20listed%20below:%0a]

 * Share on Linkedin
   
   [https://www.linkedin.com/shareArticle?mini=true&url=https://cuelang.org/docs/concept/modules/&summary=Modules%20are%20how%20CUE%20manages%20native%20dependencies.%20They%20can%20be%20published%20to%20a%20registry,%20which%20allows%20users%20to%20fetch%20and%20work%20with%20their%20contents.%0aLearn%20about%20modules%20using%20these%20recommended%20guides,%20or%20browse%20through%20all%20the%20modules%20pages%20listed%20below:%0a]


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
      1. Recommended guides
      2. All pages tagged “modules”
   
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
 * Report an Issue [https://github.com/cue-lang/cue/issues/new?labels=Triage,NeedsInvestigation,cuelang.org&title=cuelang.org:%20&template=bug_report.md&body=%23%23%23+What+page+were+you+looking+at%3F%0A%0Ahttps%3A%2F%2Fcuelang.org%2Fdocs%2Fconcept%2Fmodules%2F%0A%0A%23%23%23+What+version+of+the+site+were+you+looking+at%3F%0A%0Ahttps%3A%2F%2Fgithub.com%2Fcue-lang%2Fcuelang.org%2Fcommit%2F6215397cbbdb765fedde5348b4c1f2d7e119dff1%0A%0A%23%23%23+What+did+you+do%3F%0A%0A%0A%0A%23%23%23+What+did+you+expect%3F%0A%0A%0A%0A%23%23%23+What+did+you+see+instead%3F%0A%0A]


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