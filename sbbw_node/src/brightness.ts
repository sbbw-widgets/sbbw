import { invoke } from "./sbbw"

type SbbwBrightnessDevice = {
    name: string,
    value: number,
}

/**
 * @returns A promise with array of all batteries data
 */
const getBrightness = (): Promise<SbbwBrightnessDevice> =>
    invoke("brightness.main", null)

const getAllBrightness = (): Promise<Array<SbbwBrightnessDevice>> =>
    invoke("brightness.all", null)

const setBrightness = (value: number): Promise<void | string> =>
    invoke("brightness.set_main", value)

const setAllBrightness = (value: number): Promise<void | string> =>
    invoke("brightness.set_all", value)

export type { SbbwBrightnessDevice }
export { getBrightness, getAllBrightness, setBrightness, setAllBrightness }
