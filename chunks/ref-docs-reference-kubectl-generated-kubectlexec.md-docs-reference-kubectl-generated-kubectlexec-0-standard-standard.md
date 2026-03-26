---
doc_id: ref/docs-reference-kubectl-generated-kubectlexec.md/docs-reference-kubectl-generated-kubectlexec
chunk_id: ref/docs-reference-kubectl-generated-kubectlexec.md/docs-reference-kubectl-generated-kubectlexec#0-standard
chunk_level: standard
chunk_type: prose
heading: Table of Contents
token_count: 503
summary: ## Table of Contents    - [Synopsis](#synopsis)   - [Examples](#examples) - [Get output from running the 'date' command in ruby-container from pod...
---

## Table of Contents

  - [Synopsis](#synopsis)
  - [Examples](#examples)
- [Get output from running the 'date' command in ruby-container from pod mypod](#get-output-from-running-the-date-command-in-ruby-container-from-pod-mypod)
- [Switch to raw terminal mode; sends stdin to 'bash' in ruby-container from pod mypod](#switch-to-raw-terminal-mode-sends-stdin-to-bash-in-ruby-container-from-pod-mypod)
- [and sends stdout/stderr from 'bash' back to the client](#and-sends-stdoutstderr-from-bash-back-to-the-client)
- [List contents of /usr from the first container of pod mypod and sort by modification time](#list-contents-of-usr-from-the-first-container-of-pod-mypod-and-sort-by-modification-time)
- [If the command you want to execute in the pod has any flags in common (e.g. -i),](#if-the-command-you-want-to-execute-in-the-pod-has-any-flags-in-common-eg--i)
- [you must use two dashes (--) to separate your command's flags/arguments](#you-must-use-two-dashes----to-separate-your-commands-flagsarguments)
- [Also note, do not surround your command and its flags/arguments with quotes](#also-note-do-not-surround-your-command-and-its-flagsarguments-with-quotes)
- [unless that is how you would execute it normally (i.e., do ls -t /usr, not "ls -t /usr")](#unless-that-is-how-you-would-execute-it-normally-ie-do-ls--t-usr-not-ls--t-usr)
- [Get output from running 'date' command from the first pod of the deployment mydeployment, using the first container by default](#get-output-from-running-date-command-from-the-first-pod-of-the-deployment-mydeployment-using-the-first-container-by-default)
- [Get output from running 'date' command from the first pod of the service myservice, using the first container by default](#get-output-from-running-date-command-from-the-first-pod-of-the-service-myservice-using-the-first-container-by-default)
  - [Options](#options)
  - [Parent Options Inherited](#parent-options-inherited)
  - [Feedback](#feedback)

---