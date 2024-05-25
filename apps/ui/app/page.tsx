import { Link } from "@/components/Link"

export default function Home() {
  return (
    <main className="min-h-screen w-full max-w-3xl mx-auto p-10">
      <h1 className="text-4xl font-bold text-center">
        Next Rust Todo List application
      </h1>

      <div className="mt-10">
        <Link href="/login">Login</Link>
      </div>
    </main>
  )
}
