
import { useEffect } from "preact/hooks"
import { useLocation } from "preact-iso"

export default function AdminPanel() {
  const location = useLocation()

  useEffect(() => {
    const jwt = sessionStorage.getItem("jwt")
    if (!jwt) location.route("/login")
  }, [])

  return (
    <main className=" min-w-screen min-h-screen bg-beige-mosifra">
      <div className="max-w-6xl mx-auto px-4 py-16">
        <h1 className="text-5xl font-bold text-vert-mosifra mb-16">Mosifra - Panel d'administration</h1>

        <div className="grid md:grid-cols-2 gap-8">
          <button
            onClick={() => location.route("/universities")}
            className="bg-white p-10 rounded-xl shadow-md border-l-4 border-vert-mosifra hover:shadow-lg transition text-left"
          >
            <h2 className="text-3xl font-bold text-vert-mosifra mb-4">
              Universités
            </h2>
            <p className="text-gray-600">Gérer les comptes d'universités</p>
          </button>

          <button
            onClick={() => location.route("/companies")}
            className="bg-white p-10 rounded-xl shadow-md border-l-4 border-vert-mosifra hover:shadow-lg transition text-left"
          >
            <h2 className="text-3xl font-bold text-vert-mosifra mb-4">
              Entreprises
            </h2>
            <p className="text-gray-600">Gérer les comptes d'entreprises</p>
          </button>
        </div>
      </div>
    </main>
  )
}
