"use client"

import { useState } from "react"
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/atoms/tabs"
import { Calendar, ListChecks } from "lucide-react"
import { UpcomingSessions } from "./upcoming-session"
import ActivePatientSection from "./active-petient-section"
import { StatsCards } from "./atoms/stats-card"

export default function Dashboard({ isPatientMode = true }) {
  const [activeTab, setActiveTab] = useState("overview")

  return (
    <div className="w-full  p-6 bg-[#0a0e17] rounded-lg border border-[#1a2033]">
      {!isPatientMode && (
        <Tabs value={activeTab} onValueChange={setActiveTab} className="mb-6">
          <TabsList className="bg-[#0a0e17] w-full md:w-auto overflow-x-auto">
            <TabsTrigger value="overview" className="data-[state=active]:bg-green-500">
              <ListChecks className="h-4 w-4 mr-2" />
              <span className="whitespace-nowrap">Overview</span>
            </TabsTrigger>
        
            <TabsTrigger value="schedule" className="data-[state=active]:bg-green-500">
              <Calendar className="h-4 w-4 mr-2" />
              <span className="whitespace-nowrap">Schedule</span>
            </TabsTrigger>
          </TabsList>

          <TabsContent value="overview" className="mt-6 space-y-6">
            {!isPatientMode && <StatsCards />}
            <ActivePatientSection isPatientMode={isPatientMode} />
            <UpcomingSessions />
          </TabsContent>

          <TabsContent value="schedule" className="mt-6">
            <UpcomingSessions showFullSchedule={true} />
          </TabsContent>
        </Tabs>
      )}

      {isPatientMode && (
        <div className="space-y-6">
          <ActivePatientSection isPatientMode={isPatientMode} />
          <UpcomingSessions />
        </div>
      )}
    </div>
  )
}
