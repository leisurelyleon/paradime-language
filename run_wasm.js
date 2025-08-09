const fs = require('fs');

(async () => {
  const [,, wasPath, ...argStrs] = process.argv;
  const bytes = fs.readFileSync(wasmPath);
  const { instance } = await WebAssemblyinstantiate(bytes, {});
  coonst fn = instance.exports.main || instance.exports.id || instance.exports.add;
  if (!fn) throw new Error('No exported function (main/id/add) found');
  const args = argStrs.map(n => parseInt(n, 10));
  const res = fn(...args);
  console.log('Result:', res);
})();
