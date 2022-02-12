import React from "react";
import {Container,Row,Col,Card,Button} from 'react-bootstrap';
import { useAuth0 } from '@auth0/auth0-react'
import {Loading} from './Loading';
import {_403} from './403'

const Configuracion = () => {
    const {user,isAuthenticated,isLoading} = useAuth0();
    if(isLoading){
		return(<Loading/>)
	}
	if(!isAuthenticated){
		return(<_403/>)
	}

    return (
        <Container>
            <br/>
            <Card>
            <Row>
                
                <Col>
                    <h3 align="center">
                        Manual
                    </h3>
                    <br/>
                    <div align="center">
                    <label>Realizar actualizacion manual.</label><br/>    
                    </div>
                   
                    <br/>
                    <div align="center">
                        
                        <Button variant="success">Update</Button>    
                    </div>  
                    <br/>
                
                </Col>
                <Col>
                
                    <h3 align="center">
                        Automatico
                    </h3>
                    <div align="center">
                    <label>Realizar actualizacion periodicamente.</label><br/>    
                    </div>
                        <div align="center">
                            <input type="number" class="form-control" size="small" placeholder="Minutos"/> 
                            <br/> 
                            <Button variant="success">Update</Button> 
                        </div>  
                    <br/>
                    
                </Col>
            </Row>
            </Card>
        </Container>   
    )
};
export default Configuracion;