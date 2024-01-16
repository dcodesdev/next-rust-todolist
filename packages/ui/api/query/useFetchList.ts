import { get } from "@/utils"
import { useQuery } from "@tanstack/react-query"
import { GetTodoListDetailsResponse } from "server"

export const useFetchList = (id: string) => {
  return useQuery({
    queryKey: ["list", id],
    queryFn: () => get<GetTodoListDetailsResponse>(`/lists/${id}`),
  })
}
