import React from 'react';
import { useParams } from "react-router-dom";
import PieChart from './PieChart';
import {Container,Row,Col,Card,Button} from 'react-bootstrap';
import { useEffect,useState} from 'react';
import Form from "react-bootstrap/Form";

const countOccurrences = (arr, val) => arr.reduce((a, v) => (v === val ? a + 1 : a), 0);

function timeConverter(timestamp){
    var date = new Date(parseInt(timestamp));
    return "["+date.getDate()+
	"/"+(date.getMonth()+1)+
	"/"+date.getFullYear()+
	"]"+date.getHours()+
	":"+date.getMinutes()+
	":"+date.getSeconds();
  }

const GraficasOnOff = () => {
	const [initTimestamp,setInitTimestamp]= useState(0);
	const [lastTimestamp,setLastTimestamp] = useState(Date.now()); 
    const {id} = useParams()
    const [data,setData]=useState([]);
	const [informacion,setInformacion] = useState({})

	const getData=()=>{
		var url = 'http://localhost:8000/estado/sensor/'+id;
		if(initTimestamp!=undefined || lastTimestamp!=undefined){
			url+="?";
			if(initTimestamp!=undefined){
				url+="initTimestamp="+initTimestamp;
				if(lastTimestamp!=undefined){
					url+="&lastTimestamp="+lastTimestamp;
				}
			}else if(lastTimestamp){
				url+="lastTimestamp="+lastTimestamp;
			}
		}
		fetch(url,{
		  headers : { 
			'Content-Type': 'application/json',
			'Accept': 'application/json'
		   }})
		.then(function(response){return response.json();})
		.then(function(myJson) {console.log(myJson); setData(myJson);})
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
	},[initTimestamp,lastTimestamp]);

    return (
    <div><br/>
        <Container fluid={"sm"}>
			<Row>
				<Col xs={2}>
					<Form.Control type="date" name='date_of_birth' onChange={ e => {
																				var year = e.target.value.split('-')[0];
																				var month = e.target.value.split('-')[1]-1;
																				var day = e.target.value.split('-')[2];
																				setInitTimestamp((new Date(year,month,day).getTime()))}}/>
				</Col>
				<Col xs={2}>
					<Form.Control type="date" name='date_of_birth' onChange={e => {
																				var year = e.target.value.split('-')[0];
																				var month = (e.target.value.split('-')[1])-1;
																				var day = e.target.value.split('-')[2];
																				setLastTimestamp((new Date(year,month,day,23,59,59).getTime()))}}/>
				</Col>
			</Row>
			<br/>
			<Row>
    			<Col>
				<Card body>
                    <Row>
                    <Col sm={4}></Col>
                    <Col sm={4}><PieChart 
						label={`Sensor #${id}`}
						labels={Object.keys(data).map((key)=>timeConverter(key))}
						data = {Object.keys(data).map((key)=>data[key])}
                        on =  {countOccurrences(Object.values(data),"true")}
                        off = {countOccurrences(Object.values(data),"false")}
					/></Col>
                    <Col sm={4}></Col>
					</Row>
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

export default GraficasOnOff;