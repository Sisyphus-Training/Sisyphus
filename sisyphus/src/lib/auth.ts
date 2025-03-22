export async function login(identifier: string, password: string) {
    try {
      const res = await fetch("/api/auth/login", {
        method: "POST",
        body: JSON.stringify({ identifier, password }),
        headers: { "Content-Type": "application/json" }
      })
      
      if (!res.ok) throw new Error("Invalid credentials")
      
      return await res.json()
    } catch (error) {
      throw new Error("Login failed")
    }
  }
  