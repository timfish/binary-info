const binary = require('./index');
const assert = require('assert');
const { existsSync } = require('fs');
const { join } = require('path');

const values = [
  ['binary-info.win32-x64-msvc.node', 'win32', 'x64'],
  ['binary-info.win32-ia32-msvc.node', 'win32', 'ia32'],
  ['binary-info.win32-arm64-msvc.node', 'win32', 'arm64'],
  ['binary-info.darwin-x64.node', 'darwin', 'x64'],
  ['binary-info.darwin-arm64.node', 'darwin', 'arm64'],
  ['binary-info.linux-x64-gnu.node', 'linux', 'x64'],
  ['binary-info.linux-arm-gnueabihf.node', 'linux', 'arm'],
  ['binary-info.linux-arm64-gnu.node', 'linux', 'arm64'],
];

console.log(`Current: { platform: ${process.platform}, arch: ${process.arch} }`);

for (const [file, platform, arch] of values) {
  const path = join('./', file);
  if (existsSync(path)) {
    console.log('Testing binary: ' + file);
    console.log(`Expecting: { platform: ${platform}, arch: ${arch} }`);
    const info = binary.getInfo(path);
    console.log('Result', info);

    assert.equal(info.platform, platform);
    assert(info.arches.includes(arch));
  }
}
