# Sbbw

This is a Simple and best tool for made Widgets and tools very easy with Web technologies and generate awesome interfaces. Based on `eww` and using [Wry](https://github.com/tauri-apps/wry) as a main window rendering.

![PreviewExamples](https://user-images.githubusercontent.com/56278796/153992032-82cf3c6a-f75a-475d-95ae-eb05ef6e21b5.gif)

## Examples
### [sidebar](https://github.com/SergioRibera/sidebar-sbbw-widget)
![Screenshot_20220214_230856](https://user-images.githubusercontent.com/56278796/153992067-6e8a2cd3-969c-4eb2-9325-ac688489f45f.png)
> This project is made with [Vite](https://vitejs.dev) and [ReactJs](https://reactjs.org)

### [bottombar](https://github.com/SergioRibera/bottombar-sbbw-widget)
![image](https://user-images.githubusercontent.com/56278796/153992220-1445f40c-3ae3-4527-9df0-a4cc80e0b3ef.png)
> This project is made with  [ReactJs](https://reactjs.org)

### [analog-clock](https://github.com/SergioRibera/analogclock-sbbw-widget)
![Screenshot_20220214_230947](https://user-images.githubusercontent.com/56278796/153992300-eab39961-0dbf-4d73-b3bc-a146377b1761.png)
> This project is made with [ReactJs](https://reactjs.org)

## Features
- All Web Frameworks supported
- Transparency
- Very fast
- Low cost
- Very Small
- Autostart/Installation script support
- Cross Platform
- Test your widget using a web tools
- more...
## Sbbw Usage
```sh
Sbbw Daemon 0.1.0
Sergio Ribera
This is the launcher and manager for the Sbbw Wigets

USAGE:
    sbbw [OPTIONS]

OPTIONS:
    -c, --close <close>                        Close the widget [possible values: sidebar, bottom-bar, analog-clock]
        --check-config <check-config>          Check config of the widget [possible values: sidebar, bottom-bar, analog-clock]
    -h, --help                                 Print help information
    -o, --open <open>                          Open the widget [possible values: sidebar, bottom-bar, analog-clock]
    -p, --port <PORT>                          Port to listen on [default: 8111]
        --show-windows                         Show all widgets installeds
    -t, --toggle <toggle>                      Toggle view the widget [possible values: sidebar, bottom-bar, analog-clock]
        --test <widget_name> <local_server>    Test the widget
    -V, --version                              Print version information
```

## Widget folder struct
```sh
~
└─ .config
	    └─ sbbw
		        └─ widgets
			               └─ widget_name
				       ├─ ui
				       │     └─ index.html
				       ├─ autostart
				       │      └─ *
				       ├─ scripts
				       │      └─ *
				       └─ config.toml
```

The `~` home is variable on each operative system
| SO | Value | Example |
|--|--|--|
| Linux | `$XDG_CONFIG_HOME`  or  `$HOME`/.config | /home/sergioribera/.config |
| Windows | `{FOLDERID_RoamingAppData}` | C:\Users\SergioRibera\AppData\Roaming |
| MacOS | `$HOME`/Library/Application Support | /Users/SergioRibera/Library/Application Support |

> **Notes:** on here folder you need create a basic folder (or it's created anyways), but here is location where all widgets have stay

**Folders Explain**
| Name | Details |
|--|--|
| widget_name | This is a root of all files for your widget and this name is used by sbbw to call this |
| ui | On here is locate all web compiled or raw files for interface, notes are more Down with more details |
| autostart | On this folder is located all files what you need to run autostart commands, example: requirements.txt, `main.py`, or any you consider needed |
| scripts | Are a files you will need to get and modify system data, like a brightness, battery info, and more |
| config.toml | This file contains all configuration for show your widget |

> **Note:** all this folders and file is extricted required for launch sbbw and show this plugin

> **Other Note very important:** When you create a proyect using, vite, react, vue, any framework, you need set the homepage or basepath like `widget_name/ui`, `/ui` is very important for work correctly
## Configuration
The struct of the configuration is this
```rust
pub struct WidgetConfig {
    pub name: String,
    pub class_name: String,
    pub width: WidgetSize,
    pub height: WidgetSize,
    pub x: f32,
    pub y: f32,
    pub transparent: bool,
    pub blur: bool, // Only works on Windows and Mac. For the linux users can be set with compositor
    pub always_on_top: bool,
    pub stick: bool,
    pub autostart: Vec<AutoStartCommand>
}
```
**Explanation**
| Name | Default | Type | Description |
|--|--|--|--|
| name | Internal | String | This is a name of widget, this showed on name of window |
| class_name | Internal_class | String | This is only for linux, and this in reallity is a role but plus name, like this `{name}_{class_name}` |
| width | 200.0 | f64, Max | This define the width of widget |
| height | Max | f64, Max | This define the height of widget |
| x | 0.0 | f32 | This define a position in X of widget |
| y | 0.0 | f32 | This define a position in Y of widget |
| transparent | true | bool | This enable a transparency by default on start widget |
| blur | true | bool | This set a widget window as blurred, **This only works on MacOS and Windows** |
| always_on_top | true | bool | This define if always on top of other applications or widgets (in order of spawning) |
| stick | true | bool | This define widget as a persistent window on all workspaces, **For now, only works on Linux and soon on MacOS** |
| autostart | &[] | Vec<AutoStartCommand> | This is a list of commands to excecute on launch the first daemon of sbbw, but this only is executed if any file on `autostart` folder or `config.toml` are changed, and before execute all list, sbbw create a `config.lock` file (if you want share your widget you need ignore this `config.lock` file) |

**Example**
```toml
name = "sidebar"
# Snake_case is acepted
class_name = "class_name"
transparent = true
# On all variable names, the case is lowercase but accept snake_case
alwaysontop = true
stick = true
blur = false
width = "400.0"
# on width or height the case of "Max" is ignored
height = "mAx"
x = 0.1
y = -850.0

# This commands upgrade pip and install requirements on ./autostart
# where ./autostart is a root of subprocess command,
# so if you execute a `echo "$PWD"` the result is a {WIDGET_PATH}/autostart
autostart = [
    { cmd = "python", args = [ "-m", "ensurepip", "--upgrade" ] },
    { cmd = "python", args = [ "-m", "pip", "install", "-r", "requirements.txt" ] },
    { cmd = "python", args = [ "main.py" ] },
]
```

**Details of Autostart parametter**
This is a list of commands, but this only have two parametters:
| Name | Description |
|--|--|
| cmd | This is a command for execute, can be are a binary or local file on `autostart` folder, so is accepted strings like this "python", "echo", "ls", "./main.py", "./script.sh", "node" |
| args | This is a list of strings, where each string is a argument for `cmd` |


> **Note:** the `autostart` folder and `script` folder have a equals behaviour, but in other moment and context, the `autostart` is only executed on start **(if autostart content files or config.toml have changes)** daemon and `script` executed is determined by ui calls

### Developing UI Javascript methods
- executeCommand(cmd, args)
	- This return a Promise with data as object
	- params: 
		- cmd: Binary or file to execute on `scripts` folder
		- args: List of strings, where is specified the arguments for `cmd`
	- return: Return a promise where if
		- then: return a raw output of command
		- catch: return an object with
			- code: Code based on Http responses (this not related with exit code of command, is a totally refered to sbbw response), where 404 can be are a `command not found`
			- data: this is a raw data of the output of command

### TODO
- [ ] MacOS Stick windows Support
- [x] Test widget (using web tools)
- [ ] Lua support natively (For reject??)
- [ ] More Javascript Methods (On Demmand)
- [ ] Implement common commands natively
- [ ] Refactor Code
- [ ] Your Suggestion :D
		 
