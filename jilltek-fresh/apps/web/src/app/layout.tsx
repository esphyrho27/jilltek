import type { Metadata } from "next";
import "./globals.css";
export const metadata: Metadata = {
  title: "Jilltek ISP Management",
  description: "Jilltek network and ISP operations platform"
};
export default function RootLayout({ children }: Readonly<{ children: React.ReactNode }>) {
  return <html lang="en"><body>{children}</body></html>;
}
