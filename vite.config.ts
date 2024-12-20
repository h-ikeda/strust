import vue from '@vitejs/plugin-vue'
import wasm from '@rollup/plugin-wasm'
import { spawn } from 'child_process'

function wasmPack() {
  const p = spawn('wasm-pack', [
    'build', '--no-pack', '--out-name=index', '--target=web',
  ], {
    stdio: ['pipe', 'pipe', 'inherit'],
  })
  return new Promise((resolve, reject) => {
    p.on('exit', resolve)
    p.on('error', reject)
  })
}

export default {
  plugins: [
    vue(),
    wasm(),
    {
      name: 'wasm-pack',
      buildStart: wasmPack,
      async watchChange(id) {
        if (!/\.rs$/i.test(id)) return
        await wasmPack()
      },
    }
  ],
}
