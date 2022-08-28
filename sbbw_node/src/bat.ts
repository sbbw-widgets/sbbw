import { invoke } from "./sbbw"

type SbbwBatteryState = 'unknown' |
    'charging' | 'discharging' |
    'empty' | 'full'

type SbbwBatteryTechnology = 'Unknown' |
'LithiumIon' | 'LeadAcid' |
'LithiumPolymer' | 'NickelMetalHydride' |
'NickelCadmium' | 'NickelZinc' |
'LithiumIronPhosphate' | 'RechargeableAlkalineManganese'

type SbbwBattery = {
    /**
     * Battery vendor
     */
    vendor: string,
    /**
     * Battery model
     */
    model: string,
    /**
     * Battery serial number
     */
    serial: string,
    /**
     * Percent of energy currently available in the battery
     */
    percentage: number,
    /**
     * Amount of energy currently available in the battery
     */
    energy: number,
    /**
     * Amount of energy in the battery when it's considered full
     */
    energy_full: number,
    /**
     * Battery voltage
     */
    voltage: number,
    /**
     * Battery current state.
     */
    state: SbbwBatteryState | string,
    /**
     * Gets battery state of health
     * For more details. See also: [State of health](https://en.wikipedia.org/wiki/State_of_health) or [soh](https://www.mpoweruk.com/soh.htm)
     */
    health: number,
    /**
     * Battery technology
     */
    technology: SbbwBatteryTechnology | string,
    /**
     * Battery temperature
     */
    temperature: number,
    /**
     * Number of charge/discharge cycles
     */
    cycle_count: number,
    /**
     * Remaining time till full battery
     */
    time_to_full: number,
    /**
     * Remaining time till empty battery
     */
    time_to_empty: number,
}

/**
 * @returns A promise with count of batteries
 */
const getBatteryCounts = (): Promise<number> =>
    invoke("battery.counts", null)


/**
 * @returns A promise with array of all batteries data
 */
const getAllBatteries = (): Promise<Array<SbbwBattery>> =>
    invoke("battery.all", null)

/**
 * @returns A promise with array of all batteries data
 */
const getMainBattery = (): Promise<SbbwBattery> =>
    invoke("battery.main", null)

export type { SbbwBattery, SbbwBatteryState, SbbwBatteryTechnology }
export { getBatteryCounts, getAllBatteries, getMainBattery }
