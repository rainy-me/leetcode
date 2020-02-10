#!/usr/bin/env python
import os
from api import md

md()
os.system("git add . && git commit -m 'update' && git push origin master")
