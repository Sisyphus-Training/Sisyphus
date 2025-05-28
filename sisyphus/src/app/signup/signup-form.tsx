"use client"

import { useState } from "react"
import { useRouter } from "next/navigation"
import { Button } from "@/components/atoms/button"
import { Input } from "@/components/atoms/input"
import { Label } from "@/components/atoms/label"
import { Checkbox } from "@/components/atoms/checkbox"
import { EyeIcon, EyeOffIcon } from "lucide-react"

export function SignupForm() {
  const [email, setEmail] = useState("")
  const [username, setUsername] = useState("")
  const [password, setPassword] = useState("")
  const [confirmPassword, setConfirmPassword] = useState("")
  const [showPassword, setShowPassword] = useState(false)
  const [isPTPackage, setIsPTPackage] = useState(false)
  const [errors, setErrors] = useState<{
    email?: string
    username?: string
    password?: string
    confirmPassword?: string
  }>({})
  const [isLoading, setIsLoading] = useState(false)
  const router = useRouter()
  const [isCapsLockOn, setIsCapsLockOn] = useState(false)

  const validateForm = () => {
    const newErrors: {
      email?: string
      username?: string
      password?: string
      confirmPassword?: string
    } = {}

    if (!email) {
      newErrors.email = "Email is required"
    } else if (!/\S+@\S+\.\S+/.test(email)) {
      newErrors.email = "Email is invalid"
    }

    if (!username) {
      newErrors.username = "Username is required"
    } else if (username.length < 3) {
      newErrors.username = "Username must be at least 3 characters"
    }

    if (!password) {
      newErrors.password = "Password is required"
    } else if (password.length < 6) {
      newErrors.password = "Password must be at least 6 characters"
    }

    if (password !== confirmPassword) {
      newErrors.confirmPassword = "Passwords do not match"
    }

    setErrors(newErrors)
    return Object.keys(newErrors).length === 0
  }

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()

    if (!validateForm()) return

    setIsLoading(true)

    try {
      // In a real app, you would call your authentication API here
      // const response = await signUpWithEmail(email, username, password)

      // Simulate API call
      await new Promise((resolve) => setTimeout(resolve, 1000))

      // Redirect based on subscription choice
      if (isPTPackage) {
        router.push("/subscription")
      } else {
        router.push("/dashboard")
      }
    } catch (error) {
      console.error("Signup failed:", error)
      setErrors({ email: "Failed to create account. Please try again." })
    } finally {
      setIsLoading(false)
    }
  }

  const detectCapsLock = (event: React.KeyboardEvent) => {
    const capsLockOn = event.getModifierState && event.getModifierState("CapsLock")
    setIsCapsLockOn(capsLockOn)
  }

  return (
    <form onSubmit={handleSubmit} className="space-y-4">
      <div className="space-y-2">
        <Label htmlFor="email" className="text-white">
          Email
        </Label>
        <Input
          id="email"
          type="email"
          value={email}
          onChange={(e) => setEmail(e.target.value)}
          placeholder="Enter your email"
          className="bg-slate-800 text-white border-slate-700 focus:border-cryan-500 focus:ring-cryan-500"
        />
        {errors.email && <p className="mt-1 text-xs text-red-500">{errors.email}</p>}
      </div>

      <div className="space-y-2">
        <Label htmlFor="username" className="text-white">
          Username
        </Label>
        <Input
          id="username"
          type="text"
          value={username}
          onChange={(e) => setUsername(e.target.value)}
          placeholder="Choose a username"
          className="bg-slate-800 text-white border-slate-700 focus:border-cryan-500 focus:ring-cryan-500"
        />
        {errors.username && <p className="mt-1 text-xs text-red-500">{errors.username}</p>}
      </div>

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
            onKeyDown={detectCapsLock}
            placeholder="Create a password"
            className="bg-slate-800 text-white border-slate-700 focus:border-cryan-500 focus:ring-cryan-500 pr-10"
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

      {isCapsLockOn && (
        <div className="flex items-center mt-1 text-xs text-yellow-500">
          <svg className="w-4 h-4 mr-1" fill="currentColor" viewBox="0 0 20 20">
            <path
              fillRule="evenodd"
              d="M8.257 3.099c.765-1.36 2.722-1.36 3.486 0l5.58 9.92c.75 1.334-.213 2.98-1.742 2.98H4.42c-1.53 0-2.493-1.646-1.743-2.98l5.58-9.92zM11 13a1 1 0 11-2 0 1 1 0 012 0zm-1-8a1 1 0 00-1 1v3a1 1 0 002 0V6a1 1 0 00-1-1z"
              clipRule="evenodd"
            />
          </svg>
          Caps Lock is on
        </div>
      )}

      <div className="space-y-2">
        <Label htmlFor="confirmPassword" className="text-white">
          Confirm Password
        </Label>
        <Input
          id="confirmPassword"
          type={showPassword ? "text" : "password"}
          value={confirmPassword}
          onChange={(e) => setConfirmPassword(e.target.value)}
          onKeyDown={detectCapsLock}
          placeholder="Confirm your password"
          className="bg-slate-800 text-white border-slate-700 focus:border-cryan-500 focus:ring-cryan-500"
        />
        {errors.confirmPassword && <p className="mt-1 text-xs text-red-500">{errors.confirmPassword}</p>}
      </div>

      <div className="flex items-center space-x-2">
        <Checkbox
          id="ptPackage"
          checked={isPTPackage}
          onCheckedChange={(checked:any) => setIsPTPackage(checked as boolean)}
          className="border-cryan-500 data-[state=checked]:bg-cyan-500"
        />
        <Label htmlFor="ptPackage" className="text-sm text-white">
          Subscribe to Personal Trainer Package
        </Label>
      </div>

      <Button type="submit" className="w-full bg-cyan-500 hover:bg-cyan-600 text-white" disabled={isLoading}>
        {isLoading ? "Creating account..." : "Create Account"}
      </Button>
    </form>
  )
}
