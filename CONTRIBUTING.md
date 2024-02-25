# Sbbw Contributing Guide

### Contributing

- **ssbw**: ssbw's main daemon manages all the currently running widgets by starting/closing them through commands or key shortcuts.
- **sbbw-exec**: It handles widget startup, as well, generating the lockfile (.lock) and autostarting.
- **sbbw-widget**: The widget's binary that renders and coordinates the widget's web interface with built-in commands (such as detecting the battery percentage or handling the multimedia interface).
- **sbbw-widget-conf**: Contains the widget's configuration manifest file and its self-export content & validations
