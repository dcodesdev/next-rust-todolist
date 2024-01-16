import { useCreateTodoList } from "@/api/mutation/useCreateTodoList"
import { Button } from "@/components/ui/button"
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog"
import { Input } from "@/components/ui/input"
import { FormEvent, useRef } from "react"

export const CreateListButton = () => {
  const nameRef = useRef<HTMLInputElement>(null)

  const { mutate, isPending } = useCreateTodoList()

  const onSubmit = (e: FormEvent) => {
    e.preventDefault()

    const name = nameRef.current?.value

    mutate({ name })
  }

  return (
    <Dialog>
      <DialogTrigger asChild>
        <Button className="mt-5">Create List</Button>
      </DialogTrigger>

      <DialogContent className="bg-black text-gray-50">
        <DialogHeader>
          <DialogTitle>Create List</DialogTitle>
          <DialogDescription>
            Create a new list by entering a name below.
          </DialogDescription>
        </DialogHeader>

        <form onSubmit={onSubmit}>
          <div>
            <Input
              type="text"
              placeholder="Enter the name of your todo list"
              className="bg-black text-gray-50"
              ref={nameRef}
            />
          </div>

          <DialogFooter>
            <Button type="submit" disabled={isPending} variant="secondary">
              Create
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  )
}
