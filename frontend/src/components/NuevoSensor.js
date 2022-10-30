import React from 'react';
import {Container,Row,Col,Card,Form,Button} from 'react-bootstrap';
import {useState} from 'react';
import { useAuth0 } from '@auth0/auth0-react'
import {Loading} from './Loading';
import {Error403} from './403'

const NuevoSensor = () => {
    const {isAuthenticated,isLoading} = useAuth0();
    const [id,setId]= useState(-1);
    const [idRack,setIdRack] = useState(-1);
    const [tipo,setTipo] = useState("ph");
	const [descirpcion,setDescripcion] = useState();

    async function clickNuevo(e) {
        e.preventDefault();
        if(id<0 || idRack<0 || tipo==undefined || descirpcion==undefined){
            alert("Llena todos los campos");
            return 
        }

        var url = 'https://localhost:8000/sensor/nuevo/';
        const params = {
            id_sensor: id,
            id_rack: idRack,
            tipo: tipo,
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
                alert("Sensor registrado correctamente");
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
                        <Form.Label htmlFor="disabledSelect">Id Rack:</Form.Label>
                        <Form.Control type="number" placeholder="Elije un id valido" onChange={e => setIdRack(e.target.value)}/>
                        </Form.Group>
                    </Row>
                    <Row>
                    <Form.Group className="mb-3">
                    <Form.Label htmlFor="disabledSelect">Tipo: </Form.Label>
                    <Form.Select id="disabledSelect" onChange={e => setTipo(e.target.value)}>
                        <option value="ph">ph</option>
                        <option value="humedad_relativa">humedad_relativa</option>
                        <option value="on_off">on_off</option>
                        <option value="temperatura">temperatura</option>
                    </Form.Select>
                    </Form.Group>
                    </Row>
                    <Row>
                        <Form.Group className="mb-3">
                            <Form.Label>Descripción:</Form.Label>
                            <Form.Control type="text" placeholder="Descripción" onChange= {e => setDescripcion(e.target.value)}/>
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

export default NuevoSensor