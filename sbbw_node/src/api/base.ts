type SbbwExecParams = {
    executable: string,
    arguments: Array<string>,
}

const exec = (params: SbbwExecParams): Promise<string> =>
    window.rpc.call("exec", [params.executable, ...params.arguments])

export type { SbbwExecParams }
export { exec }
