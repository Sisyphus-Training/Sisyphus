"use client"

import { Activity, ArrowUpRight, Calendar, Users } from "lucide-react"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/atoms/card"

export function StatsCards() {
  return (
    <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4 md:gap-6">
      <Card className="bg-[#111827] border-[#1a2033]">
        <CardHeader className="flex flex-row items-center justify-between pb-2">
          <CardTitle className="text-sm font-medium">Total Patients</CardTitle>
          <div className="p-2 rounded-full bg-[#172a46] text-green-400">
            <Users className="h-5 w-5" />
          </div>
        </CardHeader>
        <CardContent>
          <div className="text-2xl font-bold">24</div>
          <div className="flex items-center mt-1 text-green-500 text-sm">
            <ArrowUpRight className="h-4 w-4 mr-1" />
            <span>+2 since last month</span>
          </div>
        </CardContent>
      </Card>

      <Card className="bg-[#111827] border-[#1a2033]">
        <CardHeader className="flex flex-row items-center justify-between pb-2">
          <CardTitle className="text-sm font-medium">Sessions This Week</CardTitle>
          <div className="p-2 rounded-full bg-[#172a46] text-blue-400">
            <Calendar className="h-5 w-5" />
          </div>
        </CardHeader>
        <CardContent>
          <div className="text-2xl font-bold">12</div>
          <div className="flex items-center mt-1 text-green-500 text-sm">
            <ArrowUpRight className="h-4 w-4 mr-1" />
            <span>+3 since last week</span>
          </div>
        </CardContent>
      </Card>

      <Card className="bg-[#111827] border-[#1a2033]">
        <CardHeader className="flex flex-row items-center justify-between pb-2">
          <CardTitle className="text-sm font-medium">Avg. Progress</CardTitle>
          <div className="p-2 rounded-full bg-[#172a46] text-white">
            <Activity className="h-5 w-5" />
          </div>
        </CardHeader>
        <CardContent>
          <div className="text-2xl font-bold">8.5%</div>
          <div className="flex items-center mt-1 text-green-500 text-sm">
            <ArrowUpRight className="h-4 w-4 mr-1" />
            <span>+1.2% since last month</span>
          </div>
        </CardContent>
      </Card>
    </div>
  )
}
