import React from 'react';
import {Pie} from 'react-chartjs-2';
import {Chart} from 'react-chartjs-2';
import annotationPlugin from 'chartjs-plugin-annotation';  
import {chartColors} from './Colors'
Chart.register(annotationPlugin);
    
const PieChart = ({title,on,off}) => {
    const data = {
        responsive: true,
        labels: ["ON", "OFF"],
        datasets: [
          {
            data: [on, off],
            backgroundColor: chartColors,
            hoverBackgroundColor: chartColors
          }
        ]
      };
    let chartInstance = null;
    return (
        <Pie
        data={data}
        options = {{
            maintainAspectRatio:true,
            plugins: {
                autocolors: false,
                title: {
                    display: true,
                    text: title,
                    font: {weight: 'bold',size:20}
                }
                
            }
        }}                 
        ref={input => {
          chartInstance = input;
        }}
        />
    );
}

export default PieChart;