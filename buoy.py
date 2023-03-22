import urllib

def get_heading():
    webpage = urllib.request.urlopen("127.0.0.1:8080").read()
    firstline = webpage.split("\n")[0]
    return float(firstline)
