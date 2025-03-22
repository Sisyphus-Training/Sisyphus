import React from "react"

interface InputProps extends React.InputHTMLAttributes<HTMLInputElement> {
  className?: string
}

export const Input: React.FC<InputProps> = ({ className = "", ...props }) => {
  return (
    <input
      {...props}
      className={`w-full px-3 py-2 rounded-md border border-gray-600 focus:outline-none 
        focus:ring-2 focus:ring-cyan-400 focus:border-cyan-400 bg-gray-800 text-white ${className}`}
    />
  )
}
