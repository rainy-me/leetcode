import json
import os

levels = ['easy', 'medium', 'hard']
local_data_path = 'test_api.json'

def temples(ext):
    if ext == 'rs':
        return """#![feature(fn_traits)]

use utils::setup;

setup!();

#[cfg(test)]
impl Solution {

}

table_test! {
    fn,
    (,) => 0;
}
"""
    return ""


def get_data():
    # if not os.path.exists(local_data_path):
    #     return get_api_data()
    print('📦  getting local data...')

    with open(local_data_path, 'r') as f:
        return json.load(f)

def create(q, ext='js'):
    stat = q['stat']
    difficulty = levels[q['difficulty']['level'] - 1]
    name = stat['question__title_slug']
    frontend_question_id = stat['frontend_question_id']
    question_title = stat['question__title']

    file_dir = f'{difficulty}/{name}'
    file_path = f'{file_dir}/{name}.{ext}'
    md_file_path = f'{file_dir}/#{frontend_question_id}-{name}.md'
    cargo_toml = f'{file_dir}/Cargo.toml'

    os.makedirs(file_dir, exist_ok=True)
    if not os.path.exists(file_path):
        with open(file_path, 'a') as f:
            f.write(temples(ext))
    if not os.path.exists(md_file_path):
        with open(md_file_path, 'a') as f:
            f.write(f'# #{frontend_question_id} {question_title}')
    with open(cargo_toml, 'a') as f:
        f.write(f"""[package]
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
""")

    with open("./Cargo.toml", 'r+') as f:
      data = f.read()
      f.seek(0)
      f.write(data.replace(',\n]',f',\n    "{file_dir}",\n]'))
      f.truncate()
    return file_path