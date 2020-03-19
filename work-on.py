#!/usr/bin/env python

import os
import api


def get_q():
    qid = int(input("🧚‍♀️#question_id to work on? "))
    for q in api.get_data()['stat_status_pairs']:
        stat = q['stat']
        name = stat['question__title_slug']
        frontend_question_id = stat['frontend_question_id']
        if frontend_question_id == qid:
            print(f'🔍  #{qid} {name}')
            return q
    print(f"no question with id {qid}")
    exit()


def run():
    q = get_q()
    ext = input("🌎  language to use? ")
    file_path = api.create(q, ext)
    os.system(f"code {file_path}")
    print("✨ done")


if __name__ == "__main__":
    run()
