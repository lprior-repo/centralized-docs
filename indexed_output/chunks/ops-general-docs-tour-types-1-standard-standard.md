---
doc_id: ops/general/docs-tour-types
chunk_id: ops/general/docs-tour-types#1-standard
chunk_level: standard
chunk_type: prose
heading: Introduction
token_count: 759
summary: However, notice that we didn’t need to specify its values explicitly. CUE is able to infer from the constraints applied, placing origin on
---

However, notice that we didn’t need to specify its values explicitly.
CUE is able to infer from the constraints applied, placing origin on
both the x-axis and y-axis, that its coordinates must be (x = 0, y = 0).

The output in this example is produced by
cue eval [/docs/reference/command/cue-help-eval/].
This command validates a configuration but, unlike cue export, doesn’t
require it to be completely concrete.
cue eval produces CUE, not JSON or YAML.

Last modified May 23, 2025 [https://github.com/cue-lang/cuelang.org/commit/2bb11f637a7a1cfe18b3bed5c0717cf07f5ea21d]


Open share options
 * 
   
   Copy link
   Copied!

 * Share on X (Twitter)
   
   [https://twitter.com/intent/tweet?url=https://cuelang.org/docs/tour/types/types/&text=CUE%20defines%20the%20following%20type%20hierarchy:%0aflowchart%20TD%20top[%22_%22]%20bottom[%22_%7c_%22]%20struct[%22%7b...%7d%22]%20list[%22[...]%22]%20top%20--%3e%20null%20--%3e%20bottom%20top%20--%3e%20bool%20--%3e%20bottom%20top%20--%3e%20string%20--%3e%20bottom%20top%20--%3e%20bytes%20--%3e%20bottom%20top%20--%3e%20number%20--%3e%20int%20&%20float%20--%3e%20bottom%20top%20--%3e%20struct%20--%3e%20bottom%20top%20--%3e%20list%20--%3e%20bottom%20CUE&#39;s%20predefined%20type%20hierarchy%20CUE%20defines%20the%20value%20top%20%28or%20any%29,%20written%20&ldquo;_&rdquo;,%20such%20that%20all%20types%20are%20an%20instance%20of%20top,%20and%20the%20value%20bottom%20%28or%20error%29,%20written%20&ldquo;_%7c_&rdquo;,%20which%20is%20an%20instance%20of%20all%20types.%0aWe%20can%20mix%20the%20terms%20types%20and%20values%20interchangeably%20because%20CUE%20doesn&rsquo;t%20distinguish%20between%20types%20and%20values.%0aThe%20term%20&ldquo;type&rdquo;%20merely%20refers%20to%20the%20kind%20of%20a%20value,%20which%20may%20or%20may%20not%20be%20a%20concrete%20instance.%0a]

 * Share on Linkedin
   
   [https://www.linkedin.com/shareArticle?mini=true&url=https://cuelang.org/docs/tour/types/types/&summary=CUE%20defines%20the%20following%20type%20hierarchy:%0aflowchart%20TD%20top[%22_%22]%20bottom[%22_%7c_%22]%20struct[%22%7b...%7d%22]%20list[%22[...]%22]%20top%20--%3e%20null%20--%3e%20bottom%20top%20--%3e%20bool%20--%3e%20bottom%20top%20--%3e%20string%20--%3e%20bottom%20top%20--%3e%20bytes%20--%3e%20bottom%20top%20--%3e%20number%20--%3e%20int%20&%20float%20--%3e%20bottom%20top%20--%3e%20struct%20--%3e%20bottom%20top%20--%3e%20list%20--%3e%20bottom%20CUE&#39;s%20predefined%20type%20hierarchy%20CUE%20defines%20the%20value%20top%20%28or%20any%29,%20written%20&ldquo;_&rdquo;,%20such%20that%20all%20types%20are%20an%20instance%20of%20top,%20and%20the%20value%20bottom%20%28or%20error%29,%20written%20&ldquo;_%7c_&rdquo;,%20which%20is%20an%20instance%20of%20all%20types.%0aWe%20can%20mix%20the%20terms%20types%20and%20values%20interchangeably%20because%20CUE%20doesn&rsquo;t%20distinguish%20between%20types%20and%20values.%0aThe%20term%20&ldquo;type&rdquo;%20merely%20refers%20to%20the%20kind%20of%20a%20value,%20which%20may%20or%20may%20not%20be%20a%20concrete%20instance.%0a]
