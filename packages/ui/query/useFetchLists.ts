import { client } from "@/utils"
import { useQuery } from "@tanstack/react-query"
import { GetTodoListsResponse } from "server"

/**
 * This hook gets all the todo lists that belong to the current user.
 */
export const useFetchLists = () => {
  return useQuery<GetTodoListsResponse>({
    queryKey: ["lists"],
    queryFn: async () => {
      return await client.get("/lists")
    },
  })
}
