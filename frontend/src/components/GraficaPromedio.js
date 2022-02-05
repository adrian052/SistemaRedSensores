import React from 'react';
import { useParams } from "react-router-dom";
import LineChart from './LineChart';
import {Container,Row,Col,Form,Card} from 'react-bootstrap';
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
	const [initTimestamp,setInitTimestamp]= useState(0);
	const [lastTimestamp,setLastTimestamp] = useState(Date.now()); 
	const {tipo} = useParams();
	const getData=()=>{
		var url = `http://localhost:8000/estado/tipo/${tipo}/`;
		console.log(initTimestamp,lastTimestamp);
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
		fetch(url,
		{
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
	},[initTimestamp,lastTimestamp]);

    return (
        <div>
		    <Container fluid={"sm"}>
				<br/>
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