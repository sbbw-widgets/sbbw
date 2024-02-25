import { invoke } from "./sbbw";

type SbbwMediaMetadata = {
	/*
	 * The track ID
	 */
	track_id: string;
	/*
	 * The track name
	 */
	title: string;
	/*
	 * The name of the album on which the track appears
	 */
	album_name: string;
	/*
	 * A list of artists on the album where the track appears
	 */
	album_artists: Array<string>;
	/*
	 * A list of artists for the track
	 */
	artists: Array<string>;
	/*
	 * A URL to the album art of the current track
	 */
	art_url: string | null;
	/*
	 * The track duration, in microseconds
	 */
	track_length: number | null;
};

type SbbwMediaState = {
	/*
	 * Player Id, (e.g. 1.1337)
	 * > Note: Linux-only
	 */
	id: string;
	/*
	 * Application's name (e.g. Spotify)
	 */
	player_name: string;
	/*
	 * This contains the metadata for the current media player and playing track
	 */
	metadata: SbbwMediaMetadata | null;
	/*
	 * Player volume
	 * Volume should be between 0.0 and 1.0
	 */
	volume: number | null;
	/*
	 * Track progress in microseconds
	 */
	track_progress: number;
	/*
	 * True if player has enabled the "Shuffle" mode
	 */
	shuffle: boolean;
};

const setPlayPause = (play_pause: boolean): Promise<void | string> =>
	invoke("media.play_pause", play_pause);

const setNext = (): Promise<SbbwMediaState> => invoke("media.next", null);

const setPrev = (): Promise<SbbwMediaState> => invoke("media.prev", null);

const setVolume = (volume: number): Promise<void | string> =>
	invoke("media.set_volume", volume);

const getState = (): Promise<SbbwMediaState> => invoke("media.state", null);

const getVolume = (): Promise<number> => invoke("media.get_volume", null);

const isPlayerActive = (): Promise<boolean> => invoke("media.active", null);

export type { SbbwMediaState, SbbwMediaMetadata };
export {
	setPlayPause,
	setNext,
	setPrev,
	setVolume,
	getState,
	getVolume,
	isPlayerActive,
};
