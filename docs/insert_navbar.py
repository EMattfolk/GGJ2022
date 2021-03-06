import os
import sys

from bs4 import BeautifulSoup

LINKS = [("Home", "index.html"), ("Guide", "guide.html"), ("Quick Reference", "quick-reference.html")]

filename = sys.argv[1]
with open(filename, "r") as f:
    soup = BeautifulSoup(f.read(), "html.parser")

navbar = soup.new_tag("nav")
navbar["class"] = "navbar"
navbar_title = soup.new_tag("div", id="navtitle")
navbar_title.string = "Navigation"
navbar_list = soup.new_tag("ul")
navbar_list["class"] = "navbar-list"
navbar.append(navbar_title)
navbar.append(navbar_list)

for text, href in LINKS:
    list_item = soup.new_tag("li")
    list_item["class"] = "navbar-item"
    if href == os.path.basename(filename):
        list_item["class"] += " navbar-item-active"
    link = soup.new_tag("a", href=href)
    link.string = text
    list_item.append(link)
    navbar_list.append(list_item)

soup.find("div", { "id": "toc" }).insert(0, navbar)

with open(filename, "w") as f:
    f.write(soup.prettify())
