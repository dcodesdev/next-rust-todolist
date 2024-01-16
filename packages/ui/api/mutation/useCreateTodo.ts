import { post } from "@/utils"
import { useMutation } from "@tanstack/react-query"
import { z } from "zod"

const createTodoSchema = z.object({
  title: z
    .string({
      required_error: "Title is required",
    })
    .min(1, "Title is required"),
  list_id: z.string({
    required_error: "List is required",
  }),
  description: z
    .string({
      required_error: "Description is required",
    })
    .min(1, "Description is required"),
})

export const useCreateTodo = () => {
  return useMutation({
    mutationFn: (payload: unknown) => {
      const parsed = createTodoSchema.safeParse(payload)

      if (!parsed.success) {
        throw new Error(parsed.error.issues[0].message)
      }

      return post(`/todos`, parsed.data)
    },
  })
}
