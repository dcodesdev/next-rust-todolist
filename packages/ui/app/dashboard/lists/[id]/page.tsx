"use client"

import { useFetchList } from "@/api/query/useFetchList"
import { Container } from "@/components/Container"
import { Loading } from "@/components/Loading"
import { dayjs } from "@/utils/dayjs"

export default function ListPage({ params }: { params: { id: string } }) {
  const { data } = useFetchList(params.id)

  if (!data) return <Loading />

  return (
    <Container>
      <h1 className="text-4xl font-bold">List Page of id {data.list.name}</h1>

      <div className="flex justify-between items-center gap-2 mt-3">
        <p>{data.list.name}</p>
        <p>{dayjs(data.list.created_at).fromNow()}</p>
      </div>

      <br className="border-white border h-1 bg-white w-full" />
    </Container>
  )
}
