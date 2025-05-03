"use client"

import { Calendar, Clock, MapPin, MoreVertical } from "lucide-react"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/atoms/card"
import { Avatar, AvatarFallback, AvatarImage } from "@/components/atoms/avatar"
import { Button } from "@/components/atoms/button"
import { DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger } from "@/components/atoms/dropdown-menu"
import { Badge } from "@/components/atoms/badge"

const upcomingSessions = [
  {
    id: 1,
    patient: "Jane Smith",
    avatar: "JS",
    date: "Today",
    time: "3:00 PM",
    location: "Main Gym",
    type: "Strength Training",
    status: "confirmed",
  },
  {
    id: 2,
    patient: "Mike Johnson",
    avatar: "MJ",
    date: "Today",
    time: "5:30 PM",
    location: "Online Session",
    type: "Nutrition Consultation",
    status: "confirmed",
  },
  {
    id: 3,
    patient: "Sarah Williams",
    avatar: "SW",
    date: "Tomorrow",
    time: "10:00 AM",
    location: "Main Gym",
    type: "Cardio Training",
    status: "pending",
  },
  {
    id: 4,
    patient: "Robert Brown",
    avatar: "RB",
    date: "Tomorrow",
    time: "2:00 PM",
    location: "Park",
    type: "Outdoor Training",
    status: "confirmed",
  },
]

export function UpcomingSessions({ showFullSchedule = false }) {
  const displaySessions = showFullSchedule ? upcomingSessions : upcomingSessions.slice(0, 3)

  return (
    <Card className="bg-[#111827] border-[#1a2033]">
      <CardHeader className="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4">
        <CardTitle>Upcoming Sessions</CardTitle>
        {!showFullSchedule && (
          <Button variant="outline" size="sm" className="border-[#1a2033] w-full sm:w-auto">
            <Calendar className="h-4 w-4 mr-2" />
            View All
          </Button>
        )}
      </CardHeader>
      <CardContent>
        <div className="space-y-4">
          {displaySessions.map((session) => (
            <div
              key={session.id}
              className="flex flex-col sm:flex-row sm:items-center justify-between p-3 rounded-md bg-[#0a0e17] border border-[#1a2033] gap-4"
            >
              <div className="flex items-center gap-3">
                <Avatar>
                  <AvatarImage src={`/placeholder.svg?height=40&width=40&text=${session.avatar}`} />
                  <AvatarFallback className="bg-[#172a46]">{session.avatar}</AvatarFallback>
                </Avatar>
                <div>
                  <div className="flex items-center gap-2 flex-wrap">
                    <p className="font-medium">{session.patient}</p>
                    <Badge
                      variant="outline"
                      className={`text-xs ${
                        session.status === "confirmed"
                          ? "border-green-500 text-green-500"
                          : "border-yellow-500 text-yellow-500"
                      }`}
                    >
                      {session.status}
                    </Badge>
                  </div>
                  <div className="flex flex-col xs:flex-row xs:items-center gap-2 text-sm text-gray-400 mt-1">
                    <div className="flex items-center">
                      <Clock className="h-3 w-3 mr-1" />
                      {session.date}, {session.time}
                    </div>
                    <div className="flex items-center">
                      <MapPin className="h-3 w-3 mr-1" />
                      {session.location}
                    </div>
                  </div>
                </div>
              </div>
              <div className="flex items-center justify-between sm:justify-end gap-2 mt-2 sm:mt-0">
                <Badge className="bg-blue-500">{session.type}</Badge>
                <DropdownMenu>
                  <DropdownMenuTrigger asChild>
                    <Button variant="ghost" size="icon" className="h-8 w-8">
                      <MoreVertical className="h-4 w-4" />
                    </Button>
                  </DropdownMenuTrigger>
                  <DropdownMenuContent align="end" className="bg-[#0a0e17] border-[#1a2033]">
                    <DropdownMenuItem>View Details</DropdownMenuItem>
                    <DropdownMenuItem>Reschedule</DropdownMenuItem>
                    <DropdownMenuItem className="text-red-500">Cancel</DropdownMenuItem>
                  </DropdownMenuContent>
                </DropdownMenu>
              </div>
            </div>
          ))}
        </div>

        {showFullSchedule && (
          <Button className="w-full mt-4 bg-green-500 hover:bg-green-600">Schedule New Session</Button>
        )}
      </CardContent>
    </Card>
  )
}
