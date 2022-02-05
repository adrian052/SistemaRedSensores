import react from 'react';
import {Navbar,Nav,Container,NavDropdown} from 'react-bootstrap'
import { useEffect,useState} from 'react';

const Navigation =() => {
  const [temperatura,setTemperatura]=useState([]);
  const [humedadRelativa,setHumedadRelativa]=useState([]);
  const [ph,setPh]=useState([]);
  const [onOff,setOnOff] = useState([]);

  const getData=(tipo,funcion)=>{
		fetch(`http://localhost:8000/sensores/tipo/${tipo}/`
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

    return (
        <Navbar collapseOnSelect expand="lg" bg="dark" variant="dark">
  <Container>
  <Navbar.Brand>Sistema de red de sensores</Navbar.Brand>
  <Navbar.Toggle aria-controls="responsive-navbar-nav" />
  <Navbar.Collapse id="responsive-navbar-nav">
    <Nav className="me-auto">
      <NavDropdown title="Temperatura" id="collasible-nav-dropdown">
        <NavDropdown.Item href="/promedio/temperatura/">Promedio</NavDropdown.Item>
        {temperatura.map((elem)=>(
        <NavDropdown.Item href={`/sensor/${elem}`}>Sensor {elem}</NavDropdown.Item>
        ))}
      </NavDropdown>
      <NavDropdown title="Humedad Relativa" id="collasible-nav-dropdown">
        <NavDropdown.Item href="/promedio/humedad_relativa/">Promedio</NavDropdown.Item>
        {humedadRelativa.map((elem)=>(
        <NavDropdown.Item href={`/sensor/${elem}`}>Sensor {elem}</NavDropdown.Item>
        ))}
      </NavDropdown>
      <NavDropdown title="On Off" id="collasible-nav-dropdown">
        <NavDropdown.Item href="#action/3.1">Porcentaje Promedio</NavDropdown.Item>
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
      <Nav.Link href="#deets">More deets</Nav.Link>
      <Nav.Link eventKey={2} href="#memes">
        Dank memes
      </Nav.Link>
    </Nav>
  </Navbar.Collapse>
  </Container>
</Navbar>
    );
}


export default Navigation;