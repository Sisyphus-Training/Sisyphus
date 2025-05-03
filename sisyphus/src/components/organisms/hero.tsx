"use client"

import { Avatar, AvatarFallback, AvatarImage } from "@/components/atoms/avatar"
import { Badge } from "@/components/atoms/badge"
import { Input } from "../atoms/input"

export function HeroSection({ isPatientMode = false }) {

  return (
    <div className="mb-8 p-6 bg-[#0a0e17] rounded-lg shadow-md">
      <div className="h-32 bg-gradient-to-r from-green-500 to-blue-500 rounded-lg mb-6 relative overflow-hidden">
      </div>

      <div className="flex flex-col md:flex-row md:items-center md:justify-between gap-4">
        <div className="flex items-center gap-4">
          <Avatar className="w-20 h-20 border-4 border-[#070b14]">
            <AvatarImage
              src={isPatientMode ? "/patient.jpg" : "/passport.webp"}
              alt={isPatientMode ? "Patient" : "Trainer"}
              className="object-cover"
            />
            <AvatarFallback className="bg-[#172a46] text-lg">{isPatientMode ? "JS" : "JD"}</AvatarFallback>
          </Avatar>

          <div>
            <div className="flex-row items-center gap-2">
              <h1 className="text-2xl font-bold">{isPatientMode ? "Jane Smith" : "John Doe"}</h1>
              <Badge className="bg-green-500 hover:bg-green-600">
                {isPatientMode ? "Premium Member" : "Certified Trainer"}
              </Badge>
            </div>
            <p className="text-gray-400">
              {isPatientMode ? "Member since January 2023" : "Certified Personal Trainer"}
            </p>
          </div>
        </div>

        <div className="flex items-center gap-2">
          {/* Search only visible on larger screens */}
          <div className="relative hidden md:block">
            <Input
              type="search"
              placeholder={isPatientMode ? "Search exercises..." : "Search patients..."}
              className="pl-3 w-[200px] md:w-[250px] bg-[#0a0e17] border-[#1a2033]"
            />
          </div>

          
        </div>
      </div>
    </div>
  )
}
