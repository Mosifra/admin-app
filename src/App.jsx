import { LocationProvider, Router, Route } from "preact-iso"
import AdminPanel from "./pages/AdminPanel"
import CreateUniversity from "./pages/CreateUniversity"

export function App() {
  return (
    <LocationProvider>
      <main class="container">
        <Router>
          <Route path="/" component={AdminPanel} />
          <Route path="/create_university" component={CreateUniversity} />
        </Router>
      </main>
    </LocationProvider>
  )
}

export default App
