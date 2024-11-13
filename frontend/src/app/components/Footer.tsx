import GitHubIcon from '@mui/icons-material/GitHub';
import LinkedInIcon from '@mui/icons-material/LinkedIn';
import ApartmentIcon from '@mui/icons-material/Apartment';

export default function Footer() {
    return (
        <footer className="footer bg-neutral text-neutral-content items-center p-4 sticky">
            <p>Made with Next & DaisyUI</p>

            <nav className="grid-flow-col gap-4 md:place-self-center md:justify-self-end">
                <a href="https://github.com/ameijboom/" target="_blank">
                    <GitHubIcon />
                </a>
                <a href="https://www.linkedin.com/in/alwin-meijboom-94908b154/" target="_blank">
                    <LinkedInIcon />
                </a>
                <a href="https://brainhive.nl/" target="_blank">
                    <ApartmentIcon />
                </a>
            </nav>
        </footer>
    )
}
