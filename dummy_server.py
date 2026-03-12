import http.server
import socketserver
import time


class FastDummyHandler(http.server.SimpleHTTPRequestHandler):
    def log_message(self, format, *args):
        pass  # Suppress logging for maximum throughput

    def do_GET(self):
        self.send_response(200)
        self.send_header("Content-type", "text/html")
        self.end_headers()

        # Generate dynamic HTML on the fly so we don't even need disk I/O!
        path = self.path
        page_num = 0
        if path.startswith("/page_"):
            try:
                page_num = int(path.split("_")[1].split(".")[0])
            except:
                pass

        # Generate 100 links per page to spider out to 10,000 pages
        links = ""
        if page_num < 10000:
            for i in range(1, 101):
                next_page = page_num * 100 + i
                if next_page <= 10000:
                    links += f'<a href="/page_{next_page}.html">Link to page {next_page}</a><br>\n'

        html = f"""
        <html>
        <head><title>Dummy Page {page_num}</title></head>
        <body>
            <h1>This is dummy page {page_num}</h1>
            <p>We are testing the absolute maximum throughput of doc_transformer.</p>
            <p>This server generates pages entirely in RAM so there is zero disk bottleneck.</p>
            {links}
        </body>
        </html>
        """
        self.wfile.write(html.encode("utf-8"))


class ThreadedHTTPServer(socketserver.ThreadingMixIn, socketserver.TCPServer):
    daemon_threads = True


if __name__ == "__main__":
    PORT = 8080
    handler = FastDummyHandler
    httpd = ThreadedHTTPServer(("", PORT), handler)
    print(f"Starting extremely fast in-memory threaded server on port {PORT}")
    httpd.serve_forever()
