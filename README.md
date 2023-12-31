# Prototype Design Value Repo
Intended as a proof-of-concept for a general architecture


The [designvalues.yaml](./designvalues.yaml) is the final source of truth for the data. The python script reads the values out of the yaml and generates markdown text file that are then rendered by off the shelf markdown renderer to make pleaseing to read by a human. 

Any changes to the source of truth `designvalues.yaml` are recorded in git, giving a full history of the values, when they were changed, who changed, and any discussion around the change. If the same line was changed multiple times, the history of each change is preserved.

