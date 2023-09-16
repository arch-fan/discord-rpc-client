import requests
import os

url = f"https://discord.com/api/v9/applications/{os.getenv('APP_ID')}"

headers = {
    "Authorization": os.getenv('AUTH_HEADER'),
}

payload = {
    "name": os.getenv('APP_NAME')
}

response = requests.patch(url, json=payload, headers=headers)

if response.status_code != 200:
    print(f"Error at the request: {response.status_code}")
    exit(1)
