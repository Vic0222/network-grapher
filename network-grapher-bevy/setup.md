To setup for wasm see:
https://bevy-cheatbook.github.io/platforms/wasm.html

If an error saying something like:
` 
Error: 

it looks like the Rust project used to create this wasm file was linked against
version of wasm-bindgen that uses a different bindgen format than this binary:

  rust wasm file schema version: 0.2.100
     this binary schema version: 0.2.92

Currently the bindgen format is unstable enough that these two schema versions
must exactly match. You can accomplish this by either updating this binary or
the wasm-bindgen dependency in the Rust project.
`

Make sure to update wasm-server-runner, by uninstalling and reinstalling it.