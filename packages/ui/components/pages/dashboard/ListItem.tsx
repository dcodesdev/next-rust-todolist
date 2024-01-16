import { Card } from "@/components/ui/card"
import { dayjs } from "@/utils/dayjs"
import { Trash } from "lucide-react"
import { FC } from "react"
import { GetTodoListsResponse } from "server"

export const ListItem: FC<{
  list: GetTodoListsResponse["data"][0]
}> = ({ list }) => {
  return (
    <Card className="flex items-center justify-between p-2 gap-2">
      <div>
        <p>{list.name}</p>

        <small className="text-gray-400">
          {dayjs(list.created_at).fromNow()}
        </small>
      </div>

      <div className="flex items-center gap-1">
        <div>
          <Trash size={20} />
        </div>
      </div>
    </Card>
  )
}
