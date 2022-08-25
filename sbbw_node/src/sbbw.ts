/** @ignore */
declare global {
    interface Window {
        rpc: {
            call: (method: string, args: Array<string>) => Promise<any>
        },
        general: {
            os: string,
            os_arch: string,
        }
    }
}

type GenericData = {
    [key: string]: any
}

type SbbwResponseError = {
    /**
     * Use the http codes to define if there was success or not.
     */
    code: number,
    /**
     * string if is an error
     * GenericData if is success
     */
    data: string | GenericData
}

const invoke = (method: string, args: Array<string>): Promise<any> =>
    window.rpc.call(method, args)

export type { GenericData, SbbwResponseError }
export { invoke }
