---
doc_id: concept/docs-tasks-tools-install-kubectl-macos.md/docs-tasks-tools-install-kubectl-macos
chunk_id: concept/docs-tasks-tools-install-kubectl-macos.md/docs-tasks-tools-install-kubectl-macos#3-standard
chunk_level: standard
chunk_type: code
heading: Install kubectl on macOS
token_count: 350
summary: #### Note: Download the same version of the binary and checksum. 3. Make the kubectl binary executable. ``` `chmod +x ./kubectl ` ``` 4. Move the kubectl binary to a file location on your system...
---

#### Note:
Download the same version of the binary and checksum.
3. Make the kubectl binary executable.
```
`chmod +x ./kubectl
`
```
4. Move the kubectl binary to a file location on your system `PATH`.
```
`sudo mv ./kubectl /usr/local/bin/kubectl
sudo chown root: /usr/local/bin/kubectl
`
```
#### Note:
Make sure `/usr/local/bin` is in your PATH environment variable.
5. Test to ensure the version you installed is up-to-date:
```
`kubectl version --client
`
```
Or use this for detailed view of version:
```
`kubectl version --client --output=yaml
`
```
6. After installing and validating kubectl, delete the checksum file:
```
`rm kubectl.sha256
`
```
### Install with Homebrew on macOS
If you are on macOS and using [Homebrew](https://brew.sh/) package manager,
you can install kubectl with Homebrew.
1. Run the installation command:
```
`brew install kubectl
`
```
or
```
`brew install kubernetes-cli
`
```
2. Test to ensure the version you installed is up-to-date:
```
`kubectl version --client
`
```
### Install with Macports on macOS
If you are on macOS and using [Macports](https://macports.org/) package manager,
you can install kubectl with Macports.
1. Run the installation command:
```
`sudo port selfupdate
sudo port install kubectl
`
```
2. Test to ensure the version you installed is up-to-date:
```
`kubectl version --client
`
```