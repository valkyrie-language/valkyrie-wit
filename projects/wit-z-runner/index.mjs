import {readFile} from 'node:fs/promises';
import {WASI} from 'wasi';
import {argv, env} from 'node:process';

const wasi = new WASI({
    version: 'preview1',
    args: argv,
    env,
    preopens: {
        '/sandbox': './tests',
    },
});

const wasm = await WebAssembly.compile(
    await readFile(new URL('./valkyrie-native.wasm', import.meta.url)),
);
const instance = await WebAssembly.instantiate(wasm, wasi.getImportObject());

wasi.start(instance);


