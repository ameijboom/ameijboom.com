import Image from "next/image";
import { RecentTrack } from "../lib/tracks";
import Link from "next/link";

type Props = {
  tracks: RecentTrack[];
};

function hex2rgb(hex: string): number[] {
    const r = parseInt(hex.slice(1, 3), 16);
    const g = parseInt(hex.slice(3, 5), 16);
    const b = parseInt(hex.slice(5, 7), 16);
    
    return [r,g,b]  
}

export default function Track({ tracks }: Props) {
  const current = tracks[0];
  const rgb = hex2rgb(`#${current.attributes.artwork.bgColor}`)
  const brightness = Math.round(((rgb[0] * 299) + (rgb[1] * 587) + (rgb[2] * 114)) / 1000)

  console.log(brightness)

  return (
    <div className={`card card-side ${ brightness > 140 ? 'bg-base-200' : 'bg-base-content'} shadow-xl max-w-4xl min-w-4xl`}>
      <figure>
        <Image
          src={current.attributes.artwork.url.replace(/({.})/g, "256")}
          alt={current.attributes.albumName}
          width={256}
          height={256}
        />
      </figure>
      <div className={`card-body ${ brightness < 140 ? 'text-base-200' : 'text-gray-200'}`}>
      <Link target="_blank" href={current.attributes.url} className="card-title" style={{
        color: `#${current.attributes.artwork.bgColor}`
    }}>
            {current.attributes.name}
          </Link>
        <h5 className="italic">{current.attributes.albumName}</h5>
        <p>by {current.attributes.artistName}</p>
        <div className="card-actions justify-end">

        </div>
      </div>
    </div>
  );
}
