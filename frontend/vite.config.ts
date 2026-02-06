import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import { copyFileSync, existsSync } from 'fs'
import { resolve } from 'path'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [
    react(),
    {
      name: 'copy-indexnow-key',
      writeBundle() {
        // Copy IndexNow key file to dist root
        const keyFile = resolve(__dirname, 'assets/ee0e9227f58b45f78f8a2420e7c29363.txt')
        const destFile = resolve(__dirname, '../dist/ee0e9227f58b45f78f8a2420e7c29363.txt')

        if (existsSync(keyFile)) {
          copyFileSync(keyFile, destFile)
          console.log('✓ Copied IndexNow key file to dist root')
        }
      }
    }
  ],
  base: '/',
  build: {
    outDir: '../dist',
    emptyOutDir: true,
    sourcemap: false,
    rollupOptions: {
      output: {
        manualChunks: {
          vendor: ['react', 'react-dom', 'react-router-dom'],
          markdown: ['react-markdown', 'rehype-highlight', 'remark-gfm'],
        },
      },
    },
  },
  server: {
    port: 5173,
    proxy: {
      '/api': {
        target: 'http://localhost:3000',
        changeOrigin: true,
      }
    }
  }
})