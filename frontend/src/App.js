import './App.css';
import GraficaPromedio from './components/GraficaPromedio'
import TemperaturaSensor from './components/TemperaturaSensor'
import Navigation from './components/Navigation'

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
			<Route path="sensor/:id" element={<TemperaturaSensor/>}>	
			</Route>
		</Route>
	</Routes>
	</BrowserRouter>
	</>
	);
}

export default App;