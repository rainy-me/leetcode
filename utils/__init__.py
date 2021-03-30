#!/usr/bin/env python3

import os
import requests
import math
import json
from pathlib import Path
from .credentials import headers, cookies

__parent__ = Path(__file__).parent
__root__ = __parent__.parent

levels = ["easy", "medium", "hard"]
local_data_path = __parent__ / "data.json"


def get_data():
    # if not os.path.exists(local_data_path):
    #     return get_api_data()
    print("📦  getting local data...")

    with open(local_data_path, "r") as f:
        return json.load(f)


def get_api_data():
    print("📦  getting api data...")
    data = requests.get(
        "https://leetcode.com/api/problems/all/", headers=headers, cookies=cookies
    ).json()
    with open(local_data_path, "w") as f:
        json.dump(data, f)
    return data


def gen_question():
    response = get_api_data()

    for level in levels:
        os.makedirs(level, exist_ok=True)

    for q in response["stat_status_pairs"]:
        create(q)


def get_question():
    qid = (
        len(sys.argv) > 1
        and int(sys.argv[1])
        or int(input("🧚‍♀️#question_id to work on? "))
    )
    for q in get_data()["stat_status_pairs"]:
        stat = q["stat"]
        name = stat["question__title_slug"]
        frontend_question_id = stat["frontend_question_id"]
        if frontend_question_id == qid:
            print(f"🔍  #{qid} {name}")
            return q
    print(f"no question with id {qid}")
    exit()


def update_md():
    response = get_api_data()
    solved = response["num_solved"]
    total = response["num_total"]
    easy = response["ac_easy"]
    medium = response["ac_medium"]
    hard = response["ac_hard"]
    with open(__root__ / "README.md", "w") as f:
        f.write(
            f"""# leetcode problems

{solved} / {total} ({round(solved / total * 100, 2)}%)

| level  | done     |
| ------ | -------- |
| easy   | {easy}   |
| medium | {medium} |
| hard   | {hard}   |

"""
        )


def temples(ext):
    if ext == "rs":
        return """#![feature(fn_traits)]

use utils::setup;

setup!();

#[cfg(test)]
impl Solution {

}

test! {
    fn,
    (,) => 0;
}
"""


def setup(q):
    stat = q["stat"]
    difficulty = levels[q["difficulty"]["level"] - 1]
    name = stat["question__title_slug"]
    frontend_question_id = stat["frontend_question_id"]
    question_title = stat["question__title"]

    file_dir = __root__ / difficulty / name
    file_path = file_dir / f"{name}.{ext}"
    md_file_path = file_dir / f"#{frontend_question_id}-{name}.md"
    cargo_toml = file_dir / "Cargo.toml"

    os.makedirs(file_dir, exist_ok=True)
    if not os.path.exists(file_path):
        with open(file_path, "a") as f:
            f.write(temples(ext))
    if not os.path.exists(md_file_path):
        with open(md_file_path, "a") as f:
            f.write(f"# #{frontend_question_id} {question_title}")
    with open(cargo_toml, "a") as f:
        f.write(
            f"""[package]
name = "{name}"
version = "0.1.0"
authors = ["rainy-me <github@rainy.me>"]
edition = "2018"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
utils = {{path="../../utils"}}

[[bin]]
name = "{name}"
path = "{name}.rs"
"""
        )

    with open(__root__ / "Cargo.toml", "r+") as f:
        data = f.read()
        f.seek(0)
        f.write(data.replace(",\n]", f',\n    "{file_dir}",\n]'))
        f.truncate()
    return file_path


def get_update_message():
    files = os.popen("git diff --name-only --cached").read()

    q_set, id_set = set(), set()
    for f in files.split("\n"):
        parts = f.split("/")
        if len(parts) > 1:
            q_set.add(parts[1])

    for q in get_data()["stat_status_pairs"]:
        stat = q["stat"]
        name = stat["question__title_slug"]
        if name in q_set:
            frontend_question_id = stat["frontend_question_id"]
            id_set.add(frontend_question_id)

    message = "update"
    if len(id_set):
        message += " " + ", ".join([str(qid) for qid in id_set])
    return message