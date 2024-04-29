# ozipw
![GitHub License](https://img.shields.io/github/license/WangQiang102938/osho-comp-app)

osho zip wizard

## Description

A CLI wizard to extract or make archives, made by osho, written with Rust.

This is a tool for CLI to have wizard-like zip/unzip experience.

## Usage
```
ozipw [OPTIONS] <ARGUMENTS...>
OPTIONS:
    -m, --mode <MODE>       Mode of operation: wizard(default), zip, unzip, auto.
    -d, --dest <DEST>       Destination of results. Default is the current directory.
    -o, --output <FILE>     Output file for the archive. default is dir/file name.
        --overwrite         Overwrite output file if exist.
    -v, --verbose           Display verbose output.
    -h, --help              Display help message.
ARGUMENTS:
    wizard: Auto detect extenders. then add to working queue.
            If it is an archive file, set to extract mode
            Otherwise will set to archive mode
    zip:    Files/directory to be archived
    unzip:  File to be extract
    auto:   Auto detect args, if arg is archive file, app will extract it.
            If arg is a directory, app will archive it.
            Otherwise, app will stop auto mode and get into wizard mode.

```

## Install(TODO)
### Linux
```sh
# TODO
```


### MacOS
```sh
# brew install 
```

## About

## Author

* OSho(Wang Qiang)