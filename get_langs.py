from os import listdir
from os.path import isfile, join
import re

PATH = "static/js/"
REGEX = r"mode-(.+).js"
files = [f for f in listdir(PATH) if isfile(join(PATH, f))]

langs = []
for file in files:
    match = re.search(REGEX, file, re.IGNORECASE)
    if match:
        langs.append(match.group(1))

for lang in langs:
    print "<option value=\"%s\">%s</option>" % (lang, lang)