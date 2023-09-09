# @author:	Jacob Xie
# @date:	2023/09/09 14:59:22 Saturday

include Makefile.env

build:
	maturin build

install: build
	pip install -I ./target/wheels/*.whl
