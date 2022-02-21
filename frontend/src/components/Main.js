import React from 'react';
import {Container,Card,Row} from 'react-bootstrap';
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
            <br/>
            <Card>
                <br/>
                <Row>
                    <div>
                    <h1 align="center">Bienvenido {user.name}</h1>
                    </div>
                </Row>
                <Row>
                    <div align="center">
                        <LogoutButton></LogoutButton>
                    </div>
                </Row>
                <br/>
            </Card>    
        </Container>
        );
    }else{
        return (
            <Container>
                <br/>
                <Card>
                    <br/>
                    <Row>
                        <div>
                            <h1 align="center">Puedes hacer login en el botón de abajo.</h1>
                        </div>
                    </Row>
                    <Row>
                        <div align="center">
                            <LoginButton></LoginButton>
                        </div>
                    </Row>
                    <br/>
                </Card>    
            </Container>
        );
    }
    
}

export default Main;