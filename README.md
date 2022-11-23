# Ice
[Ice](https://gameofthrones.fandom.com/wiki/Ice) is a lightweight command line utility to help with simple problems encountered while playing CTFs. <br />
Extracted from [graveyard](https://github.com/tamton-aquib/graveyard)

NOTE: Most of the functions can autodetect if its encode or decode.

---

### Includes
- Base: binary, octal, hex, base32, base64
- General ciphers: a1z23, atbash, etc
- Caesar ciphers: simple rots, rot13, vigenere, etc
- Xor: single byte, string vs string, hex vs hex, etc. (wip)
- check [todo.norg](https://github.com/tamton-aquib/ice/blob/main/todo.norg) for the full list working on.

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

### Todo:
moved to [todo.norg](https://github.com/tamton-aquib/ice/blob/main/todo.norg)
