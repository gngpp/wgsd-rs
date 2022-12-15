

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    const PEER: &str = "peer";
    const PEER_SERVER: &str = "peer-server";
    let node_type_option = vec![PEER, PEER_SERVER];
    let node_type_select = inquire::Select::new("Select the peer node type that needs to be revoked.", node_type_option).prompt();
    match node_type_select {
        Ok(node_type) => {
            match node_type {
                PEER => {

                    let _configuration = Configuration::new("test".to_string()).await?;
                    println!("await")
                }
                PEER_SERVER => {

                }
                _ => {}
            }
        }
        Err(_) => {}
    }
    Ok(())
}
