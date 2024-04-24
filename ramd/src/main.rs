use std::sync::Arc;
use std::thread::park;

use ramd_config::config::RamdConfig;
use ramd_db::rocks::RocksStorage;
use ramd_jsonrpc_server::launch;
use ramd_node::Node;
use ramd_tracing::init as init_tracing;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    // Init or read ramd config
    let ramd_config = RamdConfig::init_or_read()?;

    // Init tracing logger
    init_tracing(&ramd_config.tracing);

    tracing::info!("Topology is a community-driven technology that brings random access memory to the world computer to power lock-free asynchronous decentralized applications.");

    // Construct RocksDB
    let rocks = Arc::new(RocksStorage::new(&ramd_config.rocks)?);

    // Construct a RAM node
    let node = Arc::new(Node::new(&ramd_config.node, rocks.clone())?);

    // Launch jsonrpc server
    let handle = launch(&ramd_config.json_rpc, node.clone()).await?;

    // TODO: for now we don't care about server, simply start it and forget
    // Revisit once proper server handle handling will be required
    tokio::spawn(handle.stopped());

    // Launch P2P server
    // TODO: create P2P server

    // TODO: implement proper process handler, for now simply park the main thread until ctrl+c
    park();

    Ok(())
}