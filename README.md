# kubectl-dask-gateway
⚠️ This project is a _Work-In-Progress Experiment + Proof-Of-Concept!_ ⚠️
A CLI to manage Dask-Gateway Clusters in Kubernetes.

## Overview
Sometimes dask-gateway clusters fail to shutdown. For example the user's jupyter 
notebook kernel fails, leaving the cluster reference lost in the notebook runtime 
context, while the cluster pods are still present and consuming resources. 

It would be nice to have a SLSA-compliant attestation or record to audit who created a given cluster,
with a timestamp, resources requested, or anything else useful...

### Goals/Roadmap

> I want to have a single pane-of-glass view into the state of running dask clusters in K8S
> with the ability to see a digital signature with timestamp of who created/provisioned 
> a given dask cluster, and the ability to selectively delete dask clusters as needed

- [ ] Provide an _admin_ view into the running dask clusters in kubernetes
- [ ] Ability to select and delete dask clusters that failed to terminate properly
- [ ] Provide an audit-log/manifest record (user signature + verification?) of dask cluster creation
- [ ] In-cluster controller to manage gateway cluster metadata and resource lifecycle 


