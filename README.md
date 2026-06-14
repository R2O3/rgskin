# rgskin

[![crates.io](https://img.shields.io/crates/v/rgskin?style=flat-square&logo=rust&logoColor=white&color=e05d2c)](https://crates.io/crates/rgskin)
[![npm (browser)](https://img.shields.io/npm/v/%40r2o3%2Frgskin-browser?style=flat-square&logo=npm&logoColor=white&label=npm%20(browser)&color=CB3837)](https://www.npmjs.com/package/@r2o3/rgskin-browser)
[![npm (node)](https://img.shields.io/npm/v/%40r2o3%2Frgskin-nodejs?style=flat-square&logo=npm&logoColor=white&label=npm%20(node)&color=CB3837)](https://www.npmjs.com/package/@r2o3/rgskin-nodejs)

[![license](https://img.shields.io/badge/license-MIT-4A80D4?style=flat-square)](LICENSE)
[![rust](https://img.shields.io/badge/rust-nightly%201.98%2B-534eb5?style=flat-square&logo=rust&logoColor=white)](https://www.rust-lang.org/tools/install)

A library for loading and creating skins for various rhythm games. It supports multi-threading and cross-platform usage including Web and Node.js environments via WebAssembly (WASM).

## Table of Contents

- [Features](#features)
    - [Supported Games](#supported-games) 
- [Rust Usage](#rust-usage)
    - [Installation](#installation)
    - [API Reference](#api-reference)
        - [Importing/Loading Skins](#importingloading-skins)
        - [Creating Skins](#creating-skins)
        - [Exporting Skins](#exporting-skins)
- [JavaScript/TypeScript Usage](#javascripttypescript-usage)
    - [Installation](#installation-1)
    - [API Reference](#api-reference-1)
        - [Initialization](#initialization)
        - [Importing/Loading Skins](#importingloading-skins-1)
        - [Creating Skins](#creating-skins-1)
        - [Exporting Skins](#exporting-skins-1)
- [Building](#building)
    - [Rust Library](#rust-library)
    - [WASM Bindings](#wasm-bindings)
- [License](#license)

## Features

### Supported Games:
- Osu!
- Quaver
- fluXis

## Rust Usage

### Installation
Add this to your `Cargo.toml`:
```toml
[dependencies]
rgskin = "0.0.7"
```

Or run:
```sh
cargo add rgskin
```

### API Reference

#### Importing/Loading Skins

##### Recommended way of Loading a skin

```rust
use rgskin::prelude::*;

// importing a skin from a directory
let osu_skin = import::osu::skin_from_dir("path/to/skin").expect("Failed to import skin!", false);
let fluxis_skin = import::fluxis::skin_from_dir("path/to/skin").expect("Failed to import skin!", false);
let quaver_skin = import::quaver::skin_from_dir("path/to/skin").expect("Failed to import skin!", false);
```

The second argument is for if you want to import ALL assets for a skin, it will import all textures but leave the unrequired unloaded, usually recommended when merging skins of the same type as you might need all assets; otherwise if false it will only import and load the required assets.

##### Manually loading a skin

```rust
use rgskin::prelude::*;

// create a new texture store, this is where the actual textures will be stored
let mut textures = TextureStore::new();

// you can import textures from a directory like this:
textures = import::all_textures_from_dir("path/to/skin", None)?;

// or alternatively if you parse the skin config first, you can import only the textures you need like this:
let raw_str = import::osu::ini_str_from_dir("path/to/skin");
let skin_config = OsuSkinIni::from_str(&raw_str)?;

// since get_required_texture_paths returns a HashSet, we need to convert it to a Vec<&str> for the import function
let required_texture_paths_set = skin_config.get_required_texture_paths();
let required_texture_paths = required_texture_paths_set.iter().map(|s| s.as_str()).collect::<Vec<_>>();

textures = import::textures_from_dir("path/to/skin", &required_texture_paths)?;

// now you can create a skin from the config and textures

let osu_skin = OsuSkin::new(skin_config, Some(textures), None); // last parameter is the sound samples store, which you can import similarly to textures
```

#### Creating Skins

All skins are loaded in their original formats; Any textures go in ``TextureStore`` or samples go in ``SampleStore``, etc. The config is also preserved so, this next part will talk about dealing with generic skins as creating skins for a specific game differs from one to another.

Additionally all skins can be converting into a generic version of it.

Examples:
```rust
OsuSkin::from_generic_mania(&generic); 
OsuSkin.to_generic_mania(()); // yes, the extra parethesis is required.
```
```rust
FluXisSkin::from_generic_mania(&generic); 
FluXisSkin.to_generic_mania(fluxis_layout); // if you don't have a layout you can just pass None.
```

---

Unlike skins from games not all textures are stored in ``TextureStore``. Skin Elements can have their own textures that are shared pointers (``Option<Arc<RwLock<Texture>>>``). Meaning the texture can be shared anywhere either in a ``TextureStore`` or inside a Skin Element.

#### Exporting Skins

##### Recommened way of exporting a skin

```rust
export::osu::skin_to_dir(&skin, "path/to/export/to")?;
export::fluxis::skin_to_dir(&skin, "path/to/export/to")?;
```

###### Manually exporting a skin
```rust
export::osu::ini_to_dir(skin.skin_ini, "path/to/export/to"); // export::{game}
export::textures_to_dir(skin.textures, "path/to/export/to");
export::samples_to_dir(skin.samples, "path/to/export/to"); // if you have samples
```

## JavaScript/TypeScript Usage

### Installation
For Node.js:
```sh
npm install @r2o3/rgskin-nodejs
```

For web projects:
```html
<script src="https://unpkg.com/@r2o3/rgskin-browser@latest/rgskin.js"></script>
```
or
```javascript
npm install @r2o3/rgskin-browser
```
then use as an ES module

### API Reference

#### Initialization


As of now you can't parse/write using the original structures in JS/TS, will be supported in the *near* future.

> [!IMPORTANT]
> `wasm-bindgen-rayon` uses `SharedArrayBuffer` for thread communication, which requires your page to be cross-origin isolated. Your server must send these headers:
> ```
> Cross-Origin-Opener-Policy: same-origin
> Cross-Origin-Embedder-Policy: require-corp
> ```
> You can verify isolation is active with `self.crossOriginIsolated` in the browser console.

---

**ES Modules (bundler or Node.js)**

```javascript
import init, { initThreadPool } from '@r2o3/rgskin';
// or if not installed via npm:
// import init, { initThreadPool } from './path/to/rgskin.js';

await init();
await initThreadPool(navigator.hardwareConcurrency);
```

---

**Browser (`<script type="module">`)**

```javascript
import rgskin from './path/to/rgskin.js';

await rgskin.default();
await rgskin.initThreadPool(navigator.hardwareConcurrency);
```

---

**CommonJS (Node.js or Legacy Environments)**

CommonJS (`require`) is not natively supported because `wasm-bindgen-rayon` relies on ES Modules and top-level async initialization. However, you can load it using `import()`:

```javascript
async function loadWasm() {
    const wasm = await import('@r2o3/rgskin');
    
    await wasm.default();
    await wasm.initThreadPool(navigator.hardwareConcurrency);
    
    return wasm;
}
```

#### Importing/Loading Skins

For Node:

##### Recommended way of Loading a skin

```javascript
const OsuSkin = rgskin.osuSkinFromDir("path/to/skin");
const FluXisSkin = rgskin.fluXisSkinFromDir("path/to/skin");
```

##### Manually loading a skin

```javascript
// create a new texture store, this is where the actual textures will be stored
let textures = new rgskin.TextureStore();

// you can import textures from a directory like this:
textures = rgskin.allTexturesFromDir("path/to/skin");

// you can parse configs like this:
let raw_str = rgskin.iniStrFromDir("path/to/skin")
let skin_config = rgskin.OsuSkinIni.fromStr(raw_str);

let osuSKin = new rgskin.OsuSkin(skin_config, textures, null) // last parameter is the sound samples store, which you can import similarly to textures
```

For Browsers:

Unfortunately you can't automatically import everything using a single function.
So you'll have to do a bit of work. Check [FilesMap preparation](#preparing-the-filesmap)

##### Recommended way of Loading a skin

```javascript
const OsuSkin = rgskin.osuSkinFromFiles(filesMap);
const FluXisSkin = rgskin.fluXisSkinFromFiles(filesMap);
```

##### Manually loading a skin

```javascript
// create a new texture store, this is where the actual textures will be stored
let textures = new rgskin.TextureStore();

// you can import textures from files like this:
// filesMap is a Map object with relative path -> Uint8Array pairs
textures = rgskin.allTexturesFromFiles(filesMap);

// you can parse configs like this:
// assuming you have the skin.ini file in your filesMap
let raw_str = filesMap.get("skin.ini"); // get the Uint8Array
let decoder = new TextDecoder();
let ini_string = decoder.decode(raw_str);
let skin_config = rgskin.OsuSkinIni.fromStr(ini_string);

let osuSkin = new rgskin.OsuSkin(skin_config, textures, null); // last parameter is the sound samples store, which you can import similarly to textures
```

##### Preparing the filesMap

The `filesMap` parameter is a `Map` object where:
- Keys are relative paths (strings)
- Values are file contents as `Uint8Array`

Example: Reading files from an HTML file input

```javascript
const filesMap = new Map();

// assuming you have an <input type="file" multiple webkitdirectory> element
fileInput.addEventListener('change', async (event) => {
    const files = event.target.files;
    
    for (const file of files) {
        const arrayBuffer = await file.arrayBuffer();
        const uint8Array = new Uint8Array(arrayBuffer);
        const relativePath = file.webkitRelativePath || file.name;
        filesMap.set(relativePath, uint8Array);
    }
    
    // now you can import the skin
    const skin = rgskin.osuSkinFromFiles(filesMap);
});
```

#### Creating Skins

Check [Rust's Creating Skins](#creating-skins) for more details

```javascript
OsuSkin.fromGenericMania(&generic); 
OsuSkin.toGenericMania();
```
```javascript
FluXisSkin.fromGenericMania(&generic); 
FluXisSkin.toGenericMania(fluxis_layout); // if you don't have a layout you can just not pass anything or null.
```

#### Exporting Skins

For Node:

##### Recommended way of exporting a skin

```javascript
rgskin.osuSkinToDir(skin, "path/to/export/to");
rgskin.fluXisSkinToDir(skin, "path/to/export/to");
```

##### Manually exporting a skin

```javascript
rgskin.iniToDir(skin.skin_ini, "path/to/export/to");
rgskin.texturesToDir(skin.textures, "path/to/export/to");
rgskin.samplesToDir(skin.samples, "path/to/export/to"); // if you have samples
```

For Browsers:

##### Recommended way of exporting a skin

```javascript
// Returns a JavaScript Map object with relative path -> Uint8Array pairs
const filesMap = rgskin.osuSkinToFiles(skin);
const filesMap = rgskin.fluXisSkinToFiles(skin);
```

##### Manually exporting a skin

```javascript
const iniString = rgskin.iniToString(skin.skin_ini);
const texturesMap = rgskin.texturesToFiles(skin.textures);
const samplesMap = rgskin.samplesToFiles(skin.samples);

// combine them into a single Map if needed
const filesMap = new Map([
    ['skin.ini', new TextEncoder().encode(iniString)],
    ...texturesMap,
    ...samplesMap
]);
```

Actually Exporting/Downloading the files will depend on your implementation.

Example using JSZip:
```javascript
const filesMap = rgskin.osuSkinToFiles(skin);

const zip = new JSZip();
filesMap.forEach((data, path) => {
    zip.file(path, data);
});

const zipBlob = await zip.generateAsync({ type: 'blob' });
const link = document.createElement('a');
link.href = URL.createObjectURL(zipBlob);
link.download = 'skin.zip';
link.click();
```

Example Downloading each file indiviually:

```javascript
const filesMap = rgskin.osuSkinToFiles(skin);

filesMap.forEach((data, path) => {
    const blob = new Blob([data]);
    const link = document.createElement('a');
    link.href = URL.createObjectURL(blob);
    link.download = path;
    link.click();
});
```

## Building

### Rust Library
```sh
cargo build
```

### WASM Bindings
1. Install wasm-pack:
```sh
cargo install wasm-pack
```
> [!IMPORTANT]  
> It's really recommended to have [wasm-opt](https://github.com/WebAssembly/binaryen) installed and added to path for the wasm build.

2. Build the package:
```sh
npm run build # debug build
npm run build-release # release build
```

3. This will build it for both node and browser and the output will be in `dist-web` and `dist-node` directory.

## License
r2o3 uses the MIT License for all its sibiling projects.
See [LICENSE](https://github.com/r2o3/rgskin/blob/master/LICENSE) for more information