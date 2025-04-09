"use client"

import type React from "react"
import { useState } from "react"
import { useRouter } from "next/navigation"
import { EyeIcon, EyeOffIcon } from "lucide-react"
import { Label } from "@/components/atoms/label"
import { Input } from "@/components/atoms/input"
import { Button } from "@/components/atoms/button"

export function LoginForm() {
  const [identifier, setIdentifier] = useState("")
  const [isEmailMode, setIsEmailMode] = useState(true) // Toggle between Email and Username
  const [password, setPassword] = useState("")
  const [showPassword, setShowPassword] = useState(false)
  const [subscribeTrainerPackage, setSubscribeTrainerPackage] = useState(false)
  const [errors, setErrors] = useState<{ identifier?: string; password?: string }>({})
  const [isLoading, setIsLoading] = useState(false)
  const router = useRouter()

  const validateForm = () => {
    const newErrors: { identifier?: string; password?: string } = {}

    if (!identifier) {
      newErrors.identifier = isEmailMode ? "Email is required" : "Username is required"
    } else if (isEmailMode && !/\S+@\S+\.\S+/.test(identifier)) {
      newErrors.identifier = "Email is invalid"
    } else if (!isEmailMode && !/^[a-zA-Z0-9_]+$/.test(identifier)) {
      newErrors.identifier = "Username can only contain letters, numbers, and underscores"
    }

    if (!password) {
      newErrors.password = "Password is required"
    } else if (password.length < 6) {
      newErrors.password = "Password must be at least 6 characters"
    }

    setErrors(newErrors)
    return Object.keys(newErrors).length === 0
  }

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()

    if (!validateForm()) return

    setIsLoading(true)

    try {
      // Simulate API call
      await new Promise((resolve) => setTimeout(resolve, 1000))

      if (subscribeTrainerPackage) {
        router.push("/subscription/payment")
      } else {
        router.push("/dashboard")
      }
    } catch (error) {
      console.error("Login failed:", error)
      setErrors({ identifier: "Invalid email/username or password" })
    } finally {
      setIsLoading(false)
    }
  }

  return (
    <form onSubmit={handleSubmit} className="space-y-4">
      <div className="space-y-2">
        <Label htmlFor="identifier" className="text-white">
          {isEmailMode ? "Email" : "Username"}
        </Label>
        <div className="relative">
          <Input
            id="identifier"
            type="text"
            value={identifier}
            onChange={(e) => setIdentifier(e.target.value)}
            placeholder={isEmailMode ? "Enter your email" : "Enter your username"}
            className="bg-slate-800 text-white border-slate-700 focus:border-cyan-400 focus:ring-cyan-400"
          />
          {errors.identifier && <p className="mt-1 text-xs text-red-500">{errors.identifier}</p>}
        </div>
      </div>

      <button
        type="button"
        onClick={() => {
          setIsEmailMode(!isEmailMode)
          setIdentifier("")
          setErrors({})
        }}
        className="text-cyan-400 text-sm hover:underline"
      >
        {isEmailMode ? "Use Username instead" : "Use Email instead"}
      </button>

      <div className="space-y-2">
        <Label htmlFor="password" className="text-white">
          Password
        </Label>
        <div className="relative">
          <Input
            id="password"
            type={showPassword ? "text" : "password"}
            value={password}
            onChange={(e) => setPassword(e.target.value)}
            placeholder="Enter your password"
            className="bg-slate-800 text-white border-slate-700 focus:border-cyan-400 focus:ring-cyan-400 pr-10"
          />
          <button
            type="button"
            onClick={() => setShowPassword(!showPassword)}
            className="absolute right-3 top-1/2 -translate-y-1/2 text-slate-400"
          >
            {showPassword ? <EyeOffIcon className="h-5 w-5" /> : <EyeIcon className="h-5 w-5" />}
          </button>
          {errors.password && <p className="mt-1 text-xs text-red-500">{errors.password}</p>}
        </div>
      </div>

      <div className="flex items-center space-x-2">
        <input
          type="checkbox"
          id="trainer-package"
          checked={subscribeTrainerPackage}
          onChange={() => setSubscribeTrainerPackage(!subscribeTrainerPackage)}
          className="h-4 w-4 text-cyan-500 border-gray-300 rounded focus:ring-cyan-400"
        />
        <Label htmlFor="trainer-package" className="text-white">
          Subscribe to Personal Trainer Package
        </Label>
      </div>

      <Button type="submit" className="w-full bg-cyan-500 hover:bg-cyan-600 text-white" disabled={isLoading}>
        {isLoading ? "Signing in..." : "Sign In"}
      </Button>
    </form>
  )
}

