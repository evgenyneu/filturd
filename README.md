# Filturd

This is a loot filter app for Path of Exile 2.

## Development

The app code uses Rust-Tauri backend and TypeScript-Vue frontend.

### Install prerequisites

* Node.js with [NVM for Windows](https://github.com/coreybutler/nvm-windows) or [nvm](https://github.com/nvm-sh/nvm) for Linux/Mac. Node version is specified in [`.nvmrc`](.nvmrc).
* [Yarn](https://yarnpkg.com) with `corepack enable`.
* [Rust](https://www.rust-lang.org/)
* If using VS Code, install recommended extensions from [`.vscode/extensions.json`](.vscode/extensions.json).

### Clone the repository

```sh
git clone https://github.com/evgenyneu/filturd.git
```

### Install dependencies

```sh
yarn install
```

### Run in development

```sh
yarn tauri dev
```

### Build

```sh
yarn tauri build
```
