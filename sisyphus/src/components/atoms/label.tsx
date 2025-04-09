import React from "react"

interface LabelProps extends React.LabelHTMLAttributes<HTMLLabelElement> {
  className?: string
}

export const Label: React.FC<LabelProps> = ({ className = "", ...props }) => {
  return (
    <label {...props} className={`block text-sm font-medium text-white ${className}`} />
  )
}
