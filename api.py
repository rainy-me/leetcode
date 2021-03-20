#!/usr/bin/env python3

import os
import requests
import math
import json
from credentials import headers, cookies

levels = ['easy', 'medium', 'hard']
local_data_path = 'test_api.json'


def temples(ext):
    if ext == 'rs':
        return """struct Solution {}

fn main() {
  assert_eq!(0, 0);
}
"""
    return ""


def get_api_data():
    print('📦  getting api data...')
    data = requests.get(
        'https://leetcode.com/api/problems/all/',
        headers=headers,
        cookies=cookies
    ).json()
    with open(local_data_path, 'w') as f:
        json.dump(data, f)
    return data


def get_data():
    if not os.path.exists(local_data_path):
        return get_api_data()
    print('📦  getting local data...')

    with open(local_data_path, 'r') as f:
        return json.load(f)


def gen():
    response = get_api_data()

    for level in levels:
        os.makedirs(level, exist_ok=True)

    for q in response['stat_status_pairs']:
        create(q)


def create(q, ext='js'):
    stat = q['stat']
    difficulty = levels[q['difficulty']['level'] - 1]
    name = stat['question__title_slug']
    frontend_question_id = stat['frontend_question_id']
    question_title = stat['question__title']

    file_dir = f'{difficulty}/{name}'
    file_path = f'{file_dir}/{name}.{ext}'
    md_file_path = f'{file_dir}/#{frontend_question_id}-{name}.md'

    os.makedirs(file_dir, exist_ok=True)
    if not os.path.exists(file_path):
        with open(file_path, 'a') as f:
            f.write(temples(ext))
    if not os.path.exists(md_file_path):
        with open(md_file_path, 'a') as f:
            f.write(f'# #{frontend_question_id} {question_title}')
    os.chdir(file_dir)
    os.system('cargo init')

    return file_path


def md():
    response = get_api_data()
    solved = response['num_solved']
    total = response['num_total']
    easy = response['ac_easy']
    medium = response['ac_medium']
    hard = response['ac_hard']
    with open('./README.md', 'w') as f:
        f.write(
            f"""# leetcode problems

{solved} / {total}

{round(solved / total * 100, 2)}%

|        |     |
| ------ | --- |
| easy   | {easy}  |
| medium | {medium}   |
| hard   | {hard}   |

"""
        )


if __name__ == '__main__':
    gen()
