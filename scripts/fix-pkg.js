const fs = require('fs');
const path = require('path');

keywords = [
  "wasm",
  "rust",
  "rhythm-game",
  "parser",
  "converter",
  "skin"
]

const nodePackagePath = path.join(__dirname, '../dist-node/package.json');
const nodePackage = JSON.parse(fs.readFileSync(nodePackagePath, 'utf8'));
nodePackage.name = '@r2o3/rgskin-nodejs';
nodePackage.keywords = Keywords;
fs.writeFileSync(nodePackagePath, JSON.stringify(nodePackage, null, 2));
console.log('Node package name and keywords updated');

const webPackagePath = path.join(__dirname, '../dist-web/package.json');
const webPackage = JSON.parse(fs.readFileSync(webPackagePath, 'utf8'));
webPackage.name = '@r2o3/rgskin-browser';
webPackage.keywords = Keywords;
fs.writeFileSync(webPackagePath, JSON.stringify(webPackage, null, 2));
console.log('Web package name and keywords updated');

console.log('All package names and keywords updated');