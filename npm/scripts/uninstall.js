#!/usr/bin/env node

/**
 * Uninstall script for demoji npm package
 * Cleans up any downloaded binaries
 */

const fs = require('fs');
const path = require('path');

const binDir = path.join(__dirname, '..', 'bin');

// Remove bin directory if it exists
if (fs.existsSync(binDir)) {
  try {
    fs.rmSync(binDir, { recursive: true, force: true });
    console.log('Cleaned up demoji binary directory');
  } catch (err) {
    console.error('Failed to clean up binary directory:', err.message);
    // Don't exit with error - uninstall should succeed even if cleanup fails
  }
}
