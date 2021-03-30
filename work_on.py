#!/usr/bin/env python3

import os
import utils


def run():
    q = utils.get_question()
    file_path = utils.setup(q)
    os.system(f"code {file_path}")
    print("✨ done")


if __name__ == "__main__":
    run()
