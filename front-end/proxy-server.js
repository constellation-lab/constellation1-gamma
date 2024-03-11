require('dotenv').config();
const express = require('express');
const cors = require('cors');
const { createProxyMiddleware } = require('http-proxy-middleware');

const app = express();
const PORT = process.env.NEXT_PUBLIC_PROXY_PORT || 3090;
const NODE_SERVER = "https://rpc.nibiru.fi:443"




app.use(cors());

app.use('/submit_pfb', (req, res, next) => {
    //if sumbiturl is empty, use default server
    let TARGET_SERVER = NODE_SERVER;
    
    const proxy = createProxyMiddleware({
        target: TARGET_SERVER,
        changeOrigin: true,
        pathRewrite: {
            '^/submit_pfb/': '/', 
          },
                onProxyReq: (proxyReq) => {
            proxyReq.removeHeader('origin');
        },
        onProxyRes: (proxyRes) => {
            proxyRes.headers['Access-Control-Allow-Origin'] = req.headers.origin || '*';
            proxyRes.headers['Access-Control-Allow-Methods'] = 'GET,POST,PUT,DELETE,OPTIONS';
            proxyRes.headers['Access-Control-Allow-Headers'] = 'Origin, X-Requested-With, Content-Type, Accept, submiturl';
        },
    });

    proxy(req, res, next);
});

app.listen(PORT, () => {
    console.log(`Proxy server is running on http://localhost:${PORT}`);
})