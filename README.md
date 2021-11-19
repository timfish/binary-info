# `binary-info`

Native node module to get the platform and architecture of a binary

```ts
const binary = require('binary-info');

binary.getInfo('./path/to/some.node');
// {
//    platform: "darwin",
//    arch: "arm64"
// }
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
