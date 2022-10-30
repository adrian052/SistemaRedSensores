import React from 'react';
import {Navbar,Nav,Container,NavDropdown} from 'react-bootstrap'
import { useEffect,useState} from 'react';
import { useAuth0 } from '@auth0/auth0-react'

const Navigation =() => {
  const { logout,isAuthenticated,loading } = useAuth0();
  const [temperatura,setTemperatura]=useState([]);
  const [humedadRelativa,setHumedadRelativa]=useState([]);
  const [ph,setPh]=useState([]);
  const [onOff,setOnOff] = useState([]);

  const getData=(tipo,funcion)=>{
		fetch(`https://localhost:8000/sensores/tipo/${tipo}/`
		,{
		  headers : { 
			'Content-Type': 'application/json',
			'Accept': 'application/json'
		   }
		}
		)
		.then(function(response){
		  return response.json();
		})
		.then(function(myJson) {
		funcion(myJson);
		}).catch(err => {console.log(err)});
	}


  useEffect(()=>{
		getData("temperatura",setTemperatura);
		getData("humedad_relativa",setHumedadRelativa);
		getData("ph",setPh);
    getData("on_off",setOnOff);
	},[]);

  if(loading){
    return (<div></div>);
  }

    return (
        <Navbar collapseOnSelect expand="lg" bg="dark" variant="dark">
  <Container>
  <Navbar.Brand>Sistema de red de sensores</Navbar.Brand>
  <Navbar.Toggle aria-controls="responsive-navbar-nav" />
  <Navbar.Collapse id="responsive-navbar-nav">
    { isAuthenticated && 
    <>
    <Nav className="me-auto">
      <NavDropdown title="Sensores" id="collasible-nav-dropdown">
        <NavDropdown.Item href={`/nuevo/sensor`}>Nuevo</NavDropdown.Item> 
        <NavDropdown.Item href={`/lista/sensor`}>Lista</NavDropdown.Item> 
      </NavDropdown>
      <NavDropdown title="Racks" id="collasible-nav-dropdown">
        <NavDropdown.Item href={`/nuevo/rack`}>Nuevo</NavDropdown.Item> 
        <NavDropdown.Item href={`/lista/rack`}>Lista</NavDropdown.Item> 
      </NavDropdown>
      <NavDropdown title="Temperatura" id="collasible-nav-dropdown">
        <NavDropdown.Item href="/promedio/temperatura/">Promedio</NavDropdown.Item>
        {temperatura.map((elem)=>(
        <NavDropdown.Item href={`/sensor/${elem}`}>Sensor {elem}</NavDropdown.Item>
        ))}
      </NavDropdown>
      <NavDropdown title="Humedad relativa" id="collasible-nav-dropdown">
        <NavDropdown.Item href="/promedio/humedad_relativa/">Promedio</NavDropdown.Item>
        {humedadRelativa.map((elem)=>(
        <NavDropdown.Item href={`/sensor/${elem}`}>Sensor {elem}</NavDropdown.Item>
        ))}
      </NavDropdown>
      <NavDropdown title="On Off" id="collasible-nav-dropdown">
        {onOff.map((elem)=>(
        <NavDropdown.Item href={`/sensorOnOff/${elem}`}>Sensor {elem}</NavDropdown.Item>
        ))}
      </NavDropdown>
      <NavDropdown title="pH" id="collasible-nav-dropdown">
        <NavDropdown.Item href="/promedio/ph/">Promedio</NavDropdown.Item>
        {ph.map((elem)=>(
        <NavDropdown.Item href={`/sensor/${elem}`}>Sensor {elem}</NavDropdown.Item>
        ))}
      </NavDropdown>
    </Nav>
    <Nav>
      <Nav.Link href="/configuracion/">Configuración</Nav.Link>
      <Nav.Link onClick={() => logout({ returnTo: window.location.origin })}>Logout</Nav.Link>
    </Nav></>}
  </Navbar.Collapse>
  </Container>
</Navbar>
    );
}


export default Navigation;