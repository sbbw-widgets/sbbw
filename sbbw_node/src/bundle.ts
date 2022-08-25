import 'regenerator-runtime/runtime'
import * as sbbw from './sbbw';
import * as battery from './bat';
import * as base from './base';
import * as sysinfo from './sysinfo';
import * as widget from './widget';

export type SbbwGeneral = {
    /**
     * Describe the specific operating system in use, based on [rust doc](https://doc.rust-lang.org/std/env/consts/constant.OS.html)
     */
    os: string,
    /**
     * Describe the specific architecture of the CPU that is currently in use, based on [rust doc](https://doc.rust-lang.org/std/env/consts/constant.ARCH.html)
     */
    arch: string,
}

const general: SbbwGeneral = {
    os: window.general.os || "",
    arch: window.general.os_arch || "",
}

export {
    sbbw,
    general,
    battery,
    base,
    sysinfo,
    widget
}
