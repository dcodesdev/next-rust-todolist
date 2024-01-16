"use client"
import { Container } from "@/components/Container"
import { Button } from "@/components/ui/button"
import {
  Card,
  CardContent,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import { client } from "@/utils"
import { FormEvent, useRef } from "react"
import { LoginUserResponse } from "server"
import Cookies from "js-cookie"
import { useMutation } from "@tanstack/react-query"
import { toastError } from "@/utils/toast"
import { useRouter } from "next/navigation"

export default function Login() {
  const emailRef = useRef<HTMLInputElement>(null)
  const passwordRef = useRef<HTMLInputElement>(null)

  const router = useRouter()

  const { mutateAsync: onSubmit, isPending } = useMutation({
    mutationFn: async (e: FormEvent) => {
      e.preventDefault()

      const email = emailRef.current?.value
      const password = passwordRef.current?.value

      if (!email) return
      if (!password) return

      return client
        .post<LoginUserResponse>("/user/login", { email, password })
        .then((r) => {
          Cookies.set("token", r.data.token)
          router.push("/dashboard")
        })
        .catch(toastError)
    },
  })

  return (
    <Container>
      <h1 className="text-4xl font-bold text-center">Login</h1>

      <Card className="mt-28 max-w-sm mx-auto">
        <CardHeader>
          <CardTitle>Login</CardTitle>
        </CardHeader>

        <form onSubmit={onSubmit}>
          <CardContent>
            <Label>Email</Label>

            <Input
              ref={emailRef}
              type="email"
              placeholder="Enter your email address"
            />

            <Label className="mt-3">Password</Label>

            <Input
              ref={passwordRef}
              type="password"
              placeholder="Enter your password"
            />
          </CardContent>

          <CardFooter>
            <Button disabled={isPending}>Login</Button>
          </CardFooter>
        </form>
      </Card>
    </Container>
  )
}
