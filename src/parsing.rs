use crate::ApplicationOptions;
use clap::Parser;
use log::info;
use std::fmt::{Debug, Formatter};

impl Clone for ApplicationOptions {
    fn clone(&self) -> Self {
        ApplicationOptions {
            input: self.input.clone(),
            mode: self.mode.clone(),
            output: self.output.clone(),
        }
    }
}


impl Debug for ApplicationOptions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("")
            .field(&self.input)
            .field(&self.mode)
            .field(&self.output)
            .finish()
    }
}

pub fn arg_parse() -> ApplicationOptions {
    env_logger::init();

    let cli = ApplicationOptions::parse();

    info!("Parsed following arguments: ");
    info!("input: {:?}", &cli.input);
    info!("mode: {:?}", &cli.mode);
    info!("output: {:?}", &cli.output);
    cli
}
