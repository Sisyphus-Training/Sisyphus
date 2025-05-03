"use client"

import { useState } from "react"
import { Activity, ArrowUpRight, Dumbbell, Eye, ListChecks } from "lucide-react"
import { Button } from "@/components/atoms/button"
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/atoms/tabs"
import { Progress } from "@/components/atoms/progress"

// This would come from your data source in a real application
const patientData = {
  name: "Jane Smith",
  nextSession: "Today, 3:00 PM",
  metrics: [
    { name: "Weight", value: "65kg", change: -2, icon: <Activity className="h-4 w-4" /> },
    { name: "BodyFat", value: "22%", change: -1.5, icon: <Activity className="h-4 w-4" /> },
    { name: "Muscle", value: "45%", change: 1.1, icon: <Activity className="h-4 w-4" /> },
  ],
  currentRoutine: [
    { name: "Bench Press", sets: 4, reps: "8-10", completed: true },
    { name: "Squats", sets: 4, reps: "10-12", completed: true },
    { name: "Deadlifts", sets: 3, reps: "8", completed: false },
    { name: "Pull-ups", sets: 3, reps: "Max", completed: false },
  ],
  progress: {
    overall: 65,
    strength: 70,
    endurance: 60,
    flexibility: 50,
  },
}

export default function ActivePatientSection({ isPatientMode = true }) {
  const [activeTab, setActiveTab] = useState("routine")

  return (
    <div className="bg-[#111827] rounded-lg p-6 border border-[#1a2033] mt-8">
      <div className="flex justify-between items-center mb-6">
        <div className="flex items-center gap-2">
          <h2 className="text-xl font-bold">{isPatientMode ? "My Current Program" : "Active Patient"}</h2>
          {isPatientMode && (
            <div className="px-2 py-1 bg-cyan-500/20 text-cyan-400 text-xs rounded-md">Patient Mode</div>
          )}
        </div>
        <Button className="bg-cyan-500 hover:bg-cyan-600 text-white transition-colors">
          <Eye className="h-4 w-4 mr-2" />
          <span>View Details</span>
        </Button>
      </div>

      <div>
        {!isPatientMode && (
          <>
            <h3 className="text-lg font-semibold">{patientData.name}</h3>
            <p className="text-gray-400 mb-4">Next session: {patientData.nextSession}</p>
          </>
        )}

        <div className="grid grid-cols-1 md:grid-cols-3 gap-6 mb-6">
          {patientData.metrics.map((metric, index) => (
            <div key={index} className="bg-[#0a0e17] rounded-lg p-4">
              <p className="text-gray-400 mb-1">{metric.name}</p>
              <div className="flex justify-between items-center">
                <h4 className="text-2xl font-bold">{metric.value}</h4>
                <span className={`flex items-center ${metric.change > 0 ? "text-green-500" : "text-cyan-400"}`}>
                  <ArrowUpRight className={`h-4 w-4 mr-1 ${metric.change < 0 ? "rotate-90" : ""}`} />
                  {metric.change > 0 ? `+${metric.change}` : metric.change}
                </span>
              </div>
            </div>
          ))}
        </div>

        <Tabs value={activeTab} onValueChange={setActiveTab} className="w-full">
            <TabsList className="flex flex-col w-full h-auto md:flex-row gap-2 mb-4 w-full bg-[#0a0e17] rounded-lg">
            <TabsTrigger value="routine" className="w-full data-[state=active]:bg-green-500">
                <ListChecks className="h-4 w-4 mr-2" />
                Current Routine
            </TabsTrigger>
            <TabsTrigger value="progress" className="w-full data-[state=active]:bg-green-500">
                <Activity className="h-4 w-4 mr-2" />
                Progress Updates
            </TabsTrigger>
            </TabsList>

          <TabsContent value="routine" className="mt-0">
            <div className="bg-[#0a0e17] rounded-lg p-4">
              <div className="space-y-4">
                {patientData.currentRoutine.map((exercise, index) => (
                  <div key={index} className="flex items-center justify-between p-3 rounded-md bg-[#111827]">
                    <div className="flex items-center">
                      <div
                        className={`w-2 h-2 rounded-full mr-3 ${exercise.completed ? "bg-green-500" : "bg-gray-500"}`}
                      ></div>
                      <div>
                        <p className="font-medium">{exercise.name}</p>
                        <p className="text-sm text-gray-400">
                          {exercise.sets} sets × {exercise.reps} reps
                        </p>
                      </div>
                    </div>
                    <div className="flex items-center">
                      <Dumbbell className="h-5 w-5 text-cyan-400" />
                    </div>
                  </div>
                ))}
              </div>
              <div className="mt-4 flex justify-between items-center">
                <div className="text-sm text-gray-400">2 of 4 exercises completed</div>
                <Progress value={50} className="w-1/3 h-2 bg-[#172a46]" indicatorClassName="bg-cyan-500" />
              </div>
            </div>
          </TabsContent>

          <TabsContent value="progress" className="mt-0">
            <div className="bg-[#0a0e17] rounded-lg p-4">
              <div className="space-y-4">
                <div className="space-y-2">
                  <div className="flex justify-between">
                    <span className="text-sm">Overall Progress</span>
                    <span className="text-sm font-medium">{patientData.progress.overall}%</span>
                  </div>
                  <Progress
                    value={patientData.progress.overall}
                    className="h-2 bg-[#172a46]"
                    indicatorClassName="bg-green-500"
                  />
                </div>

                <div className="space-y-2">
                  <div className="flex justify-between">
                    <span className="text-sm">Strength</span>
                    <span className="text-sm font-medium">{patientData.progress.strength}%</span>
                  </div>
                  <Progress
                    value={patientData.progress.strength}
                    className="h-2 bg-[#172a46]"
                    indicatorClassName="bg-blue-500"
                  />
                </div>

                <div className="space-y-2">
                  <div className="flex justify-between">
                    <span className="text-sm">Endurance</span>
                    <span className="text-sm font-medium">{patientData.progress.endurance}%</span>
                  </div>
                  <Progress
                    value={patientData.progress.endurance}
                    className="h-2 bg-[#172a46]"
                    indicatorClassName="bg-white"
                  />
                </div>

                <div className="space-y-2">
                  <div className="flex justify-between">
                    <span className="text-sm">Flexibility</span>
                    <span className="text-sm font-medium">{patientData.progress.flexibility}%</span>
                  </div>
                  <Progress
                    value={patientData.progress.flexibility}
                    className="h-2 bg-[#172a46]"
                    indicatorClassName="bg-green-400"
                  />
                </div>
              </div>

              <div className="mt-4 text-sm text-gray-400 flex items-center">
                <ArrowUpRight className="h-4 w-4 mr-1 text-green-500" />
                <span>+5% improvement since last month</span>
              </div>
            </div>
          </TabsContent>
        </Tabs>
      </div>
    </div>
  )
}
