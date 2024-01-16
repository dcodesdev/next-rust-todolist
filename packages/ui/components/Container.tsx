import { cn } from "@/lib/utils"
import { FC, HTMLAttributes } from "react"

interface ContainerProps extends HTMLAttributes<HTMLDivElement> {}

export const Container: FC<ContainerProps> = ({ className, ...props }) => {
  return (
    <div
      className={cn("w-full max-w-4xl mx-auto p-10", className)}
      {...props}
    />
  )
}
