# kubectl-dask-gateway
⚠️ This project is a _Work-In-Progress Experiment + Proof-Of-Concept!_ ⚠️
A CLI to manage Dask-Gateway Clusters in Kubernetes.

## Overview
Sometimes dask-gateway clusters fail to shutdown. For example the user's jupyter 
notebook kernel fails, leaving the cluster reference lost in the notebook runtime 
context, while the cluster pods are still present and consuming resources. 

### Goals/Roadmap

- [ ] Provide an _admin_ view into the running dask clusters in kubernetes
- [ ] Ability to select and delete dask clusters that failed to terminate properly
- [ ] Provide an audit-log/manifest record (user signature + verification?) of dask cluster creation


