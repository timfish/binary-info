# `binary-info`

Native node module to get the platform and architecture of a binary

```ts
const binary = require('binary-info');

// getInfo will throw an error if the file does not appear to be
// a binary or is an unknown architecture
binary.getInfo('./path/to/some.node');
// {
//    platform: "darwin",
//    arch: "arm64"
// }

// isCompatible will return true if the file is a binary
// and matches the supplied platform and architecture
binary.isCompatible('./path/to/some.node', { platform: 'darwin', arch: 'x64' });
// false
binary.isCompatible('./non-binary.js', { platform: 'darwin', arch: 'x64' });
// false

// isIncompatible will return true if the file is a binary
// but doesn't match the supplied platform and architecture
binary.isIncompatible('./path/to/some.node', { platform: 'darwin', arch: 'x64' });
// true
binary.isIncompatible('./non-binary.js', { platform: 'darwin', arch: 'x64' });
// false
```

| Platform | Arch    | Binary Included | Detected | CI Tested |
| -------- | ------- | --------------- | -------- | --------- |
| `win32`  | `x64`   | ✔️              | ✔️       | ✔️        |
| `win32`  | `ia32`  | ✔️              | ✔️       | ✔️        |
| `win32`  | `arm64` | ✔️              | ✔️       |           |
| `darwin` | `x64`   | ✔️              | ✔️       | ✔️        |
| `darwin` | `arm64` | ✔️              | ✔️       |           |
| `linux`  | `x64`   | ✔️              | ✔️       | ✔️        |
| `linux`  | `arm64` | ✔️              | ✔️       |           |
| `linux`  | `arm`   | ✔️              | ✔️       |           |

## All the hard work by:

- [`napi-rs`](https://github.com/napi-rs/napi-rs) - Rust to Node-API bindings
- [`goblin`](https://github.com/m4b/goblin) - Rust library to parse binaries
