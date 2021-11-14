const binary = require('./index');
const assert = require('assert');

const values = [
  ['binary-info.win32-x64-msvc.node', 'win32', 'x64'],
  ['binary-info.win32-ia32-msvc.node', 'win32', 'ia32'],
  ['binary-info.darwin-x64.node', 'darwin', 'x64'],
  ['binary-info.darwin-arm64.node', 'darwin', 'arm64'],
  ['binary-info.linux-x64-gnu.node', 'linux', 'x64'],
  ['binary-info.linux-arm-gnueabihf.node', 'linux', 'arm'],
  ['binary-info.linux-arm64-gnu.node', 'linux', 'arm64'],
];

for (const [file, platform, arch] of values) {
  console.log('Testing ' + file);
  console.log('Expecting', platform, arch);
  const info = binary.getInfo('./' + file);
  console.log('Result', info);
  assert.equal(info.platform, platform);
  assert.equal(info.arch, arch);
}
