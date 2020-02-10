import os
import requests
import math
from credentials import headers, cookies


def get_data():
    print('geting data...')
    return requests.get(
        'https://leetcode.com/api/problems/all/',
        headers=headers,
        cookies=cookies
    ).json()


def gen():
    response = get_data()
    levels = ['easy', 'medium', 'hard']

    for level in levels:
        os.makedirs(level, exist_ok=True)

    for q in response['stat_status_pairs']:
        stat = q['stat']
        difficulty = levels[q['difficulty']['level'] - 1]
        name = stat['question__title_slug']

        file_dir = f'{difficulty}/{name}'
        file_path = f'{file_dir}/{name}.js'

        os.makedirs(file_dir, exist_ok=True)
        if not os.path.exists(file_path):
            with open(file_path, 'a') as f:
                f.write('')


def md():
    response = get_data()
    solved = response['num_solved']
    total = response['num_total']
    easy = response['ac_easy']
    medium = response['ac_medium']
    hard = response['ac_hard']
    with open('./README.md', 'w') as f:
        f.write(
            f"""# leetcode problems

{solved} / {total}

{round(solved / total,2) * 100}%

|        |     |
| ------ | --- |
| easy   | {easy}  |
| medium | {medium}   |
| hard   | {hard}   |

"""
        )
