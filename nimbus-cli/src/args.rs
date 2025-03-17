use std::path::PathBuf;

use argh::FromArgs;
use nimbus_node::{node_start, node_stop};

use crate::Result;

#[derive(Debug, FromArgs, PartialEq, Clone)]
#[argh(
    description = "nimbus: command-line tool for managing, monitoring, and operating a nimbus cluster."
)]
pub struct NimbusArg {
    #[argh(subcommand)]
    pub subcommand: NimbusSubCommand,
}

#[derive(Debug, FromArgs, PartialEq, Clone)]
#[argh(subcommand)]
pub enum NimbusSubCommand {
    Node(NodeArg),
}

#[derive(Debug, FromArgs, PartialEq, Clone)]
#[argh(subcommand, name = "node", description = "manage nimbus node")]
pub struct NodeArg {
    #[argh(subcommand)]
    pub node_subcommand: NodeSubCommand,
}

#[derive(Debug, FromArgs, PartialEq, Clone)]
#[argh(subcommand)]
pub enum NodeSubCommand {
    Start(NodeStartArg),
    Stop(NodeStopArg),
}

#[derive(Debug, FromArgs, PartialEq, Clone)]
#[argh(subcommand, name = "start", description = "start nimbus node")]
pub struct NodeStartArg {
    #[argh(option, short = 'c', description = "nimbus cluster configuration path")]
    pub config: PathBuf,
    #[argh(option, short = 'n', description = "nimbus node name")]
    pub node: u64,
}

#[derive(Debug, FromArgs, PartialEq, Clone)]
#[argh(subcommand, name = "stop", description = "")]
pub struct NodeStopArg {
    #[argh(option, short = 'c', description = "nimbus cluster configuration path")]
    pub config: PathBuf,
    #[argh(option, short = 'n', description = "nimbus node name")]
    pub node: u64,
}

pub trait ArgRunner {
    fn run(&self) -> Result<()>;
}

impl ArgRunner for NimbusArg {
    fn run(&self) -> Result<()> {
        self.subcommand.run()
    }
}

impl ArgRunner for NimbusSubCommand {
    fn run(&self) -> Result<()> {
        match self {
            Self::Node(node) => node.run(),
        }
    }
}

impl ArgRunner for NodeArg {
    fn run(&self) -> Result<()> {
        self.node_subcommand.run()
    }
}

impl ArgRunner for NodeSubCommand {
    fn run(&self) -> Result<()> {
        match self {
            Self::Start(start_arg) => start_arg.run(),
            Self::Stop(stop_arg) => stop_arg.run(),
        }
    }
}

impl ArgRunner for NodeStartArg {
    fn run(&self) -> Result<()> {
        Ok(node_start(&self.config, self.node)?)
    }
}

impl ArgRunner for NodeStopArg {
    fn run(&self) -> Result<()> {
        Ok(node_stop(&self.config, self.node)?)
    }
}
