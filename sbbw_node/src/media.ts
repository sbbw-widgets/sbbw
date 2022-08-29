import { invoke } from "./sbbw"

type SbbwMediaMetadata = {
    /*
     * The track ID
     */
    track_id: string,
    /*
     * The track name
     */
    title: string,
    /*
     * The name of the album the track appears on
     */
    album_name: string,
    /*
     * A list of artists of the album the track appears on
     */
    album_artists: Array<string>,
    /*
     * A list of artists of the track
     */
    artists: Array<string>,
    /*
     * An URL to album art of the current track
     */
    art_url: string | null,
    /*
     * The duration of the track, in microseconds
     */
    track_length: number | null,
}

type SbbwMediaState = {
    /*
     * Id of player, (usually something like :1.1337)
     * > Note: Only on linux
     */
    id: string,
    /*
     * This is usually the application's name, like Spotify
     */
    player_name: string,
    /*
     * This contains data for current player an track
     */
    metadata: SbbwMediaMetadata | null,
    /*
     * A player volume
     * Volume should be between 0.0 and 1.0
     */
    volume: number | null,
    /*
     * Track progress on microseconds
     */
    track_progress: number,
    /*
     * True if the player has enable the "Shuffle" mode
     */
    shuffle: boolean,
}


const setPlayPause = (play_pause: boolean): Promise<void | string> =>
    invoke("media.play_pause", play_pause)

const setNext = (): Promise<SbbwMediaState> =>
    invoke("media.next", null)

const setPrev = (): Promise<SbbwMediaState> =>
    invoke("media.prev", null)

const setVolume = (volume: number): Promise<void | string> =>
    invoke("media.set_volume", volume)

const getState = (): Promise<SbbwMediaState> =>
    invoke("media.state", null)

const getVolume = (): Promise<number> =>
    invoke("media.get_volume", null)

const isPlayerActive = (): Promise<boolean> =>
    invoke("media.active", null)

export type { SbbwMediaState, SbbwMediaMetadata }
export { setPlayPause, setNext, setPrev, setVolume, getState, getVolume, isPlayerActive }
