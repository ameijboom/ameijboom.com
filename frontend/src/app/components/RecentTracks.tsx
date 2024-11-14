'use client'

import useSWR, { Fetcher } from "swr"
import { RecentTrack } from "../lib/tracks"
import Track from "./Track"

export default function RecentTracks() {
    const fetcher: Fetcher<RecentTrack[], string> = async (url) => (await fetch(url)).json()
    const url = `${process.env.NEXT_PUBLIC_MUSIC_API_URL}/api/v1/recent`
    const { data, error, isLoading } = useSWR<RecentTrack[], Error>(url, fetcher, { refreshInterval: 5000 })

    return(
        <div className="p-5">
            {isLoading &&
            <div className="flex justify-center">
                <span className="loading loading-ring loading-lg"></span>
            </div>
            }

            {error &&
            <div className="bg-error flex justify-center px-4 py-16">An error occured!</div>
            }

            {(data && !isLoading && !error) && <Track tracks={data} />}
        </div>
    );
}