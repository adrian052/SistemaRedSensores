import React from 'react';
import {Container,Row,Col,Card,Form,Button} from 'react-bootstrap';
import {useState,useEffect} from 'react';
import Table from 'react-bootstrap/Table'
import { useAuth0 } from '@auth0/auth0-react'
import {Loading} from './Loading';
import {Error403} from './403'


const ListaSensor = () => {
    const {isAuthenticated,isLoading} = useAuth0();
    const [data,setData]= useState([]);
    const getData=()=>{
      fetch('http://localhost:8000/lista/sensores/',{
        headers : { 
        'Content-Type': 'application/json',
        'Accept': 'application/json'
         }})
      .then(function(response){return response.json();})
      .then(function(myJson) {setData(myJson);console.log(myJson);})
      .catch(err => {console.log(err)});
    }

    
  
  
    useEffect(()=>{
      getData();
    },[]);// eslint-disable-line

    if(isLoading){
      return(<Loading/>)
    }
  
    if(!isAuthenticated){
      return(<Error403/>)
    }

    return <Container>
        <br/>
        <Table striped bordered hover size="sm">
  <thead>
    <tr>
      <th>Id</th>
      <th>Id Rack</th>
      <th>Tipo</th>
      <th>Descripción</th>
    </tr>
  </thead>
  <tbody>
      {data.map((elem)=>(
        <tr>
        <td>{elem.id}</td>
        <td>{elem.id_rack}</td>
        <td>{elem.tipo}</td>
        <td>{elem.descripcion}</td>
      </tr>
        ))}
        </tbody>
</Table>
    </Container>
    
}

export default ListaSensor