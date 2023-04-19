use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct SacnWs281xAdapterArgs {
    #[arg(short, long)]
    pub pixel_count: usize,
    #[arg(short = 'u', long)]
    pub universes: Option<Vec<u8>>,
}
