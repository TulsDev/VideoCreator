import { Navigate, Route, Routes } from "react-router-dom";
import {
  StaticLayout,
  NotFoundLayout,
  DashboardPage,
  ProjectsPage
} from "./pages"

function PageManager() {

  return (
    <Routes>
      <Route path='/' element={<StaticLayout/>}>
        <Route path="/" element={<Navigate to="/dashboard" />}></Route> {/* Auto redirect to fix rect-router-dom default route */}
        <Route path='/dashboard' element={<DashboardPage />}></Route>
        <Route path='/projects' element={<ProjectsPage />}></Route>
      </Route>
      <Route path='*' element={<NotFoundLayout />}></Route>
    </Routes>
  );
}

export default PageManager;
