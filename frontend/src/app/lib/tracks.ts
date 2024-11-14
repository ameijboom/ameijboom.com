export interface TrackArtwork {
    bgColor: string;
    url: string;
}


export interface TrackAttributes {
    albumName: string;
    artistName: string;
    name: string;
    url: string;
    artwork: TrackArtwork;
}

export interface RecentTrack {
    attributes: TrackAttributes;
}