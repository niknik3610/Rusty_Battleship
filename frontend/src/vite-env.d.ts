/// <reference types="vite/client" />
//
export default {
    server: {
        proxy: {
            '/api': {
                target: 'http://localhost:8000',
                changeOrigin: true,
                rewrite: (path) => path.replace(/^\/api/, ''),    
            }
        }
    }
};
