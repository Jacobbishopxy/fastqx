# Remark

## Linking Problem

- If `cargo test` encountered error like this:

    ```txt
    error while loading shared libraries: libpython3.8.so.1.0: cannot open shared object file: No such file or directory
    ```

    We can add following setting into `./vscode/settings.json`:

    ```json
        ...
        "rust-analyzer.runnables.extraEnv": {
            "LD_LIBRARY_PATH": "/opt/anaconda3/envs/py38/lib"
        },
        ...
    ```

- In MacOS, if `cargo build` encountered error like this ([ref](https://stackoverflow.com/a/65698711)):

    ```txt
    error: linking with `cc` failed: exit status: 1
    ```

    We can add following setting into `~/.cargo/config`:

    ```cfg
    [target.x86_64-apple-darwin]
    rustflags = [
    "-C", "link-arg=-undefined",
    "-C", "link-arg=dynamic_lookup",
    ]

    [target.aarch64-apple-darwin]
    rustflags = [
    "-C", "link-arg=-undefined",
    "-C", "link-arg=dynamic_lookup",
    ]
    ```
