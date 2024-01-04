//! Rage Bootstrap
//!
//! Entry point for the bootstrap build system.

use std::{path::PathBuf, time::SystemTime};

//use rage_bootstrap::{compiler::Compiler, interpreter::InstructionTree};
macro_rules! rg_func {
    (
        $(#[$meta:meta])*
        $vis:vis $fn_label:ident ( $( $arg_label:ident $arg_type:ty ),* $(,)? ) $($ret_type:ty)? { $($tt:tt)* }
    ) => {
        $( #[$meta] )*
        $vis fn $fn_label( $($arg_label : $arg_type),* ) $(-> $ret_type)? { $($tt)* }
    };
}
#[async_std::main]
async fn main() -> anyhow::Result<()> {
    /* let mut logger = env_logger::builder();
    #[cfg(debug_assertions)]
    logger.filter_level(log::LevelFilter::Debug).init();
    #[cfg(not(debug_assertions))]
    logger.filter_level(log::LevelFilter::Info).init();

    let start_time = SystemTime::now();

    //let root_path: PathBuf = "./examples/".into();
    let root_path: PathBuf = "./".into();

    let mut compiler = Compiler::new(root_path)?;
    let instruction_tree = compiler.run().await?;

    log::info!(
        "compiled in {} seconds",
        start_time.elapsed()?.as_secs_f64()
    );*/
    rg_func!(pub rage_add(a u32, b u32) u32 { return a + b; });
    println!("{}", rage_add(4, 5));

    Ok(())
}


#[test]
fn rg_func() {
    rg_func!(pub rage_add(a u32, b u32) u32 { return a + b; });
    pub fn rust_add(a: u32, b: u32) -> u32 { return a + b; }
    assert!(rage_add(1, 2) == rust_add(1, 2))
}

