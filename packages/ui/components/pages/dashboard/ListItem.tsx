"use client"
import { useDeleteList } from "@/api/mutation/useDeleteList"
import { useFetchLists } from "@/api/query/useFetchLists"
import { Button } from "@/components/ui/button"
import { Card } from "@/components/ui/card"
import {
  Dialog,
  DialogContent,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog"
import { useModal } from "@/hooks/useModal"
import { dayjs } from "@/utils/dayjs"
import { Trash } from "lucide-react"
import { FC } from "react"
import { GetTodoListsResponse } from "server"
import { toast } from "sonner"

export const ListItem: FC<{
  list: GetTodoListsResponse["data"][0]
}> = ({ list }) => {
  const { close, isOpen, open, setIsOpen } = useModal()

  const { mutateAsync: deleteList } = useDeleteList(list.id)
  const { refetch } = useFetchLists()

  const onPress = () => {
    deleteList().then(() => {
      toast.success("List deleted successfully")
      refetch()
      close()
    })
  }

  return (
    <Card className="flex items-center justify-between p-2 gap-2">
      <div>
        <p>{list.name}</p>

        <small className="text-gray-400">
          {dayjs(list.created_at).fromNow()}
        </small>
      </div>

      <div className="flex items-center gap-1"></div>

      <Dialog open={isOpen} onOpenChange={setIsOpen}>
        <DialogTrigger asChild>
          <Trash onClick={open} className="cursor-pointer" size={20} />
        </DialogTrigger>

        <DialogContent>
          <DialogHeader>
            <DialogTitle>
              Are you sure you want to delete this list?
            </DialogTitle>
          </DialogHeader>

          <DialogFooter>
            <Button variant="destructive" onClick={onPress}>
              Yes
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </Card>
  )
}
