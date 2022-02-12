import React from "react";
import {Container,Row,Col,Card,Button,Image} from 'react-bootstrap';
import iencendido from '../images/encendido.png'
import iapagado from '../images/apagado.png'
export const Foco = ({ultima,encendido}) => {
    return (
        <Container>
                {encendido=='true' &&<Row><div><img src={iencendido} width="40" height="50"/></div></Row>}
                {encendido=='false' &&<Row><div><img src={iapagado} width="50" height="50"/></div></Row>}
                <Row>Ultima actualizacion:<br/>{ultima}</Row>           
        </Container>   
    )
};