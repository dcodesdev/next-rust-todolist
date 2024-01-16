import { del } from "@/utils"
import { useMutation } from "@tanstack/react-query"

export const useDeleteList = (id: string) => {
  return useMutation({
    mutationFn: () => del(`/lists/${id}`),
  })
}
