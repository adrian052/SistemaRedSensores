import React from 'react';
import { useParams } from "react-router-dom";
import LineChart from './LineChart';
import {Container,Row,Col,Form,Card} from 'react-bootstrap';
import { useEffect,useState} from 'react';
import { useAuth0 } from '@auth0/auth0-react'
import {Loading} from './Loading';
import {Error403} from './403'
import TimePicker from 'react-bootstrap-time-picker';


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
	const {isAuthenticated,isLoading} = useAuth0();
    const [data,setData]=useState([]);
	const [initTimestamp,setInitTimestamp]= useState();
	const [lastTimestamp,setLastTimestamp] = useState(); 
	const {tipo} = useParams();
	const getData=()=>{
		var url = `https://localhost:8000/estado/tipo/${tipo}/`;
		console.log(initTimestamp,lastTimestamp);
		if(initTimestamp!==undefined || lastTimestamp!==undefined){
			url+="?";
			if(initTimestamp!==undefined){
				url+="initTimestamp="+initTimestamp;
				if(lastTimestamp!==undefined){
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
	},[initTimestamp,lastTimestamp]);// eslint-disable-line

	if(isLoading){
		return(<Loading/>)
	}

	if(!isAuthenticated){
		return(<Error403/>)
	}

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
						{initTimestamp!==undefined && <TimePicker start="00:00" end="23:00" step={60} />}
					</Col>
					<Col xs={2}>
						<Form.Control type="date" name='date_of_birth' onChange={e => {
																				var year = e.target.value.split('-')[0];
																				var month = (e.target.value.split('-')[1])-1;
																				var day = e.target.value.split('-')[2];
																				setLastTimestamp((new Date(year,month,day,23,59,59).getTime()))}}/>
						{lastTimestamp!==undefined && <TimePicker start="00:00" end="23:00" step={60} />}
					</Col>
				</Row>
				<br/>
		    	<Row>
    	    		<Col>
						<Card body>
							<LineChart 
								title={`Gráfica de ${tipo}`}
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