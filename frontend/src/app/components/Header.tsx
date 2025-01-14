import Link from "next/link";

export default function Header() {
    return (
        <nav className="navbar bg-base-200">
            <div className="flex-1">
                <Link className="btn text-xl" href="/">Alwin Meijboom</Link>
            </div>

            <div className="flex-none">
                <ul className="menu menu-horizontal px-1">
                    <li><Link href="/resume">Resume</Link></li>
                    <li><Link href="/about-me">About Me</Link></li>
                    <li><Link href="/projects">Projects</Link></li>
                </ul>
            </div>
        </nav>
    );
}
