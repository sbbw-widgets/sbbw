import { invoke } from "./sbbw";

type SbbwSysInfo = {
	/**
	 * Returns the system hostname based out of the DNS
	 */
	hostname: string;
	/**
	 * Returns system uptime (in seconds)
	 */
	uptime: number;
	/**
	 * Returns the system’s kernel version
	 */
	kernel_version: string;
	/**
	 * Returns the system version (e.g. for MacOS this will return 11.1 rather than the kernel version)
	 */
	os_version: string;
	/**
	 * Returns the system long os version (e.g “MacOS 11.2 BigSur”)
	 */
	long_os_version: string;
	/**
	 *
	 */
	users: Array<SbbwSysInfoUser>;
};

type SbbwSysInfoUser = {
	/**
	 * Return the user's id
	 */
	id: string;
	/**
	 * Return the group id of the user
	 * > Note: On Windows, this value defaults to 0. Windows doesn’t have a username specific group assigned to the user
	 */
	group_id: string;
	/**
	 * Returns the name of the user
	 */
	name: string;
	/**
	 * Returns the groups of the user
	 */
	groups: Array<string>;
};

type SbbwSysInfoDisk = {
	/**
	 * Returns the disk name
	 */
	name: string;
	/**
	 * Returns the total disk size, in bytes
	 */
	total_space: number;
	/**
	 * Returns the available disk size, in bytes
	 */
	free_space: number;
	/**
	 * Returns true if the disk is removable
	 */
	is_removable: boolean;
	/**
	 * Returns the mount point of the disk
	 */
	mount_point: string;
	/**
	 * Returns the file system used on this disk
	 */
	file_system: string;
};

type SbbwSysInfoNetwork = {
	/**
	 * Returns the name of Network interface
	 */
	name: string;
	/**
	 * Returns the number of received bytes
	 */
	received: number;
	/**
	 * Returns the total number of received bytes
	 */
	total_received: number;
	/**
	 * Returns the number of transmitted bytes
	 */
	transmitted: number;
	/**
	 * Returns the total number of transmitted bytes
	 */
	total_transmitted: number;
	/**
	 * Returns the number of incoming packets
	 */
	packets_received: number;
	/**
	 * Returns the total number of incoming packets
	 */
	total_packets_received: number;
	/**
	 * Returns the number of outcoming packets
	 */
	packets_transmitted: number;
	/**
	 * Returns the total number of outcoming packets
	 */
	total_packets_transmitted: number;
	/**
	 * Returns the number of incoming errors since the last refresh
	 */
	errors_on_received: number;
	/**
	 * Returns the total number of incoming errors
	 */
	total_errors_on_received: number;
	/**
	 * Returns the number of outcoming errors since the last refresh
	 */
	errors_on_transmitted: number;
	/**
	 * Returns the total number of outcoming errors
	 */
	total_errors_on_transmitted: number;
};

type SbbwSysInfoMemory = {
	/**
	 * Returns the RAM size in KB
	 */
	total: number;
	/**
	 * Returns the amount of free RAM in KB
	 */
	free: number;
	/**
	 * Returns the amount of available RAM in KB
	 */
	aviable: number;
	/**
	 * Returns the amount of used RAM in KB
	 */
	used: number;
	/**
	 * Returns the SWAP size in KB
	 */
	swap_total: number;
	/**
	 * Returns the amount of free SWAP in KB
	 */
	swap_free: number;
	/**
	 * Returns the amount of used SWAP in KB
	 */
	swap_used: number;
};

type SbbwSysInfoCpu = {
	/**
	 * Returns this CPU’s usage
	 */
	cpu_usage: number;
	/**
	 * Returns this CPU’s name
	 */
	name: string;
	/**
	 * Returns the CPU’s vendor id
	 */
	vendor_id: string;
	/**
	 * Returns the CPU’s brand
	 */
	brand: string;
	/**
	 * Returns the CPU’s frequency
	 */
	frequency: number;
};

const getAllDisks = (): Promise<Array<SbbwSysInfoDisk>> =>
	invoke("sys.disks", null);

const getAllNetworks = (): Promise<Array<SbbwSysInfoNetwork>> =>
	invoke("sys.net", null);

const getSysInfo = (): Promise<Array<SbbwSysInfo>> => invoke("sys.info", null);

const getMemory = (): Promise<Array<SbbwSysInfoMemory>> =>
	invoke("sys.memory", null);

const getCpu = (): Promise<Array<SbbwSysInfoCpu>> => invoke("sys.cpu", null);

export type {
	SbbwSysInfo,
	SbbwSysInfoUser,
	SbbwSysInfoDisk,
	SbbwSysInfoNetwork,
	SbbwSysInfoMemory,
	SbbwSysInfoCpu,
};
export { getAllDisks, getAllNetworks, getSysInfo, getMemory, getCpu };
