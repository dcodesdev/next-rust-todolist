import { get } from "@/utils"
import { useQuery } from "@tanstack/react-query"
import { GetTodoListsResponse } from "server"

/**
 * This hook gets all the todo lists that belong to the current user.
 */
export const useFetchLists = () => {
  return useQuery({
    queryKey: ["lists"],
    queryFn: () => get<GetTodoListsResponse>("/lists"),
  })
}
