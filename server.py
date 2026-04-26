import http.server
import os

class SPAHandler(http.server.SimpleHTTPRequestHandler):
    def do_GET(self):
        # Если запрашивают файл, которого нет — отдаём index.html
        if not os.path.exists(self.translate_path(self.path)):
            self.path = '/index.html'
        return super().do_GET()

if __name__ == '__main__':
    http.server.test(HandlerClass=SPAHandler, port=8080)
