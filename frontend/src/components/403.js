import React from "react";
import { useAuth0 } from "@auth0/auth0-react";
import {Button,Container} from 'react-bootstrap';


export const _403 = () => {
    const {loginWithRedirect} = useAuth0();

    return (
        <Container>
            <h1>403 Forbidden </h1>
            <Button variant="primary" size="lg" onClick={()=>{loginWithRedirect()}}>
                Log-in
            </Button>
        </Container>   
    )
};