#!/usr/bin/env python3
import os
import utils

utils.update_md()
os.system("git --no-pager diff README.md")
os.system("git add .")
os.system(f"git commit -m '{utils.get_update_message()}'")
os.system("git push origin master")
