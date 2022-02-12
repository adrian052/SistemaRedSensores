import React from 'react';
import {Container,Row,Col} from 'react-bootstrap';
import Spinner from 'react-bootstrap/Spinner'
export const Loading = () => {
    return(
    <>	
        <Container>
            <br/>
            <Row>
                <Col sm="5"></Col>
                <Col sm="4">
                    <Spinner animation="border" role="status" variant="warning">
                        <span className="visually-hidden">Loading...</span>
                      </Spinner>
                </Col>
            </Row>
        </Container>
    </>);
}