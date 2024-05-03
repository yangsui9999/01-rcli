use std::fs;

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use clap::Parser;
use rcli::{
    get_content, get_reader, process_csv, process_decode, process_encode, process_genpass,
    process_key_generate, process_text_sign, process_text_verify, Base64SubCommand, Opts,
    SubCommand, TextSubCommand,
};
use zxcvbn::zxcvbn;
fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output.clone()
            } else {
                format!("output.{}", opts.format)
            };
            process_csv(&opts.input, output, opts.format)?
        }
        SubCommand::GenPass(opts) => {
            let ret = process_genpass(
                opts.length,
                opts.uppercase,
                opts.lowercase,
                opts.numbers,
                opts.symbol,
            )?;

            println!("{}", ret);

            let estimate = zxcvbn(&ret, &[])?;
            eprintln!("Password strength: {}", estimate.score());
        }
        SubCommand::Base64(subcmd) => match subcmd {
            Base64SubCommand::Encode(opts) => {
                println!("Encode: {:?}", opts);
                process_encode(&opts.input, opts.format)?
            }
            Base64SubCommand::Decode(opts) => {
                println!("Decode: {:?}", opts);
                process_decode(&opts.input, opts.format)?
            }
        },
        SubCommand::Text(subcmd) => match subcmd {
            TextSubCommand::Sign(opts) => {
                println!("Sign: {:?}", opts);
                let mut reader = get_reader(&opts.input)?;
                let key = get_content(&opts.key)?;
                let sign = process_text_sign(&mut reader, &key, opts.format)?;

                // base64 output
                let encode = URL_SAFE_NO_PAD.encode(sign);
                println!("{}", encode);
            }
            TextSubCommand::Verify(opts) => {
                println!("Verify: {:?}", opts);
                let mut reader = get_reader(&opts.input)?;
                let key = get_content(&opts.key)?;
                let decoded = URL_SAFE_NO_PAD.decode(&opts.sig)?;
                let verified = process_text_verify(&mut reader, &key, &decoded, opts.format)?;

                if verified {
                    println!("✓ Signature verified");
                } else {
                    println!("⚠ Signature not verified");
                }
            }
            TextSubCommand::Generate(opts) => {
                println!("Generate: {:?}", opts);
                let key = process_key_generate(opts.format)?;
                for (k, v) in key {
                    fs::write(opts.output_path.join(k), v)?;
                }
            }
        },
    }

    Ok(())
}
