import { invoke } from "./sbbw"

type SbbwExecParams = {
    /**
     * Executable file to call
     * > Note: the current directory for executable is the `{theme_name}/scripts`
     *
     * @example **python**
     */
    executable: string,
    /**
     * Array with arguments for executable
     *
     * @example **[ "main.py" ]**
     * @example **[ "-m", "pip", "install", "--user", "-r", "requirements" ]**
     */
    arguments: Array<string>,
}

const exec = (params: SbbwExecParams): Promise<string> =>
    invoke("exec", [params.executable, ...params.arguments])

export type { SbbwExecParams }
export { exec }
