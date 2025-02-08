# Filturd

This is a loot filter app for Path of Exile 2.

## Development

The app code uses Rust-Tauri backend and TypeScript-Vue frontend.

### Clone the repository

```sh
git clone https://github.com/evgenyneu/filturd.git
```

### Install prerequisites

* Node.js with [NVM for Windows](https://github.com/coreybutler/nvm-windows) or [nvm](https://github.com/nvm-sh/nvm) for Linux/Mac. Node version is specified in [`.nvmrc`](.nvmrc).
* [Rust](https://www.rust-lang.org/)
* Tauri 2 [prerequisites](https://v2.tauri.app/start/prerequisites/).

### Install dependencies

```sh
npm install
```

### Run in development

```sh
npm run tauri dev
```

### Build

```sh
npm run tauri build
```


### Run tests

```sh
npm run test_rust
```
