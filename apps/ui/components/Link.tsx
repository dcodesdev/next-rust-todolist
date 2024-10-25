import NextLink, { type LinkProps as NextLinkProps } from "next/link"
import type { FC, ReactNode } from "react"

import { cn } from "@/lib/utils"

interface LinkProps extends NextLinkProps {
  className?: string
  children?: ReactNode
}

export const Link: FC<LinkProps> = ({ className, ...props }) => (
  <NextLink
    className={cn("hover:text-gray-300 hover:underline", className)}
    {...props}
  />
)
