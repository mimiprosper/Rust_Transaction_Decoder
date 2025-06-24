use clap::Parser;

#[derive(Parser)]
#[command(name = "Bitcoin Transaction Decoder")]
#[command(version = "1.0")]
#[command(about = "Decode a Bitcoin transaction", long_about = None)]

struct Cli {
    #[arg(
        required = true,
        help = "The hex-encoded Bitcoin transaction to decode"
    )]
    transaction_hex: String,
}

fn main() {
    // let transaction_hex = "0200000000010260f95338027d4763cfb1f58cd66f6819579d9daf5413ed3cd6a6ddf8079f08c30000000000ffffffffaa4ea40e113a6c6416b1009ee5f8475a6da66ab9cee6482bead647d15b6dc0c30000000000ffffffff015de70200000000001976a914c116837aa60d50114777a6fc07d452470d03aca788ac024830450221008d7c9498bd12316e20a4571464e7ca79322c96e40c0fb2984390ebb2ec26051102206bb81aa3af0495eb390c429a682861974e496ce3f3db6e6a9d182ba4c63e823b012102718cc0ff55d879ad96623f59ea140290d6cedbd4a107f1a247862bac883cbe7202483045022100b1b3f9244495965d84d71d3a1455ebcec87e353ea524c824e4136cba76dd427a0220606671904618d7facb160838c138d50635a2896c028f98312adbade275f75630012102718cc0ff55d879ad96623f59ea140290d6cedbd4a107f1a247862bac883cbe7200000000";
    let cli = Cli::parse();
    match transaction_decoder::decode(cli.transaction_hex) {
        Ok(transaction_json) => println!("{}", transaction_json),
        Err(e) => println!("Error decoding transaction: {}", e),
    }
}