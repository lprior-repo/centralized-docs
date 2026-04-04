---
doc_id: concept/docs-tasks-tools-install-kubectl-macos.md/docs-tasks-tools-install-kubectl-macos
chunk_id: concept/docs-tasks-tools-install-kubectl-macos.md/docs-tasks-tools-install-kubectl-macos#1-detailed
chunk_level: detailed
chunk_type: code
heading: Install kubectl on macOS
token_count: 962
summary: ## Install kubectl on macOS The following methods exist for installing kubectl on macOS: * [Install kubectl on macOS](#install-kubectl-on-macos) * [Install kubectl binary with curl on...
---

## Install kubectl on macOS
The following methods exist for installing kubectl on macOS:
* [Install kubectl on macOS](#install-kubectl-on-macos)
* [Install kubectl binary with curl on macOS](#install-kubectl-binary-with-curl-on-macos)
* [Install with Homebrew on macOS](#install-with-homebrew-on-macos)
* [Install with Macports on macOS](#install-with-macports-on-macos)
* [Verify kubectl configuration](#verify-kubectl-configuration)
* [Optional kubectl configurations and plugins](#optional-kubectl-configurations-and-plugins)
* [Enable shell autocompletion](#enable-shell-autocompletion)
* [Install `kubectl convert` plugin](#install-kubectl-convert-plugin)### Install kubectl binary with curl on macOS
1. Download the latest release:
```
`
curl -LO "https://dl.k8s.io/release/$(curl -L -s https://dl.k8s.io/release/stable.txt)/bin/darwin/amd64/kubectl"
`
```
```
`
curl -LO "https://dl.k8s.io/release/$(curl -L -s https://dl.k8s.io/release/stable.txt)/bin/darwin/arm64/kubectl"
`
```
#### Note:
To download a specific version, replace the `$(curl -L -s https://dl.k8s.io/release/stable.txt)`
portion of the command with the specific version.
For example, to download version 1.35.0 on Intel macOS, type:
```
`curl -LO "https://dl.k8s.io/release/v1.35.0/bin/darwin/amd64/kubectl"
`
```
And for macOS on Apple Silicon, type:
```
`curl -LO "https://dl.k8s.io/release/v1.35.0/bin/darwin/arm64/kubectl"
`
```
2. Validate the binary (optional)
Download the kubectl checksum file:
```
`
curl -LO "https://dl.k8s.io/release/$(curl -L -s https://dl.k8s.io/release/stable.txt)/bin/darwin/amd64/kubectl.sha256"
`
```
```
`
curl -LO "https://dl.k8s.io/release/$(curl -L -s https://dl.k8s.io/release/stable.txt)/bin/darwin/arm64/kubectl.sha256"
`
```
Validate the kubectl binary against the checksum file:
```
`echo "$(cat kubectl.sha256) kubectl" | shasum -a 256 --check
`
```
If valid, the output is:
```
`kubectl: OK
`
```
If the check fails, `shasum` exits with nonzero status and prints output similar to:
```
`kubectl: FAILED
shasum: WARNING: 1 computed checksum did NOT match
`
```
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