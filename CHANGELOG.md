### Features
- Install command (only if hosted in git repository)
- New daemon comunication
- Fork process to background
- Verbosity level
- Fix widget log file path
- Add internal commands (battery, sysinfo, widget)
- Create nodejs bridgen to the internal API

### Refactor
- SbbwWidget new structure, more scalable and easy add features
- New arguments on cli
- Replace structopts to native clap

### Details
New commands and migrate old commands and arguments
---
Old:
```sh
Sbbw Daemon 0.1.2
Sergio Ribera
This is the launcher and manager for the Sbbw Wigets

USAGE:
    sbbw [FLAGS] [OPTIONS]

FLAGS:
    -h, --help            Prints help information
        --no-fork
    -s, --show-windows
    -V, --version         Prints version information

OPTIONS:
        --check-config <check-config>
    -c, --close <close>
    -o, --open <open>
    -p, --port <port>                     [default: 8111]
        --test <test>...
    -t, --toggle <toggle>
```

New:
```sh
sbbw 0.1.2
Sergio Ribera
This is the launcher and manager for the Sbbw Wigets

USAGE:
    sbbw [OPTIONS] [SUBCOMMAND]

OPTIONS:
    -h, --help            Print help information
        --no-fork
    -p, --port <PORT>     [default: 8111]
    -q, --quiet           Less output per occurrence
    -s, --show-windows
    -v, --verbose         More output per occurrence
    -V, --version         Print version information

SUBCOMMANDS:
    check
    close
    help       Print this message or the help of the given subcommand(s)
    install
    open
    run
    test
    toggle
```

### TODO
- Add event callbacks api
