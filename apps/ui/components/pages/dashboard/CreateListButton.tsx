import { useCreateTodoList } from "@/api/mutation/useCreateTodoList"
import { useFetchLists } from "@/api/query/useFetchLists"
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
import { useModal } from "@/hooks/useModal"
import { toastError } from "@/utils/toast"
import { FormEvent, useRef } from "react"
import { toast } from "sonner"

export const CreateListButton = () => {
  const { close, isOpen, open, setIsOpen } = useModal()

  const nameRef = useRef<HTMLInputElement>(null)

  const { mutateAsync, isPending } = useCreateTodoList()
  const { refetch } = useFetchLists()

  const onSubmit = (e: FormEvent) => {
    e.preventDefault()

    const name = nameRef.current?.value

    mutateAsync({ name })
      .then(() => {
        refetch()
        close()
        toast.success("List created successfully")
      })
      .catch(toastError)
  }

  return (
    <Dialog open={isOpen} onOpenChange={setIsOpen}>
      <DialogTrigger asChild>
        <Button onClick={open} className="mt-5">
          Create List
        </Button>
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

          <DialogFooter className="mt-2">
            <Button type="submit" disabled={isPending} variant="secondary">
              Create
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  )
}
