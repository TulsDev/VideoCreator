
import { Outlet } from "react-router-dom"
import { SideDrawer } from "../../components/SideDrawer"
import { useState } from "react"
import '../styles/StaticLayout.css'

export const StaticLayout = () => {

    const [open, setOpen] = useState(false)

    return (
        <div>
            <SideDrawer open={open} setOpen={setOpen} />
            <div className="topbar">
                <button className="sidebar-open" onClick={() => setOpen(true)}>=</button>
            </div>
            <Outlet />
        </div>
    )

}