# Fetchy

A fast system info tool for your terminal, written in Rust.

![fetchy screenshot](https://res.cloudinary.com/dzulab559/image/upload/v1782482436/fetchy_1_wt49uy.png)

## Install

```bash
curl -sSf https://raw.githubusercontent.com/Samujalphukan228/fetchy/master/install.sh | sh
source ~/.bashrc   # or ~/.zshrc
```

Then run `fetchy`.

## Usage

```bash
fetchy                   # full output
fetchy --compact         # shorter view
fetchy --no-logo         # hide logo
fetchy --no-colors       # plain text
fetchy --logo arch       # force a logo
fetchy --list-logos      # list available logos
fetchy --json            # JSON output
fetchy --init-config     # create config file
```

## Configuration

```bash
fetchy --init-config
```

Creates `~/.config/fetchy/config.toml`.

## Uninstall

```bash
curl -sSf https://raw.githubusercontent.com/Samujalphukan228/fetchy/master/install.sh | sh -s -- --uninstall
```

## License

MIT — see [LICENSE](LICENSE)