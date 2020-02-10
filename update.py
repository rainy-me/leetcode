#!/usr/bin/env python
import os
from api import md

md()
os.system("git --no-pager diff README.md")
os.system("git add .")
os.system("git commit -m 'update'")
os.system("git push origin master")
