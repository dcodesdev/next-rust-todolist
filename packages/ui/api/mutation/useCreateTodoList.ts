import { post } from "@/utils"
import { toastError } from "@/utils/toast"
import { useMutation } from "@tanstack/react-query"
import { GetTodoListsResponse } from "server"
import { z } from "zod"

const createListSchema = z.object({
  name: z.string({
    required_error: "Name is required",
  }),
})

export const useCreateTodoList = () => {
  return useMutation({
    mutationKey: ["createList"],
    mutationFn: async (body: any) => {
      const parsed = createListSchema.safeParse(body)

      if (!parsed.success) {
        return toastError(parsed.error.issues[0].message)
      }

      const { name } = parsed.data

      return await post<GetTodoListsResponse>("/lists", { name })
    },
  })
}
