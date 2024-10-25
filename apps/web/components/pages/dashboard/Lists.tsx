import { FC } from "react"

import { useFetchLists } from "@/api/query/useFetchLists"
import { CreateListButton } from "./CreateListButton"
import { ListItem } from "./ListItem"

export const Lists: FC<{
  data: ReturnType<typeof useFetchLists>["data"]
}> = ({ data }) => {
  return (
    <div>
      <CreateListButton />

      <div className="mt-5 flex flex-col gap-1">
        {data?.data.map((list) => <ListItem key={list.id} list={list} />)}
      </div>
    </div>
  )
}
