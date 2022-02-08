import React from 'react';
import {Container} from 'react-bootstrap';
import {LoginButton} from './Login';
import {LogoutButton} from './Logout';
import { useAuth0 } from '@auth0/auth0-react'
import {Loading} from './Loading';

const Main = () => {
    const {user,isAuthenticated,isLoading} = useAuth0();

    if (isLoading) {
        return <Loading/>
    }

    if (isAuthenticated){
        return (
            <Container>
                <div>
                    <h1>Bienvenido {user.name}</h1>
                </div>
                <LogoutButton></LogoutButton>
            </Container>
        );
    }else{
        return (
            <Container>
                <div>
                    <h1>Bienvenido, puedes hacer login en el boton de abajo...</h1>
                </div>
                <LoginButton></LoginButton>
            </Container>
        );
    }
    
}

export default Main;