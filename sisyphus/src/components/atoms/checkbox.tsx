"use client"

import * as React from "react"
import { Check } from "lucide-react"
import { cn } from "@/lib/utils"

interface CheckboxProps {
  checked?: boolean
  onCheckedChange?: (checked: boolean) => void
  disabled?: boolean
  className?: string
  id?: string
}

const Checkbox = React.forwardRef<HTMLButtonElement, CheckboxProps>(
  ({ className, checked = false, onCheckedChange, disabled = false, id, ...props }, ref) => {
    const handleClick = () => {
      if (!disabled && onCheckedChange) {
        onCheckedChange(!checked)
      }
    }

    const handleKeyDown = (event: React.KeyboardEvent) => {
      if (event.key === " " || event.key === "Enter") {
        event.preventDefault()
        handleClick()
      }
    }

    return (
      <button
        ref={ref}
        type="button"
        role="checkbox"
        aria-checked={checked}
        aria-disabled={disabled}
        id={id}
        className={cn(
          "peer h-4 w-4 shrink-0 rounded-sm border border-slate-300 ring-offset-background focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-green-500 focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50",
          checked && "bg-green-500 border-green-500 text-white",
          !checked && "bg-white",
          className,
        )}
        onClick={handleClick}
        onKeyDown={handleKeyDown}
        disabled={disabled}
        {...props}
      >
        {checked && (
          <div className="flex items-center justify-center text-current">
            <Check className="h-3 w-3" />
          </div>
        )}
      </button>
    )
  },
)

Checkbox.displayName = "Checkbox"

export { Checkbox }
