from mini_server import start_server
port = start_server(thread_count=4, port=None)  # Port `None` lets OS pick free port

import requests
url = f"http://127.0.0.1:{port}/hello/World"
response = requests.get(url)
print(response.json())  # {'message': 'Hello, World!'}
