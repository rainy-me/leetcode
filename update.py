#!/usr/bin/env python
import os
import api

api.md()
os.system("git --no-pager diff README.md")
os.system("git add .")
files = os.popen("git diff --name-only --cached").read()

q_set, id_set = set(), set()
for f in files.split('\n'):
    parts = f.split('/')
    if len(parts) > 1:
        q_set.add(parts[1])

for q in api.get_data()['stat_status_pairs']:
    stat = q['stat']
    name = stat['question__title_slug']
    if name in q_set:
        frontend_question_id = stat['frontend_question_id']
        id_set.add(frontend_question_id)

message = 'update'
if len(id_set):
    message += " " + ", ".join([str(qid) for qid in id_set])
os.system(f"git commit -m '{message}'")
os.system("git push origin master")
