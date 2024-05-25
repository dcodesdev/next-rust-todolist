import { CreateListButton } from "./CreateListButton"

export const NoLists = () => {
  return (
    <div>
      <h2 className="text-2xl font-bold">You have no lists yet. Create one!</h2>

      <CreateListButton />
    </div>
  )
}
