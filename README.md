# Sbbw

Sbbww is the simplest and best way for making good-looking widgets & tools with web technologies. Based on `eww` and using [Wry](https://github.com/tauri-apps/wry) as rendering backend.

> **Note:** Each widget is configured independently within its associated folder.

![PreviewExamples](https://user-images.githubusercontent.com/56278796/153992032-82cf3c6a-f75a-475d-95ae-eb05ef6e21b5.gif)

## Examples

### [sidebar](https://github.com/SergioRibera/sidebar-sbbw-widget)
![Screenshot_20220214_230856](https://user-images.githubusercontent.com/56278796/153992067-6e8a2cd3-969c-4eb2-9325-ac688489f45f.png)
> Project made with [Vite](https://vitejs.dev) and [ReactJs](https://reactjs.org)

### [bottombar](https://github.com/SergioRibera/bottombar-sbbw-widget)
![image](https://user-images.githubusercontent.com/56278796/153992220-1445f40c-3ae3-4527-9df0-a4cc80e0b3ef.png)
> Project made with  [ReactJs](https://reactjs.org)

### [analog-clock](https://github.com/SergioRibera/analogclock-sbbw-widget)
![Screenshot_20220214_230947](https://user-images.githubusercontent.com/56278796/153992300-eab39961-0dbf-4d73-b3bc-a146377b1761.png)
> Project made with [ReactJs](https://reactjs.org)

## Features

- Supports all web frameworks
- Transparency support
- Fast performance
- Low memory usage
- Small-sized binaries
- Autostart/Installation script support
- Cross-Platform
- Widget testing using web tools
- And much more...

## Sbbw Usage

```sh
sbbw 0.1.3
Sergio Ribera
This serves as the launcher and manager for Sbbw Widgets.

USAGE:
    sbbw [OPTIONS] [SUBCOMMAND]

OPTIONS:
    -h, --help            Print help information
        --no-fork
    -p, --port <PORT>     [default: 8111]
    -s, --show-windows
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
## Installation
By being platform-agnostic, you'll need only to download the zip file from the releases page [here](https://github.com/SergioRibera/sbbw/releases) and uncompress the file for performing the setup according to your operating system.

**Windows**
> Just right-click `setup.bat` and run as Administrator.

**Linux/Mac**
> Open your terminal in the uncompressed folder path and run:
> ```sh
> sudo setup.sh
> ```



## Widget folder structure
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

The `~` stands for your user directory
| SO | Value | Example |
|--|--|--|
| Linux | `$XDG_CONFIG_HOME`  or  `$HOME`/.config | /home/sergioribera/.config |
| Windows | `{FOLDERID_RoamingAppData}` | C:\Users\SergioRibera\AppData\Roaming |
| MacOS | `$HOME`/Library/Application Support | /Users/SergioRibera/Library/Application Support |


> **Notes:** A basic folder should be created by the setup script. Otherwise, create it manually; the folder is required because of being the place where all sbbw widgets must be placed in.


**Folders structure & details**
| Name | Details |
|--|--|
| widget_name | Your widget's root path, the folder's name is used by sbbw for starting your widget |
| ui | Here is located all web-compiled or raw files for your widget interface, **there's a note below with further information about widget interfaces** |
| autostart | This folder encloses all required files to handle your widget's autostart lifecycle, examples:: requirements.txt, `main.py`, or any other file that your widget relies into |
| scripts | Files you'll need to fetch or modify system data or events, such as increasing/decreasing brightness, battery percentage information, and much more |
| config.toml | Contains all your widget configuration |

> **Note:** These folders and the config.toml file are required for launching sbbw and displaying your widget

> **UI-related note:** When creating a project using, vite, react, vue, or any framework, you'll need to set the homepage or basepath like this `widget_name/ui`, the `/ui` path plays an important role for your widget to function correctly

## Configuration
The configuration struct schema looks like this:
```rust
pub struct WidgetConfig {
    pub name: String,
    pub class_name: String,
    pub width: WidgetSize,
    pub height: WidgetSize,
    pub x: f32,
    pub y: f32,
    pub transparent: bool,
    pub blur: bool, // Only works for Windows and Mac, linux users can set this from the compositor-side.
    pub always_on_top: bool,
    pub stick: bool,
    pub autostart: Vec<AutoStartCommand>
}
```
**Explanation**
| Name | Default | Type | Description |
|--|--|--|--|
| name | Internal | String | Your widget's name, shown on top of the widget's title |
| class_name | Internal_class | String | Linux-only, it's a role plus name, like `{name}_{class_name}` |
| width | 200.0 | f64, Max | Your widget's width |
| height | Max | f64, Max | Your widget's height |
| x | 0.0 | f32 | Defines your widget's position in the X axis |
| y | 0.0 | f32 | Defines your widget's position in the Y axis |
| transparent | true | bool | Enables transparency by default when your widget is starting |
| blur | true | bool | Enables widget blur; **only works in MacOS and Windows** |
| always_on_top | true | bool | Defines if the widget will be placed always on top of other applications/widgets when spawning |
| stick | true | bool | Defines a widget as a persistent window that goes throughout your workspaces, **At the moment, it's only working on Linux and soonly it'll work in MacOS also** |
| autostart | &[] | Vec<AutoStartCommand> | List of commands to execute when the first sbbw daemon is being launched; only executed when any file inside the `autostart` folder or your`config.toml` changed. Before sbbw executing this list of arguments, sbbw will create a `config.lock` file (In case of sharing your widget, you'll need to ignore this file) |

**Example**
```toml
name = "sidebar"
# Snake_case is accepted
class_name = "class_name"
transparent = true
# Single variable names should be lowercased. Otherwise, snake_case naming should be used.
alwaysontop = true
stick = true
blur = false
width = "400.0"
# In both width & height the case of "Max" is always ignored
height = "mAx"
x = 0.1
y = -850.0

# This upgrades pip and install all the widget's requirements on ./autostart
# where ./autostart is the autostart root path,
# You can check it by executing`echo "$PWD"`, the outcome will be {WIDGET_PATH}/autostart
autostart = [
    { cmd = "python", args = [ "-m", "ensurepip", "--upgrade" ] },
    { cmd = "python", args = [ "-m", "pip", "install", "-r", "requirements.txt" ] },
    { cmd = "python", args = [ "main.py" ] },
]
```

>**FYI**: **Autostart parametter** is command list requiring only two parameters per command:

| Name | Description |
|--|--|
| cmd | **Command for executing** whether file being a binary or local one inside the `autostart` folder, so strings like "python", "echo", "ls", "./main.py", "./script.sh", "node" are accepted |
| args | List of strings, where each string is an argument for `cmd` |


> **Note:** Both `autostart` and `script` folders will behave the same, but `autostart` is only executed when starting **(or if the content files in autostart or config.toml have changed)** the sbbw daemon. `script` execution is determined by ui calls

## Widget NodeJs Module
See more details on [wiki page](https://github.com/SergioRibera/sbbw/wiki)

### TODO
> FYI: Moved to [TODO.md](TODO.md)
