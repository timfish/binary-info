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