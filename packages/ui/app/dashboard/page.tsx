"use client"

import { Container } from "@/components/Container"
import { useFetchLists } from "@/query/useFetchLists"

const Loading = () => {
  return <div>Loading</div>
}

const NoLists = () => {
  return <div>No lists</div>
}

const Lists = () => {
  return <div>Lists</div>
}

const conditions = {
  loading: Loading,
  noLists: NoLists,
  lists: Lists,
}

export default function Dashboard() {
  const { data, isLoading } = useFetchLists()

  const condition = data?.data.length
    ? "lists"
    : isLoading
      ? "loading"
      : "noLists"

  const Component = conditions[condition]

  return (
    <Container>
      <main>
        <h1 className="text-4xl font-bold text-center">Dashboard Page</h1>

        <div className="mt-20">
          <Component />
        </div>
      </main>
    </Container>
  )
}
