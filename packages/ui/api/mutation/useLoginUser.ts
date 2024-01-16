import { client } from "@/utils"
import { useMutation } from "@tanstack/react-query"
import { LoginUserResponse } from "server"
import Cookies from "js-cookie"
import { useRouter } from "next/navigation"
import { toastError } from "@/utils/toast"
import { z } from "zod"

const userLoginSchema = z.object({
  email: z
    .string({
      required_error: "Email is required",
    })
    .email(),
  password: z
    .string({
      required_error: "Password is required",
    })
    .min(1, "Password is required"),
})

export const useLoginUser = () => {
  const router = useRouter()

  return useMutation({
    mutationFn: async (body: any) => {
      const parsed = userLoginSchema.safeParse(body)

      if (!parsed.success) {
        return toastError(parsed.error.issues[0].message)
      }

      const { email, password } = parsed.data

      return client
        .post<LoginUserResponse>("/user/login", { email, password })
        .then((r) => {
          Cookies.set("token", r.data.token)
          router.push("/dashboard")
        })
        .catch(toastError)
    },
  })
}
