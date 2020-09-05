#!/usr/bin/env python3

import os
import api
import sys
import work_on


def run():
    q = work_on.get_q()
    # len(sys.argv) > 2 and sys.argv[2] or input("🌎  language to use? ")
    ext = "rs"
    stat = q['stat']
    difficulty = api.levels[q['difficulty']['level'] - 1]
    name = stat['question__title_slug']

    file_dir = f'{difficulty}/{name}'
    file_path = f'{file_dir}/{name}.{ext}'

    os.system(f"rustc {file_path} -o tmp")
    os.system(f"./tmp")
    print("✨ done")


if __name__ == "__main__":
    run()
