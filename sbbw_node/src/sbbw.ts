/** @ignore */
declare global {
    interface Window {
        rpc: {
            call: (method: string, data: any) => Promise<any>
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

const invoke = (method: string, data: any): Promise<any> =>
    window.rpc.call(method, data)

export type { GenericData, SbbwResponseError }
export { invoke }
