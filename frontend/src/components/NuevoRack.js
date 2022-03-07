import React from 'react';
import {Container,Row,Col,Card,Form,Button} from 'react-bootstrap';
import {useState} from 'react';
import { useAuth0 } from '@auth0/auth0-react'
import {Loading} from './Loading';
import {Error403} from './403'

const NuevoRack = () => {
    const {isAuthenticated,isLoading} = useAuth0();
    const [id,setId]= useState(-1);
	const [descirpcion,setDescripcion] = useState(); 

    async function clickNuevo(e) {
        e.preventDefault();
        if(id<0 || descirpcion==undefined){
            alert("Llena todos los campos");
            return 
        }

        var url = 'http://localhost:8000/rack/nuevo/';
        const params = {
            id: id,
            descripcion: descirpcion 
        };
        
        await fetch(url,{
            method: 'POST',
		  headers : { 
			'Content-Type': 'application/json',
			'Accept': 'application/json',
            'Access-Control-Allow-Origin': '*'
		   },
           body: JSON.stringify(params) 
        }
           )
		.then(function(response){
            if (response.ok){
                alert("Rack registrado correctamente");
            }else{
                if(response.status == 500){
                    alert("Error: El id ya está registrado");
                }else{
                    alert("Error");
                }
                
            }
        }).catch(e => alert("Error no se puede conectar con la API."))        
    }

    if(isLoading){
		return(<Loading/>)
	}

	if(!isAuthenticated){
		return(<Error403/>)
	}

    return <Container>
        <br/>
        <Card>
            <div align="center">
                <Form>
                    <Row>
                        <Form.Group className="mb-3">
                        <Form.Label>Id:</Form.Label>
                        <Form.Control type="number" placeholder="Elije un id valido" onChange={e => setId(e.target.value)}/>
                        </Form.Group>
                    </Row>
                    <Row>
                        <Form.Group className="mb-3">
                            <Form.Label>Descripción:</Form.Label>
                            <Form.Control type="text" placeholder="Descripción" onChange={e => setDescripcion(e.target.value)}/>
                        </Form.Group>
                    </Row>
                    <Button variant="primary" type="submit" onClick={clickNuevo}>
                            Aceptar
                        </Button>
                    
                </Form>
            </div>
        
        </Card>
    </Container>
}

export default NuevoRack