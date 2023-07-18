# Skeleton CLI

This is the skeleton command for the initial project file deployment.

See: https://github.com/departure-inc/skeleton-cli

Check skeleton/README.md on departure inc for details.

See: https://github.com/rawhide/skeleton

# Installation

```bash
$ brew tap departure-inc/skeleton-cli
$ brew install skeleton-cli
```

# Usage

Go to project root.

```
Usage: skeleton [OPTIONS] [ARGS]

OPTIONS:
  rails  : Ruby on Rails
  next   : Next.js
  rust   : Rust, Axum and SQLx
  python : Python, FastAPI and SQLAlchemy
  flutter: Flutter

ARGS:
  init  : initialize project.
  help  : help.
```

Run any command.

```bash
$ skeleton rails init
$ skeleton next init
$ skeleton rust init
$ skeleton python init
$ skeleton flutter init
```

# Help

```bash
$ skeleton help
```

## The following features to be implemented.

```bash
$ skeleton next g scaffold User name:string email:string
$ skeleton rust g scaffold User name:string email:string

ARGS:
  g | generate : resource generator
```

