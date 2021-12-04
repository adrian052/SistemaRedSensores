import React from 'react';
import {Line} from 'react-chartjs-2';
import {Chart} from 'react-chartjs-2';
import annotationPlugin from 'chartjs-plugin-annotation';
Chart.register(annotationPlugin);



const LineChart = ({title,labels,label,data}) => {
    return (
            <Line
                data = {{
                    labels,
                    datasets: [
                        {
                            label,
                            data,
                            backgroundColor:'green',
                            borderColor:'green',
                            borderWidth:2,
                        }
                    ]
                }}
                height = {400}
                width = {600}            
                options = {{
                    maintainAspectRatio:false,
                    plugins: {
                        autocolors: false,
                        annotation: {
                          annotations: {
                            line1: {
                              type: 'line',
                              yMin: 30,
                              yMax: 30,
                              borderColor: 'rgb(255, 99, 132)',
                              borderWidth: 2,
                            },
                            line2: {
                                type: 'line',
                                yMin: 20,
                                yMax: 20,
                                borderColor: 'blue',
                                borderWidth: 2,
                              }
                          }
                        },
                        title: {
                            display: true,
                            text: title,
                            font: {weight: 'bold',size:20}
                        }
                        
                    }
                }}
            />
    );
}

export default LineChart;