#!/usr/bin/env node

/**
 * Install script for demoji npm package
 * Downloads the appropriate pre-built binary for the current platform
 * 
 * Placeholder implementation - replace with actual download logic when publishing
 */

const fs = require('fs');
const path = require('path');
const os = require('os');
const https = require('https');

// Determine platform and architecture
const platform = os.platform();
const arch = os.arch();

// Map Node.js platform/arch to Rust target triple
const targetMap = {
  'darwin-x64': 'x86_64-apple-darwin',
  'darwin-arm64': 'aarch64-apple-darwin',
  'linux-x64': 'x86_64-unknown-linux-gnu',
  'linux-arm64': 'aarch64-unknown-linux-gnu',
  'win32-x64': 'x86_64-pc-windows-msvc',
  'win32-arm64': 'aarch64-pc-windows-msvc',
};

const key = `${platform}-${arch}`;
const target = targetMap[key];

if (!target) {
  console.error(`Unsupported platform: ${platform} ${arch}`);
  process.exit(1);
}

// Placeholder: Replace with actual GitHub release URL when publishing
const version = require('../package.json').version;
const binaryName = platform === 'win32' ? 'demoji.exe' : 'demoji';
const downloadUrl = `https://github.com/dbmrq/demoji/releases/download/v${version}/demoji-${version}-${target}.tar.gz`;

console.log(`Installing demoji for ${platform} ${arch} (${target})...`);
console.log(`Download URL: ${downloadUrl}`);

// Create bin directory if it doesn't exist
const binDir = path.join(__dirname, '..', 'bin');
if (!fs.existsSync(binDir)) {
  fs.mkdirSync(binDir, { recursive: true });
}

// Placeholder: Actual implementation would:
// 1. Download the binary from the URL
// 2. Extract it to the bin directory
// 3. Make it executable on Unix systems
// 4. Verify the checksum

console.log(`
Placeholder: Binary download not yet implemented.

To complete the installation, you can:
1. Download the binary manually from: ${downloadUrl}
2. Extract it to: ${binDir}
3. Make it executable: chmod +x ${path.join(binDir, binaryName)}

Or install from source:
  cargo install demoji
`);

// For now, create a placeholder script that shows the installation message
const placeholderScript = `#!/bin/sh
echo "demoji binary not yet installed. Please run:"
echo "  npm run postinstall"
echo "Or install from source:"
echo "  cargo install demoji"
exit 1
`;

const scriptPath = path.join(binDir, binaryName);
fs.writeFileSync(scriptPath, placeholderScript);
fs.chmodSync(scriptPath, 0o755);

console.log('Placeholder binary created. Run "npm run postinstall" to download the actual binary.');
