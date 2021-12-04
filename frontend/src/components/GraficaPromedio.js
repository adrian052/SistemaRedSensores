import React from 'react';
import { useParams } from "react-router-dom";
import LineChart from './LineChart';
import {Container,Row,Col,Form,Button,Card} from 'react-bootstrap';
import { useEffect,useState} from 'react';

function timeConverter(timestamp){
    var date = new Date(parseInt(timestamp));
    return "["+date.getDate()+
	"/"+(date.getMonth()+1)+
	"/"+date.getFullYear()+
	"]"+date.getHours()+
	":"+date.getMinutes()+
	":"+date.getSeconds();
  }

  const formatPromedio  = (data) => {
	let formated = {};
	for (var n in data){
		var sum = 0;
		var cont = 0;
		for(var m in data[n]){
			sum+= parseInt(data[n][m]);
			cont++;
		}
		formated[n] = sum/cont;
	}
	return formated;
}

const GraficaPromedio  = ()  => {
    const [data,setData]=useState([]);
	const {tipo} = useParams();
	const getData=()=>{
		console.log("hola");
		fetch(`http://localhost:8000/estado/tipo/${tipo}/`
		,{
		  headers : { 
			'Content-Type': 'application/json',
			'Accept': 'application/json'
		   }
		}
		)
		.then(function(response){
		  console.log(response);
		  return response.json();
		})
		.then(function(myJson) {
		console.log(formatPromedio(myJson));
		setData(formatPromedio(myJson));
		}).catch(err => {console.log(err)});
	}


    useEffect(()=>{
		getData();
	},[]);

    return (
        <div>
		    <Container fluid={"sm"}>
				<br/>
				<Row>
					<Col xs={2}>
						<Form.Control type="date" name='date_of_birth'/>
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
								title={`Grafica de ${tipo}`}
								label="Promedio"
		    					labels={Object.keys(data).map((key)=>timeConverter(key))}
		    					data = {Object.keys(data).map((key)=>data[key])}
		    				/>
						</Card>
		    			
		    		</Col>
  		    	</Row>		
		    </Container>	
	    </div>
    );
}

export default GraficaPromedio;