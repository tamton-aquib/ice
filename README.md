# Ice
[Ice](https://gameofthrones.fandom.com/wiki/Ice) is a lightweight command line utility to help with simple problems encountered while playing CTFs. <br />
Extracted from [graveyard](https://github.com/tamton-aquib/graveyard)

NOTE: Most of the functions can autodetect if its encode or decode.

---

### Includes
- Base: binary, octal, hex, base32, base64
- Ciphers: a1z26, atbash, bacon, morse, railfence, playfair
- Caesar: rot13, caesar, vigenere
- Xor: single byte, string vs string, hex vs hex
- Hashing: md5, sha1, sha256, sha512
- URL: url encode/decode
- Formatting: lower, upper, reverse, length
- Extracting: emails, phone numbers, ipv4 addresses
- Utilities: factor database lookup, JWT decode
- check [todo.norg](https://github.com/tamton-aquib/ice/blob/main/todo.norg) for the full list

---

### Installation

##### From AUR
```sh
yay -S ice-bin
```

##### From source (if rust is installed in your system)
```bash
cargo install --git https://github.com/tamton-aquib/ice.git
```

#### From releases
- Go to [releases](https://github.com/tamton-aquib/ice/releases/) and download the appropriate file.
- extract the archive and you get a binary.
- `chmod +x ice` to make it executable.
- move it to one of the PATHS (usually ~/.local/bin/)

---

### Usage examples
```bash
ice b64 "example string"        # encodes the string
ice b64 ZXhhbXBsZSBzdHJpbmc=    # decodes the string (autodetect)

ice morse "example"
ice morse ". _.._ ._ __ .__. ._.. ."  # autodetect

# ... and the other 20 subcommands: checkout src/main.rs
```


### Todo:
moved to [todo.norg](https://github.com/tamton-aquib/ice/blob/main/todo.norg)
