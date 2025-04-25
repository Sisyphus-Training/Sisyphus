import type { Metadata } from "next"
import { Inter } from "next/font/google"
import Sidebar from "@/components/sidebar"
import HeroSection from "@/components/organisms/hero"
import FabMenu from "@/components/organisms/fixedFloatingActionButton"

const inter = Inter({ subsets: ["latin"] })

export const metadata: Metadata = {
  title: "Fitness Trainer Dashboard",
  description: "Fitness trainer dashboard with expandable sidebar",
}

export default function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
    <html lang="en">
      <body className={inter.className}>
        <div className="flex h-screen bg-[#0a0e17] text-white">
          <Sidebar />
          <main className="flex-1 overflow-auto">
            <HeroSection />
            <FabMenu />
          </main>
        </div>
      </body>
    </html>
  )
}
