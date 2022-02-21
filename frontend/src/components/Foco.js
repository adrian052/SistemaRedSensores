import React from "react";
import {Container,Row} from 'react-bootstrap';
import iencendido from '../images/encendido.png'
import iapagado from '../images/apagado.png'
export const Foco = ({ultima,encendido}) => {
    return (
        <Container>
                {encendido==='true' &&<Row><div><img src={iencendido} width="40" height="50" alt="encendido"/></div></Row>}
                {encendido==='false' &&<Row><div><img src={iapagado} width="50" height="50" alt="apagado"/></div></Row>}
                <Row>Última actualización:<br/>{ultima}</Row>           
        </Container>   
    )
};