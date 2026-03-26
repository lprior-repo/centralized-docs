---
doc_id: ops/docs-setup-production-environment-tools-kubeadm-high-availability.md/docs-setup-production-environment-tools-kubeadm-high-availability
chunk_id: ops/docs-setup-production-environment-tools-kubeadm-high-availability.md/docs-setup-production-environment-tools-kubeadm-high-availability#60-summary
chunk_level: summary
chunk_type: prose
heading: Manual certificate distribution
token_count: 108
summary: #### Caution: Copy only the certificates in the above list. kubeadm will take care of generating the rest of the certificates with the required SANs for the joining control-plane instances. If you...
---

#### Caution:
Copy only the certificates in the above list. kubeadm will take care of generating the rest of the certificates
with the required SANs for the joining control-plane instances. If you copy all the certificates by mistake,
the creation of additional nodes could fail due to a lack of required SANs.
* Then on each joining control plane node you have to run the following script before running `kubeadm join`.
This script will move the previously copied certificates from the home directory to `/etc/kubernetes/pki`: