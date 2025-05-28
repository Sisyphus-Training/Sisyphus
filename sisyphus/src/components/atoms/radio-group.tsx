"use client"

import * as React from "react"
import { cn } from "@/lib/utils"

interface RadioGroupProps {
  value?: string
  onValueChange?: (value: string) => void
  className?: string
  children: React.ReactNode
}

const RadioGroup = React.forwardRef<HTMLDivElement, RadioGroupProps>(
  ({ className, value, onValueChange, children, ...props }, ref) => {
    return (
      <div ref={ref} role="radiogroup" className={cn("grid gap-2", className)} {...props}>
        {React.Children.map(children, (child) => {
          if (React.isValidElement<RadioGroupItemProps>(child) && child.type === RadioGroupItem) {
            return React.cloneElement(child, {
              checked: (child.props as RadioGroupItemProps).value === value,
              onSelect: onValueChange,
            })
          }
          return child
        })}
      </div>
    )
  },
)

RadioGroup.displayName = "RadioGroup"

interface RadioGroupItemProps {
  value: string
  checked?: boolean
  onSelect?: (value: string) => void
  disabled?: boolean
  className?: string
  id?: string
}

const RadioGroupItem = React.forwardRef<HTMLButtonElement, RadioGroupItemProps>(
  ({ className, value, checked = false, onSelect, disabled = false, id, ...props }, ref) => {
    const handleClick = () => {
      if (!disabled && onSelect) {
        onSelect(value)
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
        role="radio"
        aria-checked={checked}
        aria-disabled={disabled}
        id={id}
        className={cn(
          "aspect-square h-4 w-4 rounded-full border border-slate-300 ring-offset-background focus:outline-none focus-visible:ring-2 focus-visible:ring-green-500 focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50",
          checked && "border-green-500",
          className,
        )}
        onClick={handleClick}
        onKeyDown={handleKeyDown}
        disabled={disabled}
        {...props}
      >
        {checked && (
          <div className="flex items-center justify-center">
            <div className="h-2 w-2 rounded-full bg-green-500" />
          </div>
        )}
      </button>
    )
  },
)

RadioGroupItem.displayName = "RadioGroupItem"

export { RadioGroup, RadioGroupItem }
