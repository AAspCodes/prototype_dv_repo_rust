
lint-dv:
	# lint the yaml
	yamllint designvalues.yaml

fmt-python:
	black main.py
	
generate-book:
	# generate markdown book populated with yaml design values
	python main.py

save-requirements:
	# save python enviroment requirements to requirements.txt
	pip list > requirements.txt