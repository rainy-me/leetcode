import os
import requests
from credentials import headers, cookies

response = requests.get(
    'https://leetcode.com/api/problems/algorithms/',
    headers=headers,
    cookies=cookies
)

levels = ['easy', 'medium', 'hard']

for level in levels:
    os.makedirs(level, exist_ok=True)

for q in response.json()['stat_status_pairs']:
    stat = q['stat']
    difficulty = levels[q['difficulty']['level'] - 1]
    name = stat['question__title_slug']

    file_dir = f'{difficulty}/{name}'
    file_path = f'{file_dir}/{name}.js'

    os.makedirs(file_dir, exist_ok=True)
    if not os.path.exists(file_path):
        with open(file_path, 'a') as f:
            f.write('')
