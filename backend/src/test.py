import requests
y = requests.post('http://localhost:8000/gen', json = {"url": "https://docs.rs"})
#y = requests.post('http://localhost:8000/gen', {"url": "awd"})
print(y.text)