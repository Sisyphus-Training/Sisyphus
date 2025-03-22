import Link from "next/link"
import { LoginForm } from "./login-form"

export default function Home() {
  return (
    <main className="flex min-h-screen flex-col items-center justify-center bg-gradient-to-b from-slate-900 to-slate-950 p-4">
      <div className="w-full max-w-md rounded-lg border border-slate-800 bg-slate-900 p-8 shadow-xl">
        <div className="mb-6 flex flex-col items-center justify-center">
          <div className="flex h-20 w-20 items-center justify-center rounded-full bg-slate-800">
            <span className="text-2xl font-bold text-cyan-400">FT</span>
          </div>
          <h1 className="mt-6 text-center text-2xl font-bold text-white">Sign in to your account</h1>
        </div>

        <LoginForm />

        <p className="mt-6 text-center text-sm text-slate-400">
          Don&apos;t have an account?{" "}
          <Link href="/signup" className="text-cyan-400 hover:underline">
            Sign up
          </Link>
        </p>
      </div>
    </main>
  )
}

