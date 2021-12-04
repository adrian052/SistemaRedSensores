import React from 'react';
import { useParams } from "react-router-dom";
import LineChart from './LineChart';
import {Container,Row,Col,Card,Button} from 'react-bootstrap';
import { useEffect,useState} from 'react';
import Form from "react-bootstrap/Form";


function timeConverter(timestamp){
    var date = new Date(parseInt(timestamp));
    return "["+date.getDate()+
	"/"+(date.getMonth()+1)+
	"/"+date.getFullYear()+
	"]"+date.getHours()+
	":"+date.getMinutes()+
	":"+date.getSeconds();
  }

const TemperaturaSensor = () => {
    const {id} = useParams()
    const [data,setData]=useState([]);
	const [informacion,setInformacion] = useState({})

	const getData=()=>{
		fetch('http://localhost:8000/estado/sensor/'+id,{
		  headers : { 
			'Content-Type': 'application/json',
			'Accept': 'application/json'
		   }})
		.then(function(response){return response.json();})
		.then(function(myJson) {setData(myJson);})
		.catch(err => {console.log(err)});
	};
	const getInformacion=()=>{
		fetch('http://localhost:8000/sensor/informacion/'+id,{
		  headers : { 
			'Content-Type': 'application/json',
			'Accept': 'application/json'
		   }})
		.then(function(response){return response.json();})
		.then(function(myJson) {setInformacion(myJson);})
		.catch(err => {console.log(err)});
	}


    useEffect(()=>{
		getData();
		getInformacion();
	},[]);

    return (
    <div><br/>
        <Container fluid={"sm"}>
			<Row>
				<Col xs={2}>
					<Form.Control type="date" name='date_of_birth' />
				</Col>
				<Col xs={2}>
					<Form.Control type="date" name='date_of_birth' />
				</Col><Col xs={2}>
					<Button variant="outline-success">Filtrar</Button>
				</Col>
			</Row>
			<br/>
			<Row>
    			<Col>
				<Card body>
					<LineChart 
						label={`Sensor #${id}`}
						labels={Object.keys(data).map((key)=>timeConverter(key))}
						data = {Object.keys(data).map((key)=>data[key])}
					/>
				</Card>
				</Col>
  			</Row>
			<br/>
			<Row>
    			<Col>
					<Card body>
						<b>Id Sensor:</b> {informacion.id_sensor}<br/>
						<b>Id Rack:</b> {informacion.id_rack}<br/>
						<b>Tipo:</b> {informacion.tipo}<br/>
						<b>Descripcion:</b> {informacion.descripcion}<br/>	
					</Card>
				</Col>
  			</Row>
		</Container>   
    </div>
    );
}

export default TemperaturaSensor;