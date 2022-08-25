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
    general,
}
