use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Metaboss",
    about = "Metaplex NFT-standard Swiss army knife tool."
)]
pub struct Opt {
    #[structopt(short, long, default_value = "https://api.devnet.solana.com")]
    pub rpc: String,

    #[structopt(subcommand)]
    pub cmd: Command,
}

#[derive(Debug, StructOpt)]
pub enum Command {
    #[structopt(name = "decode")]
    Decode {
        /// List of mint accounts to decode.
        #[structopt(short, long)]
        mint_accounts: Vec<String>,

        #[structopt(short, long, default_value = ".")]
        /// Path to directory to save output files.
        output: String,
    },
    /// Change an NFT's URI to point to a new metadata JSON file.
    #[structopt(name = "update_metadata_uri")]
    UpdateMetadataUri {
        #[structopt(short, long)]
        keypair: String,

        /// Mint account to update.
        #[structopt(short, long)]
        mint_account: String,

        #[structopt(short, long)]
        new_uri: String,
    },
    /// Change an NFT's URI to point to a new metadata JSON file.
    #[structopt(name = "update_metadata_uri_all")]
    UpdateMetadataUriAll {
        #[structopt(short, long)]
        keypair: String,

        /// Path to file containing list of mint accounts and their new URIs.
        #[structopt(short, long)]
        mint_accounts: String,
    },
}
