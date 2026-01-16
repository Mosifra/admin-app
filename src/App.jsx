import { LocationProvider, Router, Route } from "preact-iso"
import AdminPanel from "./pages/AdminPanel"
import AdminLogin from "./pages/Login"
import UniversityList from "./pages/Universities/List.jsx"
import CompanyList from "./pages/Companies/List.jsx"
import CreateCompany from "./pages/Companies/Create.jsx"
import CreateUniversity from "./pages/Universities/Create.jsx"
import Success from "./pages/Success.jsx"

export function App() {
  return (
    <LocationProvider>
      <main class="container">
        <Router>
          <Route path="/" component={AdminPanel} />
          <Route path="/login" component={AdminLogin} />
          <Route path="/universities" component={UniversityList} />
          <Route path="/universities/create" component={CreateUniversity} />
          <Route path="/companies" component={CompanyList} />
          <Route path="/companies/create" component={CreateCompany} />
          <Route path="/success" component={Success} />
        </Router>
      </main>
    </LocationProvider>
  )
}

export default App
