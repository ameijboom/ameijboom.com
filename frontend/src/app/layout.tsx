import localFont from 'next/font/local';

import "./globals.css";
import Header from "../components/Header";
import Footer from "../components/Footer";

const monaspaceNeon = localFont({
  src: [
    {
      path: '../fonts/MonaspaceNeon-Regular.woff2',
      weight: '400',
      style: 'normal'
    },
    {
      path: '../fonts/MonaspaceNeon-Italic.woff2',
      weight: '400',
      style: 'italic'
    },
    {
      path: '../fonts/MonaspaceNeon-Light.woff2',
      weight: '300',
      style: 'normal'
    },
    {
      path: '../fonts/MonaspaceNeon-LightItalic.woff2',
      weight: '300',
      style: 'italic'
    },
    {
      path: '../fonts/MonaspaceNeon-Bold.woff2',
      weight: '700',
      style: 'normal'
    },
    {
      path: '../fonts/MonaspaceNeon-BoldItalic.woff2',
      weight: '700',
      style: 'italic'
    },
  ],
  variable: '--font-monaspace-neon'
})

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en" className={`${monaspaceNeon.variable}`}>
      <body>
        <Header />
        {children}
        <Footer />
      </body>
    </html>
  );
}
