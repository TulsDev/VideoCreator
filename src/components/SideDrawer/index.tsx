
import Drawer from '@mui/material/Drawer';
import './index.css'
import { Link } from 'react-router-dom';

type LinkItemProps = {
    label: string
    path: string
    closeMenu: () => void
}

const LinkItem = (props: LinkItemProps) => {
    
    const {label,path,closeMenu} = props;

    return (
        <Link 
            to={path}
            className='sidebar-item'
            onClick={() => closeMenu()}
        >{ label }</Link>
    )
}

type SideDrawerProps = {
    open: boolean
    setOpen: (_:boolean) => void
}

export const SideDrawer = (props: SideDrawerProps) => {

    const {
        open,
        setOpen
    } = props;

    const close = () => setOpen(false)

    return (
        <Drawer open={open} onClose={close}>
            <div className='sidebar'>
                
                <LinkItem 
                    path='/dashboard'
                    label='Dashboard'
                    closeMenu={close}
                />
                
                <LinkItem 
                    path='/projects'
                    label='Projects'
                    closeMenu={close}
                />
            </div>
        </Drawer>
    )

}