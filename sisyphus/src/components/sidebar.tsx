"use client"

import type React from "react"

import { useState } from "react"
import Link from "next/link"
import { usePathname } from "next/navigation"
import {
  BarChart2,
  Dumbbell,
  Clock,
  Utensils,
  MessageSquare,
  Building2,
  Calendar,
  Users,
  Settings,
  ChevronRight,
  ChevronLeft,
} from "lucide-react"
import { cn } from "@/lib/utils"
import Image from "next/image"

type NavItem = {
  title: string
  href: string
  icon: React.ReactNode
}

const commonNavItems: NavItem[] = [
  {
    title: "My Data & Progress",
    href: "/data-progress",
    icon: <BarChart2 className="h-5 w-5" />,
  },
  {
    title: "My Exercise Routines",
    href: "/exercise-routines",
    icon: <Dumbbell className="h-5 w-5" />,
  },
  {
    title: "My Training Sessions",
    href: "/training-sessions",
    icon: <Clock className="h-5 w-5" />,
  },
  {
    title: "My Nutrition Plan",
    href: "/nutrition-plan",
    icon: <Utensils className="h-5 w-5" />,
  },
  {
    title: "Chats",
    href: "/chats",
    icon: <MessageSquare className="h-5 w-5" />,
  },
]

const trainerNavItems: NavItem[] = [
  {
    title: "My Gym",
    href: "/gym",
    icon: <Building2 className="h-5 w-5" />,
  },
  {
    title: "My Schedules",
    href: "/schedules",
    icon: <Calendar className="h-5 w-5" />,
  },
  {
    title: "My Patients",
    href: "/patients",
    icon: <Users className="h-5 w-5" />,
  },
]

export default function Sidebar() {
  const [expanded, setExpanded] = useState(true)
  const pathname = usePathname()

  return (

       <div
      className={cn(
        "flex flex-col h-screen hidden md:block bg-[#0a0e17] border-r border-[#1a2033] transition-all duration-300",
        expanded ? "w-64" : "w-16", 
      )}
    >
      {/* Logo at the top */}
      <div className="flex items-center justify-center h-24 border-b border-[#1a2033]">
        <Image
          src="/logo.png"
          alt="The Sisyphus"
          width={expanded ? 96 : 48}
          height={expanded ? 93 : 45}
          className="object-contain transition-all duration-300"
        />
      </div>

      <div className="flex items-center justify-between h-12 px-4 border-b border-[#1a2033]">
        {expanded ? (
          <Link href="/dashboard" className="flex items-center gap-2 text-cyan-400 font-bold text-xl">
            <span className="text-cyan-400">FT</span>
            <span>Fitness Trainer</span>
          </Link>
        ) : (
          <Link href="/dashboard" className="text-cyan-400 font-bold text-xl">
            FT
          </Link>
        )}

        <button
          onClick={() => setExpanded(!expanded)}
          className="flex items-center justify-center p-1.5 rounded-md text-gray-300 hover:bg-[#1a2033] hover:text-white transition-colors"
        >
          {expanded ? <ChevronLeft className="h-5 w-5" /> : <ChevronRight className="h-5 w-5" />}
        </button>
      </div>

      <div className="flex-1 overflow-y-auto py-4 flex flex-col gap-2">
        <div className="px-3">
          {expanded && <h3 className="text-xs uppercase text-gray-400 font-semibold px-2 mb-2">Patient Menu</h3>}
          <nav className="space-y-1">
            {commonNavItems.map((item) => (
              <Link
                key={item.href}
                href={item.href}
                className={cn(
                  "flex items-center gap-3 rounded-md px-3 py-2 text-sm transition-colors",
                  pathname === item.href
                    ? "bg-gradient-to-r from-cyan-600 to-green-500 text-white"
                    : "text-gray-300 hover:bg-[#1a2033] hover:text-white",
                )}
              >
                <span className="flex-shrink-0">{item.icon}</span>
                {expanded && <span>{item.title}</span>}
              </Link>
            ))}
          </nav>
        </div>

        <div className="px-3 mt-6">
          {expanded && <h3 className="text-xs uppercase text-gray-400 font-semibold px-2 mb-2">PT Menu</h3>}
          <nav className="space-y-1">
            {trainerNavItems.map((item) => (
              <Link
                key={item.href}
                href={item.href}
                className={cn(
                  "flex items-center gap-3 rounded-md px-3 py-2 text-sm transition-colors",
                  pathname === item.href
                    ? "bg-gradient-to-r from-cyan-600 to-green-500 text-white"
                    : "text-gray-300 hover:bg-[#1a2033] hover:text-white",
                )}
              >
                <span className="flex-shrink-0">{item.icon}</span>
                {expanded && <span>{item.title}</span>}
              </Link>
            ))}
          </nav>
        </div>
      </div>

      <div className="p-2 mt-auto border-t border-[#1a2033]">
        <Link
          href="/settings"
          className={cn(
            "flex items-center gap-3 rounded-md px-3 py-2 text-sm transition-colors",
            pathname === "/settings"
              ? "bg-gradient-to-r from-cyan-600 to-green-500 text-white"
              : "text-gray-300 hover:bg-[#1a2033] hover:text-white",
          )}
        >
          <Settings className="h-5 w-5" />
          {expanded && <span>Settings</span>}
        </Link>
      </div>
    </div>
  )
}
