# @author:	Jacob Xie
# @date:	2023/09/09 14:59:22 Saturday

# public vars
include Makefile.env
# private vars
include Makefile.local.env

# https://www.maturin.rs/installation
devenv-init:
	pip install maturin maturin[patchelf]

build-base:
	cargo build

build: build-base
	cd fastqx-py && maturin build

install: build
	pip install -I ./target/wheels/*.whl

install-no-build:
	pip install -I ./target/wheels/*.whl

upload-nexus:
	python -m twine upload --repository-url ${NEXUS_URL} ./target/wheels/*.whl -u ${NEXUS_USER} -p ${NEXUS_PSWD}
