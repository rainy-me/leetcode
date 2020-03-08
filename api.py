import os
import requests
import math
from credentials import headers, cookies


def get_data():
    print('getting data...')
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
        frontend_question_id = stat['frontend_question_id']
        question_title = stat['question__title']

        file_dir = f'{difficulty}/{name}'
        file_path = f'{file_dir}/{name}.js'
        md_file_path = f'{file_dir}/#{frontend_question_id}-{name}.md'

        os.makedirs(file_dir, exist_ok=True)
        if not os.path.exists(file_path):
            with open(file_path, 'a') as f:
                f.write('')
        if not os.path.exists(md_file_path):
            with open(md_file_path, 'a') as f:
                f.write(f'# #{frontend_question_id} {question_title}')


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
