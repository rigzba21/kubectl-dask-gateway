//use kube_client::{Client, Api, ResourceExt};
use kube::{
    api::{Api, ListParams, ResourceExt},
    Client,
};
use k8s_openapi::api::core::v1::Pod;
use dialoguer::MultiSelect;
use std::env;


async fn list_pods(_client: Client, namespace: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Getting dask-scheduler Pods in Namespace: {}", namespace);
    let pods: Api<Pod> = Api::namespaced(_client, namespace);

    let mut dask_clusters: Vec<String> = Vec::new();

    let list_params = ListParams::default();
    for p in pods.list(&list_params).await? {
        if p.name().contains("dask-scheduler") {
            println!("Found dask-scheduler: {}", p.name());
            dask_clusters.push(p.name());
        }
    }

    if dask_clusters.is_empty() {
        println!("No Dask-Gateway Clusters Found....");
        return Ok(())
    }
    else {

        let chosen: Vec<usize> = MultiSelect::new()
            .with_prompt("Select Dask Cluster with <SPACEBAR>")
            .items(&dask_clusters)
            .interact()?;

        println!("{:#?}", chosen);

        return Ok(())

    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //create  cluster client using the default context or .kube/config
    // or the $KUBECONFIG environment variable
    let _client = Client::try_default().await?;
    
    //verify our kubeconfig
    let _apiserver_version = _client.apiserver_version().await.unwrap();
    println!("{:#?}", _apiserver_version);

    // get namespace
    let args: Vec<String> = env::args().collect();
    let _subcommand = &args[1];

    match _subcommand.as_str() {
    "list" => list_pods(_client, &args[2]).await?,
    _ => eprintln!("bad subcommand")
    }

    Ok(())
}
