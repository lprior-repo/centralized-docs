---
doc_id: ref/docs-contribute-style-style-guide.md/docs-contribute-style-style-guide
chunk_id: ref/docs-contribute-style-style-guide.md/docs-contribute-style-style-guide#17-standard
chunk_level: standard
chunk_type: table
heading: Shortcodes
token_count: 491
summary: ### Headings and titles People accessing this documentation may use a screen reader or other assistive technology (AT). [Screen readers](https://en.wikipedia.org/wiki/Screen_reader) are linear output...
---

### Headings and titles
People accessing this documentation may use a screen reader or other assistive technology (AT).
[Screen readers](https://en.wikipedia.org/wiki/Screen_reader) are linear output devices,
they output items on a page one at a time. If there is a lot of content on a page, you can
use headings to give the page an internal structure. A good page structure helps all readers
to easily navigate the page or filter topics of interest.
Do and Don't - Headings|Do|Don't|
|Update the title in the front matter of the page or blog post.|Use first level heading, as Hugo automatically converts the title in the front matter of the page into a first-level heading.|
|Use ordered headings to provide a meaningful high-level outline of your content.|Use headings level 4 through 6, unless it is absolutely necessary. If your content is that detailed, it may need to be broken into separate articles.|
|Use pound or hash signs (`#`) for non-blog post content.|Use underlines (`---` or `===`) to designate first-level headings.|
|Use sentence case for headings in the page body. For example, **Extend kubectl with plugins**|Use title case for headings in the page body. For example, **Extend Kubectl With Plugins**|
|Use title case for the page title in the front matter. For example, `title: Kubernetes API Server Bypass Risks`|Use sentence case for page titles in the front matter. For example, don't use `title: Kubernetes API server bypass risks`|
|Place relevant links in the body copy.|Include hyperlinks (`&lt;a href=""&gt;&lt;/a&gt;`) in headings.|
|Use pound or hash signs (`#`) to indicate headings.|Use **bold** text or other indicators to split paragraphs.|
### Paragraphs
Do and Don't - Paragraphs|Do|Don't|
|Try to keep paragraphs under 6 sentences.|Indent the first paragraph with space characters. For example, ⋅⋅⋅Three spaces before a paragraph will indent it.|
|Use three hyphens (`---`) to create a horizontal rule. Use horizontal rules for breaks in paragraph content. For example, a change of scene in a story, or a shift of topic within a section.|Use horizontal rules for decoration.|