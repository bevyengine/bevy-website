import re
import os
import sys

name_fix = {
    'bevy_nbody': 'thallada-bevy_nbody',
    'bevy-nbody': 'WhoisDavid-bevy-nbody',
}

f = open(sys.argv[1] + "/README.md")
lines = f.readlines()

root_folder = sys.argv[2] + "/"
os.mkdir(root_folder)

category = None
subcategory = None
current_path = root_folder
for line in lines:
    if line[0:3] == "## ":
        category = line.split("# ")[1][0:-1]
        current_path = root_folder + category
        os.mkdir(current_path)
    elif line[0:3] == "###":
        subcategory = line.split("# ")[1][0:-1]
        current_path = root_folder + category + "/" + subcategory
        os.mkdir(current_path)
    elif line[0:2] == "* ":
        line = line[0:-1]
        m = re.search('\* \[([^]]*)\]\(([^)]*)\)(: (.*))?', line)
        name = m.group(1)
        link = m.group(2)
        desc = m.group(4)
        if name in name_fix:
            name = name_fix[name]
        f = open(current_path + '/' + name.replace(' ', '-').replace('/', '-') + ".toml", "w")
        f.write("name = \"" + name + "\"\n")
        if desc is not None:
            f.write("description = \"" + desc.replace('"', '\'') + "\"\n")
        f.write("link = \"" + link + "\"\n")
        f.close()

