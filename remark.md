# Remark

## Linking Problem

If `cargo test` encountered error like this:

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
