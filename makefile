SHELL := /bin/bash

CC := /usr/bin/cargo
CURL := /usr/bin/curl
DATA_DIR := datas

IRIS_DATASET_URL := https://gist.githubusercontent.com/curran/a08a1080b88344b0c8a7/raw/0e7a9b0a5d22642a06d3d5b9bcbad9890c8ee534/iris.csv

build: clean
	$(CC) build --release

run:
	$(CC) run --release

clean:
	rm -rvf ${DATA_DIR} target

fetch: dependencies
	[[ -d ${DATA_DIR} ]] || mkdir -v ${DATA_DIR}
	${CURL} ${IRIS_DATASET_URL} > ${DATA_DIR}/iris.csv

dependencies:
	command -v ${CURL} >/dev/null \
		|| echo "Dependency Error: ${CURL} not installed." \
		|| false
