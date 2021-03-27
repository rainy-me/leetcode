#!/usr/bin/env python3

import os
import local
import sys


def get_q():
    qid = len(sys.argv) > 1 and int(sys.argv[1]) or int(
        input("🧚‍♀️#question_id to work on? "))
    for q in local.get_data()['stat_status_pairs']:
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
    ext = len(sys.argv) > 2 and sys.argv[2]  or "rs" #or input("🌎  language to use? ")
    file_path = local.create(q, ext)
    os.system(f"code {file_path}")
    print("✨ done")


if __name__ == "__main__":
    run()
