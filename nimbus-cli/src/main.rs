use nimbus_cli::{
    Result,
    args::{ArgRunner, NimbusArg},
};

fn main() -> Result<()> {
    let arg: NimbusArg = argh::from_env();
    arg.run()
}
