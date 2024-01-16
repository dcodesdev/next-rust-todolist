import { Card } from "@/components/ui/card"
import { dayjs } from "@/utils/dayjs"
import { FC } from "react"
import { GetTodoListsResponse } from "server"

export const ListItem: FC<{
  list: GetTodoListsResponse["data"][0]
}> = ({ list }) => {
  return (
    <Card className="flex items-center justify-between p-2 gap-2">
      <div>{list.name}</div>
      <div>{dayjs(list.created_at).fromNow()}</div>
    </Card>
  )
}
