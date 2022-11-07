# Sbbw TODO list
In this list you will find all the tasks that are being developed and those that are still to start, I invite you to see each one and see if you can collaborate in any way in any of them

### Contributing
- **sbbw**: this is the daemon, it is in charge of managing the widgets that are running, it is in charge of launching and closing the widgets based on commands or shortcuts.
- **sbbw-exec**: It is in charge of doing what is related to the first launching of the widget, such as generating the .lock or executing the autostart.
- **sbbw-widget**: It is the binary of the widget, this is the one that renders and communicates the web interface with the built-in functions, like detecting the battery, or controlling the multimedia.
- **sbbw-widget-conf**: Contains everything related to the widget manifest file and the sbbw configuration structures in general, as well as exports and validation.

### TODO

- [ ] MacOS
    - [ ] Stick windows Support
    - [ ] Implement brightness
    - [ ] Implement media controll
    - [ ] Implement wifi (get data)

- [ ] Windows
    - [ ] Implement media controll
    - [ ] Implement wifi (get data)

- [ ] Linux
    - [ ] TODO (xd)

- [ ] General
    - [ ] Sbbw daemon detect shortcuts and widgets configurable shortcuts
    - [ ] Implement support for multiple widgets of the same open type
    - [ ] Comunication between widgets
    - [ ] A widget can launch another widget and get a response from it
    - [ ] Widget request exit to daemon and daemon aprove close (Related to the above)
    - [ ] Find better solution for launch command using shortcuts

- [ ] NodeJs Module
    - [ ] Fix issues using the module (When some functions are called, they do not have an expected behavior)

### In Progress

- [ ] ~~Refactor Code~~

### Done ✓

- [x] ~~Test widget (using web tools)~~
- [x] ~~Command to install widget easiest~~
- [x] ~~Javascript variables, like as SO, Widget Name, and more~~
- [x] ~~Nodejs module bridgen api~~
- [x] ~~implement media controller built-in (linux)~~
- [x] ~~Implement common commands natively (bat, brightness, sys_info, widget, media, wifi)~~
- [x] ~~Sbbw daemon detect shortcuts and widgets configurable shortcuts~~

### Rejected

- [x] ~~Lua support natively~~
