# Code Certifier

`code-certifier` is a command-line tool divided into two sub-commands: `hazard-analyzer` and `pub-api`.

The command-line binary is in `ccertifier`.

The `hazard-analyzer` crate is in [crates/hazard-analyzer](./crates/hazard-analyzer/). 

The `pub-api` crate is in [crates/pub-api](./crates/pub-api/).

## Hazard Analyzer

Run `hazard-analyzer` with the following command:

```console
ccertifier hazard-analyzer [OPTIONS] --firmware-path <FIRMWARE_PATH> --manifest-path <MANIFEST_PATH>
```

To see the list of supported options, run:

```console
ccertifier help hazard-analyzer
```

Example with some options:

```console
ccertifier hazard-analyzer --firmware-path path/to/firmware --devices-path path/to/devices --manifest-path path/to/manifest --quiet
```

### Firmware Path

To specify the path to the firmware, use `--firmware-path` or `-f`:

```console
ccertifier hazard-analyzer -f path/to/firmware -m <MANIFEST_PATH>
```

### Devices Path

To specify the path to a local directory containing the `Ascot` devices, use `--devices-path` or `-d`:

```console
ccertifier hazard-analyzer -f path/to/firmware -d path/to/devices -m <MANIFEST_PATH>
```

### Manifest Path

To specify the output `JSON` manifest path, use `--manifest-path` or `-m`:

```console
ccertifier hazard-analyzer -f path/to/firmware -m path/to/manifest
```

If not specified the tool will clone the remote `ascot-firmware` repository and use that `ascot-firmware/ascot-axum/src/devices/`.

### Quiet

To prevent the tool from printing the analysis results to the terminal, use `--quiet` or `-q`:

```console
ccertifier hazard-analyzer -f path/to/firmware -q -m <MANIFEST_PATH>
```

## Pub API

Run `pub-api` with the following command:

```console
ccertifier pub-api [OPTIONS] --manifest-path <MANIFEST_PATH>
```

To see the list of supported options, run:

```console
ccertifier help pub-api
```

Example with some options:

```console
ccertifier pub-api --library-path path/to/ascot/library/manifest --axum-path path/to/ascot/axum/manifest --manifest-path path/to/manifest
```

### Ascot Library Path

To specify the path to the `Cargo.toml` manifest of a local directory containing `ascot-library`, use `--library-path` or `-l`:

```console
ccertifier pub-api -l path/to/ascot/library/manifest -m <MANIFEST_PATH>
```

If not specified the tool will clone the remote `ascot-firmware` repository and use that `ascot-firmware/Cargo.toml`.

### Ascot Axum Path

To specify the path to the `Cargo.toml` manifest of a local directory containing `ascot-axum`, use `--axum-path` or `-a`:

```console
ccertifier pub-api -a path/to/ascot/axum/manifest -m <MANIFEST_PATH>
```

If not specified the tool will clone the remote `ascot-firmware` repository and use that `ascot-firmware/ascot-axum/Cargo.toml`.

### Manifest Path

To specify the output `JSON` manifest path, use `--manifest-path` or `-m`:

```console
ccertifier pub-api -m path/to/manifest
```
