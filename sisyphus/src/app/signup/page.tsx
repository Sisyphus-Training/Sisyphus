import Link from "next/link"
import Image from "next/image"
import { SignupForm } from "./signup-form"

export default function SignupPage() {
  return (
    <main className="flex min-h-screen flex-col items-center justify-center bg-gradient-to-b from-slate-900 to-slate-950 p-4">
      <div className="w-full max-w-md rounded-lg border border-slate-800 bg-slate-900 pb-8 px-8 shadow-xl">
        <div className="mb-6 flex flex-col items-center justify-center">
          <div className="h-28 w-28 relative">
            <Image
              src="/logo.png"
              alt="The Sisyphus Exercise Routine Builder"
              fill
              className="object-contain"
              priority
            />
          </div>
          <h1 className="mt-4 text-center text-2xl font-bold text-white">Create your account</h1>
        </div>

        <SignupForm />

        <p className="mt-6 text-center text-sm text-slate-400">
          Already have an account?{" "}
          <Link href="/auth/login" className="text-cyan-400 hover:underline">
            Sign in
          </Link>
        </p>
      </div>
    </main>
  )
}
