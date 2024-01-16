"use client"

import { useCreateTodo } from "@/api/mutation/useCreateTodo"
import { useFetchList } from "@/api/query/useFetchList"
import { Container } from "@/components/Container"
import { Loading } from "@/components/Loading"
import { Button } from "@/components/ui/button"
import {
  Card,
  CardContent,
  CardDescription,
  CardTitle,
} from "@/components/ui/card"
import {
  Dialog,
  DialogContent,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog"
import { Input } from "@/components/ui/input"
import { Separator } from "@/components/ui/separator"
import { useModal } from "@/hooks/useModal"
import { cn } from "@/lib/utils"
import { dayjs } from "@/utils/dayjs"
import { toastError } from "@/utils/toast"
import { FC, FormEvent, useRef } from "react"
import { GetTodoListDetailsResponse } from "server"
import { toast } from "sonner"

export default function ListPage({ params }: { params: { id: string } }) {
  const { data } = useFetchList(params.id)

  if (!data) return <Loading />

  return (
    <Container>
      <h1 className="text-4xl font-bold">List Page of id {data.list.name}</h1>

      <AddTodoButton listId={params.id} className="my-10" />

      <div className="flex justify-between items-center gap-2 my-3">
        <p>{data.list.name}</p>
        <p>{dayjs(data.list.created_at).fromNow()}</p>
      </div>

      <Separator />

      <div className="flex flex-col gap-2">
        {data.items.map((todo) => (
          <TodoItem key={todo.id} todo={todo} />
        ))}
      </div>
    </Container>
  )
}

export const TodoItem: FC<{
  todo: GetTodoListDetailsResponse["items"][number]
}> = ({ todo }) => {
  return (
    <Card className="flex justify-between items-center gap-2 p-3 cursor-pointer hover:bg-slate-900">
      <CardContent>
        <CardTitle>{todo.title}</CardTitle>
        <CardDescription className="mt-3">{todo.description}</CardDescription>

        <small className="text-gray-500">
          Created {dayjs(todo.created_at).fromNow()}
        </small>
      </CardContent>
    </Card>
  )
}

export const AddTodoButton: FC<{ listId: string; className?: string }> = ({
  className,
  listId,
}) => {
  const titleRef = useRef<HTMLInputElement>(null)
  const descriptionRef = useRef<HTMLInputElement>(null)

  const { close, isOpen, open, setIsOpen } = useModal()

  const { mutateAsync } = useCreateTodo()
  const { refetch } = useFetchList(listId)

  const onSubmit = (e: FormEvent) => {
    e.preventDefault()

    const title = titleRef.current?.value
    const description = descriptionRef.current?.value

    mutateAsync({
      title,
      description,
      list_id: listId,
    })
      .then(() => {
        toast.success("Todo created successfully")
        refetch()
        close()
      })
      .catch(toastError)
  }

  return (
    <div className={cn("", className)}>
      <Button onClick={open}>Add Todo</Button>

      <Dialog open={isOpen} onOpenChange={setIsOpen}>
        <DialogContent>
          <form onSubmit={onSubmit}>
            <DialogHeader>
              <DialogTitle>Add a new todo item</DialogTitle>
            </DialogHeader>

            <div className="flex flex-col gap-2 mt-4">
              <Input ref={titleRef} placeholder="Title" />
              <Input ref={descriptionRef} placeholder="Description" />
            </div>

            <DialogFooter className="mt-2">
              <Button>Add</Button>
            </DialogFooter>
          </form>
        </DialogContent>
      </Dialog>
    </div>
  )
}
