import { Link } from "react-router-dom"


export const NotFoundLayout = () => {

    return (
        <div>
            <p>Page Not Found</p>
            <p>Go to <Link to="dashboard">Dashboard</Link></p>
        </div>
    )

}