"use client"

import { Container } from "@/components/Container"
import { useFetchLists } from "@/api/query/useFetchLists"
import { Lists } from "@/components/pages/dashboard/Lists"
import { LoadingLists } from "@/components/pages/dashboard/LoadingLists"
import { NoLists } from "@/components/pages/dashboard/NoLists"

export default function Dashboard() {
  const { data, isLoading } = useFetchLists()

  const Component = getComponent(data, isLoading)

  return (
    <Container>
      <main>
        <h1 className="text-4xl font-bold text-center">Dashboard Page</h1>

        <div className="mt-20">
          <Component data={data} />
        </div>
      </main>
    </Container>
  )
}

const conditions = {
  LoadingLists,
  NoLists,
  Lists,
}

const getComponent = (
  data: ReturnType<typeof useFetchLists>["data"],
  loading: boolean
) => {
  if (loading) return conditions.LoadingLists
  if (!data?.data.length) return conditions.NoLists
  return conditions.Lists
}
