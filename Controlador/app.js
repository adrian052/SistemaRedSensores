//imports of express
const express = require('express')
const app = express()
const port = 8000

const {getContract} = require('./config.js');
const cors=require("cors");
const corsOptions ={
   origin:'*', 
   credentials:true,
   optionSuccessStatus:200,
}

app.use(cors(corsOptions))
app.use(express.json());

async function getUltimaActualizacion(contractPromise){
	const contract = await contractPromise;
	const res = await contract.get_time_ultima_actualizacion();
	return res;
}


async function nuevoRack(contractPromise,id,descripcion){
	const contract = await contractPromise;
	const res = await contract.nuevo_rack({args:{"id":id,"descripcion":descripcion}});
	return res;
}

async function nuevoSensor(contractPromise,idSensor,idRack,tipoSensor,descripcion){
	const contract = await contractPromise;
	const res = await contract.nuevo_sensor({args:{"id":idSensor,
											"id_rack":idRack,
											"tipo_sensor":tipoSensor,
											"descripcion":descripcion,}});
											
	return res;
}

async function actualizarEstado(contractPromise,estados){
	
	const contract = await contractPromise;
	let estadosString = [];
	
	for(let i = 0;i<estados.length;i++){
		estadosString.push(estados[i][0]+":"+estados[i][1]);	
	}
	
	const res = await contract.actualizar_estado(
		{args:
			{"estados_string":estadosString,
			"timestamp_update":Date.now()},gas:300000000000000
		});
	return res;
}

async function getHistorial(contractPromise,tipo){
	const contract = await contractPromise;
	//console.log(contract);
	const res = await contract.get_historial(
		{"tipo":tipo,
		"init_timpstamp": 0,
		"last_timpstamp":Date.now()}
		);
	return res;
}

async function getHistorialSensor(contractPromise,sensor){
	const contract = await contractPromise;
	const res = await contract.get_historial_sensor(
		{"id":sensor,
		"init_timpstamp": 0,
		"last_timpstamp":Date.now()}
	);
	return res;
}

async function getSensoresPorTipo(contractPromise,tipo){
	const contract = await contractPromise;
	const res = await contract.get_sensores_por_tipo({"tipo_sensor":tipo});
	return res;
}

async function getSensorInformacion(contractPromise,id){
	const contract = await contractPromise;
	const res = await contract.get_informacion_sensor({"id":id});
	return res;
}

async function nuevoSistema(contractPromise){
	const contract = await contractPromise;
	const res = await contract.new({args:{}});
	return res;
}

///Endpoints
//Actualizaciones
app.get('/ultima_actualizacion', (_,res) => {
  	const value = getUltimaActualizacion(getContract());
  	value.then(val => res.send(val)).catch(err => res.send(err));
});

//Racks
app.post('/rack/nuevo', (req,res) => {
  	const id = req.body.id;
  	const descripcion = req.body.descripcion;
  	const response = nuevoRack(getContract(),id,descripcion);
  	response
	.then(_ => res.status(201).send("Nuevo rack registrado."))
	.catch(err => {res.status(500).send(err)});
});


//Sensores
app.post('/sensor/nuevo', (req,res) => {
	const idRack = req.body.id_rack;
	const idSensor = req.body.id_sensor;
	const tipoSensor = req.body.tipo;
	const descripcion = req.body.descripcion;
	const response = nuevoSensor(getContract(),idSensor,idRack,tipoSensor,descripcion);
	response
	.then(_ => res.status(201).send("Nuevo sensor registrado."))
	.catch(err => {res.status(500).send(err)});
});

app.get('/sensor/informacion/:id',(req,res) =>{
	const response = getSensorInformacion(getContract(),req.params.id);
	response.then(data => res.send(data)).catch(err => {res.status(500).send(err);console.log(err)});
});

app.get('/sensores/tipo/:tipo', (req,res) => {
	const response = getSensoresPorTipo(getContract(),req.params.tipo.toUpperCase());
	response.then(historial => res.send(historial)).catch(err => {res.status(500).send(err);console.log(err)});
});



//Actualizaciones
app.post('/estado/actualizar/',(req,res) => {
	const arregloEstados = req.body.arreglo_estados;
	const response = actualizarEstado(getContract(),arregloEstados);
	
	response
	.then(_ => res.status(201).send("Nueva actualizacion de estados"))
	.catch(err => {res.status(500).send(err)});	
});

//Obtener estados
app.get('/estado/tipo/:tipo/',(req, res) => {
	const response = getHistorial(getContract(),req.params.tipo.toUpperCase());
	response.then(historial => res.send(historial)).catch(err => {res.status(500).send(err);console.log(err)});
});

app.get('/estado/sensor/:id_sensor/',(req,res) => {
	const response = getHistorialSensor(getContract(),req.params.id_sensor);
	response.then(historial => res.send(historial)).catch(err => {res.status(500).send(err);console.log(err)});
});

//Nuevo sistema
app.post('/init/',(req,res)=>{
	response = nuevoSistema(getContract());
	response
	.then(_ => res.status(201).send("Sistema inicializado"))
	.catch(err => {res.status(500).send(err)});	
});


app.listen(port, () => {
  	console.log(`Blockchain API listening on port http://localhost:${port}`);
});
