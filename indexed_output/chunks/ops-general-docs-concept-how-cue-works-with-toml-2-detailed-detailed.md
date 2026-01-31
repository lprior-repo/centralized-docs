---
doc_id: ops/general/docs-concept-how-cue-works-with-toml
chunk_id: ops/general/docs-concept-how-cue-works-with-toml#2-detailed
chunk_level: detailed
chunk_type: prose
heading: Introduction
token_count: 1032
summary: Open share options.  * Share on X (Twitter)
---


Open share options
 * 
   
   Copy link
   Copied!

 * Share on X (Twitter)
   
   [https://twitter.com/intent/tweet?url=https://cuelang.org/docs/concept/how-cue-works-with-toml/&text=%20Reading%20and%20writing%20TOML%20The%20cue%20command%20natively%20supports%20reading%20and%20writing%20TOML%20files%20and%20data.%20TOML%20can%20be%20processed%20by%20CUE&rsquo;s%20wide%20range%20of%20data,%20schema,%20and%20policy%20validation%20capabilities.%20Data%20in%20any%20supported%20encoding%20can%20be%20read%20and%20exported%20as%20TOML%20&ndash;%20as%20demonstrated%20here%20by%20cue%20export%20unifying%20its%20TOML,%20JSON,%20and%20CUE%20input%20files%20and%20producing%20TOML:%0aCopied!%20a.toml%20Copied!%20b.json%20Copied!%20c.cue%20Copy%20code%20Copied!%20a%20=%20&#34;1&#34;%20[b]%20c%20=%202.2%20[b.d]%20e%20=%203%20Copy%20code%20Copied!%20%7b%20&#34;f&#34;:%20&#34;4&#34;,%20&#34;g&#34;:%205.5%20%7d%20Copy%20code%20Copied!%20b:%20_%20g:%20_%20h:%20&#34;six&#34;%20b:%20d:%20i:%20g%20+%20b.d.e%20TERMINAL%20Copy%20code%20Copied!%20$%20cue%20export%20--out%20toml%20a.toml%20b.json%20c.cue%20a%20=%20&#39;1&#39;%20f%20=%20&#39;4&#39;%20g%20=%205.5%20h%20=%20&#39;six&#39;%20[b]%20c%20=%202.2%20[b.d]%20e%20=%203%20i%20=%208.5%20The%20cue%20command%20can%20read%20and%20write%20a%20range%20of%20other%20formats%20as%20well%20as%20TOML.%0a]

 * Share on Linkedin
   
   [https://www.linkedin.com/shareArticle?mini=true&url=https://cuelang.org/docs/concept/how-cue-works-with-toml/&summary=%20Reading%20and%20writing%20TOML%20The%20cue%20command%20natively%20supports%20reading%20and%20writing%20TOML%20files%20and%20data.%20TOML%20can%20be%20processed%20by%20CUE&rsquo;s%20wide%20range%20of%20data,%20schema,%20and%20policy%20validation%20capabilities.%20Data%20in%20any%20supported%20encoding%20can%20be%20read%20and%20exported%20as%20TOML%20&ndash;%20as%20demonstrated%20here%20by%20cue%20export%20unifying%20its%20TOML,%20JSON,%20and%20CUE%20input%20files%20and%20producing%20TOML:%0aCopied!%20a.toml%20Copied!%20b.json%20Copied!%20c.cue%20Copy%20code%20Copied!%20a%20=%20&#34;1&#34;%20[b]%20c%20=%202.2%20[b.d]%20e%20=%203%20Copy%20code%20Copied!%20%7b%20&#34;f&#34;:%20&#34;4&#34;,%20&#34;g&#34;:%205.5%20%7d%20Copy%20code%20Copied!%20b:%20_%20g:%20_%20h:%20&#34;six&#34;%20b:%20d:%20i:%20g%20+%20b.d.e%20TERMINAL%20Copy%20code%20Copied!%20$%20cue%20export%20--out%20toml%20a.toml%20b.json%20c.cue%20a%20=%20&#39;1&#39;%20f%20=%20&#39;4&#39;%20g%20=%205.5%20h%20=%20&#39;six&#39;%20[b]%20c%20=%202.2%20[b.d]%20e%20=%203%20i%20=%208.5%20The%20cue%20command%20can%20read%20and%20write%20a%20range%20of%20other%20formats%20as%20well%20as%20TOML.%0a]


How CUE works with Protocol Buffers
[/docs/concept/how-cue-works-with-protocol-buffers/]How CUE works with YAML
[/docs/concept/how-cue-works-with-yaml/]
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
   * How CUE works with TOML [/docs/concept/how-cue-works-with-toml/]
      1. Reading and writing TOML
      2. Validating TOML files against a schema
      3. Processing and transforming TOML files
      4. Embedding TOML file data inside CUE
      5. Encoding TOML inside CUE
      6. Converting TOML files to CUE
      7. Related content
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
