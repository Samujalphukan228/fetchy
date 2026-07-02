# Fetchy

A fast system info tool for your terminal, written in Rust.

![fetchy screenshot](https://res.cloudinary.com/dzulab559/image/upload/v1782482436/fetchy_1_wt49uy.png)

## Install

```bash
curl -sSf https://raw.githubusercontent.com/Samujalphukan228/fetchy/master/install.sh | sh
source ~/.bashrc   # or ~/.zshrc
```

Then run `systeminfo`.

## Usage

```bash
systeminfo                   # full output
systeminfo --compact         # shorter view
systeminfo --no-logo         # hide logo
systeminfo --no-colors       # plain text
systeminfo --logo arch       # force a logo
systeminfo --list-logos      # list available logos
systeminfo --json            # JSON output
systeminfo --init-config     # create config file
```

## Configuration

```bash
systeminfo --init-config
```

Creates `~/.config/systeminfo/config.toml`.

## Uninstall

```bash
curl -sSf https://raw.githubusercontent.com/Samujalphukan228/fetchy/master/install.sh | sh -s -- --uninstall
```

## License

MIT — see [LICENSE](LICENSE)