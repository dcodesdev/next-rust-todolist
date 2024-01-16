"use client"
import { Container } from "@/components/Container"
import { Button } from "@/components/ui/button"
import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import { FormEvent, useRef } from "react"
import { useUserAuth } from "@/api/mutation/useUserAuth"
import { toastError } from "@/utils/toast"
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs"

export default function Login() {
  const nameRef = useRef<HTMLInputElement>(null)
  const emailRef = useRef<HTMLInputElement>(null)
  const passwordRef = useRef<HTMLInputElement>(null)

  const { mutateAsync: loginUser, isPending: isLoggingIn } = useUserAuth()
  const { mutateAsync: registerUser, isPending: isRegistering } =
    useUserAuth(true)

  const onSubmit = (isRegister: boolean) => (e: FormEvent) => {
    e.preventDefault()

    const email = emailRef.current?.value
    const password = passwordRef.current?.value

    if (isRegister) {
      registerUser({ email, password }).catch(toastError)
    } else {
      loginUser({ email, password }).catch(toastError)
    }
  }

  return (
    <Container>
      <h1 className="text-4xl font-bold text-center">Login</h1>

      <Tabs defaultValue="login" className="w-[400px] mx-auto mt-20">
        <TabsList className="grid w-full grid-cols-2">
          <TabsTrigger value="login">Log In</TabsTrigger>
          <TabsTrigger value="register">Register</TabsTrigger>
        </TabsList>
        <TabsContent value="login">
          <Card>
            <CardHeader>
              <CardTitle>Login</CardTitle>
              <CardDescription>
                Enter your email and password to login into your account.
              </CardDescription>
            </CardHeader>

            <form onSubmit={onSubmit(false)}>
              <CardContent className="space-y-2">
                <div className="space-y-1">
                  <Label htmlFor="email">Email</Label>
                  <Input ref={emailRef} id="email" placeholder="jon@doe.com" />
                </div>
                <div className="space-y-1">
                  <Label htmlFor="password">Password</Label>
                  <Input
                    ref={passwordRef}
                    id="password"
                    placeholder="********"
                  />
                </div>
              </CardContent>

              <CardFooter>
                <Button disabled={isLoggingIn} type="submit">
                  Login
                </Button>
              </CardFooter>
            </form>
          </Card>
        </TabsContent>

        <TabsContent value="register">
          <Card>
            <CardHeader>
              <CardTitle>Register</CardTitle>
              <CardDescription>
                Create a new account if you don&apos;t have one yet.
              </CardDescription>
            </CardHeader>

            <form onSubmit={onSubmit(true)}>
              <CardContent className="space-y-2">
                <div className="space-y-1">
                  <Label htmlFor="name">Name</Label>
                  <Input
                    ref={nameRef}
                    id="name"
                    type="name"
                    placeholder="Enter your name"
                  />
                </div>
                <div className="space-y-1">
                  <Label htmlFor="email">Email</Label>
                  <Input
                    ref={emailRef}
                    id="email"
                    type="email"
                    placeholder="Enter your email address"
                  />
                </div>
                <div className="space-y-1">
                  <Label htmlFor="password">Password</Label>
                  <Input
                    ref={passwordRef}
                    id="password"
                    type="password"
                    placeholder="Enter your password"
                  />
                </div>
              </CardContent>

              <CardFooter>
                <Button disabled={isRegistering} type="submit">
                  Register
                </Button>
              </CardFooter>
            </form>
          </Card>
        </TabsContent>
      </Tabs>
    </Container>
  )
}
