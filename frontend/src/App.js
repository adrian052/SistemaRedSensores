import './App.css';
import GraficaPromedio from './components/GraficaPromedio'
import TemperaturaSensor from './components/TemperaturaSensor'
import GraficasOnOff from './components/GrafficasOnOff';
import Navigation from './components/Navigation';
import Main from './components/Main';
import Configuracion from './components/Configuracion';


import {
  	BrowserRouter,
	Routes,
  	Route
} from "react-router-dom";

const App = () => {
	
	return (
	<>
	<Navigation/>
	<BrowserRouter>
	<Routes>
	<Route path="/">
			<Route path="promedio/">
				<Route path=":tipo/" element={<GraficaPromedio/>}></Route>
			</Route>
			<Route path="sensor/:id" element={<TemperaturaSensor/>}></Route>
			<Route path="sensorOnOff/:id" element={<GraficasOnOff/>}></Route>
			<Route path="configuracion/" element={<Configuracion/>}></Route>
			<Route path="" element={<Main/>}></Route>
	</Route>
	</Routes>
	</BrowserRouter>
	</>
	);
}

export default App;