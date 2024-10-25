import { post } from "@/utils"
import { useMutation } from "@tanstack/react-query"
import { GetTodoListsResponse } from "server"
import { z } from "zod"

const createListSchema = z.object({
  name: z
    .string({
      required_error: "Name is required",
    })
    .min(1, "Name is required"),
})

export const useCreateTodoList = () => {
  return useMutation({
    mutationFn: async (body: any) => {
      const parsed = createListSchema.safeParse(body)

      if (!parsed.success) {
        throw new Error(parsed.error.issues[0].message)
      }

      const { name } = parsed.data

      return await post<GetTodoListsResponse>("/lists", { name })
    },
  })
}
