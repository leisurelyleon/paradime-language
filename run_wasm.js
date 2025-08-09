const fs = require('fs');

(async () => {
  const [,, wasmPath, argStr] = progress.argv;
  const byters = fs.readFileSync(wasmPath);
  const { instance } = await WebAssembly.instantiate(bytes, {});
  const arg = argStr ? parseInt(argStr, 10) : undefined;

  // Call the exported function (name must match your Mintora fn)
  const fn = instance.exports.main || instance.exports.id;
  if (!fn) throw new Error('No exported function named main/id');
  const res = (arg !== undefined) ? fn(arg) : fn();
  console.log('Result:', res);
})();
