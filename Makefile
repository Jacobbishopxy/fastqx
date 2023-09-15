# @author:	Jacob Xie
# @date:	2023/09/09 14:59:22 Saturday

include Makefile.env

build-base:
	cargo build

build: build-base
	cd fastqx-py && maturin build

install: build
	pip install -I ./target/wheels/*.whl
