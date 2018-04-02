# ffind
A very simple Rust program to search in your folders with the CLI

# Features
This utility allows you to search recursively in your folders for a filename with the CLI. It ignores the upper/lower case, so it makes file search very easy.

# Usage
```
$ ffind filename
./path/to/filename
```

By default, ffind doesn't dive into .something folders. If you want to enable deep search, issue:
```
$ffind -a filename
./path/to/filename
```

Or:
```
$ ffind --deep filename
./path/to/filename
```

The output is colorized by default. If you want an uncolorized output, then you can set the `NO_COLOR` variable or use this command:
```
$ ffind --no-color filename
./path/to/filename
```

# Compilation
First clone the repository:
```
git clone git://github.com/josephscade/ffind.git
```

Then you have to install the following dependencies:
* rust
* cargo

Then compile it with Cargo:
```
cargo build --release
```

# Installation
Simply run this command as root:
```
make install
```
