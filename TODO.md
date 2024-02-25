# Sbbw TODO list

This list contains tasks currently under development, as well as tasks that are still in the backlog. I kindly request you to review the task list to catch up as much work progress as possible. Should you be interested in contributing, your participation would be greatly appreciated.

### Contributing

- **ssbw**: ssbw's main daemon manages all the currently running widgets by starting/closing them through commands or key shortcuts.
- **sbbw-exec**: It handles widget startup, as well, generating the lockfile (.lock) and autostarting.
- **sbbw-widget**: The widget's binary that renders and coordinates the widget's web interface with built-in commands (such as detecting the battery percentage or handling the multimedia interface).
- **sbbw-widget-conf**: Contains the widget's configuration manifest file and its self-export content & validations

### TODO

- [ ] MacOS
    - [ ] Implement support for macOS's pin window on top feature
    - [ ] Implement brightness handling
    - [ ] Implement media control handling
    - [ ] Implement wifi interface support (to get network data)

- [ ] Windows
    - [ ] Implement media control handling
    - [ ] Implement wifi interface support (to get network data)

- [ ] Linux
    - [ ] TODO (xd)

- [ ] General
    - [ ] Implement shortcut detection & widget's configurable shortcut loading into sbbw daemon
    - [ ] Implement widget multi-instance support
    - [ ] Implement widget comunication system
    - [ ] Implement nested widget calling/launching
    - [ ] Implement widget closure handling (daemon should approve when widget's about to close) in sbbw daemon (It regards to the task below)
    - [ ] SPIKE: Look into finding better approaches for executing commands by using key shortcuts.

- [ ] NodeJs Module
    - [ ] Fix module issues (Calling some functions might end in unexpected behavior)

### In Progress

- [ ] Code refactor

### Done ✓

- [x] ~~Widget unit testing using web tools~~
- [x] ~~Command to install widgets the easy way~~
- [x] ~~Javascript environment variables per widget (such as as OS information, Widget Name, and more)~~
- [x] ~~NodeJS module package bridge api~~
- [x] ~~implement built-in media controller (linux-only at the moment)~~
- [x] ~~Implement support for common commands (bat, brightness, sys_info, widget, media, wifi) natively~~
- [x] ~~Sbbw daemon detecting shortcuts and widgets configurable shortcuts~~

### Rejected

- [x] ~~Lua native support~~
