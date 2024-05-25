import { put } from "@/utils"
import { useMutation } from "@tanstack/react-query"
import { z } from "zod"

const updateTodoSchema = z.object({
  title: z.string().optional(),
  description: z.string().optional(),
  completed: z.boolean().optional(),
})

export const useUpdateTodo = (id: string) => {
  return useMutation({
    mutationFn: (payload: unknown) => {
      const parsed = updateTodoSchema.safeParse(payload)

      if (!parsed.success) {
        throw new Error("Invalid payload")
      }

      return put(`/todos/${id}`, parsed.data)
    },
  })
}
