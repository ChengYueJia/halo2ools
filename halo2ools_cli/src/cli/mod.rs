extern crate clap;
use clap::{command, Parser};

#[derive(Parser, Debug)]
pub enum CliCommand {
    #[command(name = "gen_param")]
    GenParam(GenParamOpt),
    #[command(name = "downsize_param")]
    DownsizeParam(DownsizeParamOpt),
}

#[derive(Parser, Debug)]
pub struct GenParamOpt {
    #[arg(long = "k", required = true)]
    pub(crate) degree: u32,
    #[arg(long = "file", required = true)]
    pub(crate) param_file: String,
}

#[derive(Parser, Debug)]
pub struct DownsizeParamOpt {
    /// target degree
    #[arg(long = "k", required = true)]
    pub(crate) degree: u32,
    /// src_param file
    #[arg(long = "src", required = true)]
    pub(crate) src_param: String,
    /// target_param file
    #[arg(long = "target", required = true)]
    pub(crate) target_param: String,
}
