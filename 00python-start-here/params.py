import json
import sys

if len(sys.argv) == 1: sys.exit(f'usage: {sys.argv[0]} <json file>')

with open(sys.argv[1]) as fp:
	hmm = json.load(fp)
print(json.dumps(hmm, indent=2))
