mod args;
mod io;
mod ops;
mod options;
mod output;

pub fn run() -> anyhow::Result<()> {
    options::run()
}
