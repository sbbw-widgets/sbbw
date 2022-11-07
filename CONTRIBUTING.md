# Sbbw Contributing Guide

### Contributing
- **sbbw**: this is the daemon, it is in charge of managing the widgets that are running, it is in charge of launching and closing the widgets based on commands or shortcuts.
- **sbbw-exec**: It is in charge of doing what is related to the first launching of the widget, such as generating the .lock or executing the autostart.
- **sbbw-widget**: It is the binary of the widget, this is the one that renders and communicates the web interface with the built-in functions, like detecting the battery, or controlling the multimedia.
- **sbbw-widget-conf**: Contains everything related to the widget manifest file and the sbbw configuration structures in general, as well as exports and validation.
