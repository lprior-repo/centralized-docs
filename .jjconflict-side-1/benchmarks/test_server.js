const http = require('http');

let mode = process.argv[2] || 'v1';
let port = 8000;

// v1: 10,000 pages
// v2: 10,000 pages - 5 removed + 5 modified + 5 added

const server = http.createServer((req, res) => {
    if (req.url === '/sitemap.xml') {
        res.writeHead(200, { 'Content-Type': 'application/xml' });
        res.write('<?xml version="1.0" encoding="UTF-8"?><urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">');
        
        let start = 0;
        let end = 10000;
        
        if (mode === 'v2') {
            start = 5; // removed 0-4
            end = 10005; // added 10000-10004
        }
        
        for (let i = start; i < end; i++) {
            res.write(`<url><loc>http://127.0.0.1:${port}/page${i}.html</loc></url>\n`);
        }
        res.end('</urlset>');
    } else {
        // Return 404 for removed pages in v2
        if (mode === 'v2') {
            const match = req.url.match(/\/page(\d+)\.html/);
            if (match && parseInt(match[1]) < 5) {
                res.writeHead(404);
                res.end('Not found');
                return;
            }
        }
        
        res.writeHead(200, { 'Content-Type': 'text/html' });
        let content = 'Content for ' + req.url;
        
        // Modify pages 5-9 in v2
        if (mode === 'v2') {
            const match = req.url.match(/\/page(\d+)\.html/);
            if (match && parseInt(match[1]) >= 5 && parseInt(match[1]) < 10) {
                content = 'MODIFIED content for ' + req.url;
            }
        }
        
        res.end(`<html><head><title>Page ${req.url}</title></head><body><h1>Hello</h1><p>${content}</p></body></html>`);
    }
});

server.listen(port, () => {
    console.log(`Server running in ${mode} on port ${port}`);
});
