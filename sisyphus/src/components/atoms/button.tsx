import React from "react"

interface ButtonProps extends React.ButtonHTMLAttributes<HTMLButtonElement> {
  className?: string
}

export const Button: React.FC<ButtonProps> = ({ className = "", ...props }) => {
  return (
    <button
      {...props}
      className={`px-4 py-2 rounded-md font-medium transition duration-200 ease-in-out 
        bg-cyan-500 hover:bg-cyan-600 text-white disabled:opacity-50 ${className}`}
    />
  )
}
