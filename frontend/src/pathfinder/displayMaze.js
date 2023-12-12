import { useEffect, useState } from "react";
import axios from "axios";
import './maze.css';

const baseURL = process.env.BACKEND_URL;

export default function DisplayMaze() {
    const [maze, setMaze] = useState(null);
    
    useEffect(() => {
        const body = JSON.stringify({
            "width": 10,
            "height": 10
        })
        axios.post("http://localhost:8080/maze", body, {
            headers: {
                'Content-Type': 'application/json',
            }
        })
        .then((response) => {
            console.log(response.data);
            setMaze(response.data);
        })
    }, [])

    if(!maze) {
        return null;
    }
    //console.log(maze);
    return (
      <>
      {/* <svg
            width="100%"
            height="100%"
            xmlns="http://www.w3.org/2000/svg"
            xmlnsXlink="http://www.w3.org/1999/xlink"
            style="background-color: white;" viewBox="0 0 30 30"
            >
            <path d="M0,0 l0,10" fill="none" stroke="black" stroke-linecap="square" stroke-linejoin="square" stroke-width="2"/>
            <path d="M0,0 l10,0" fill="none" stroke="black" stroke-linecap="square" stroke-linejoin="square" stroke-width="2"/>
            <path d="M10,0 l10,0" fill="none" stroke="black" stroke-linecap="square" stroke-linejoin="square" stroke-width="2"/>
            <path d="M10,0 l0,10" fill="none" stroke="black" stroke-linecap="square" stroke-linejoin="square" stroke-width="2"/>
            <path d="M20,0 l10,0" fill="none" stroke="black" stroke-linecap="square" stroke-linejoin="square" stroke-width="2"/>
            <path d="M30,0 l0,10" fill="none" stroke="black" stroke-linecap="square" stroke-linejoin="square" stroke-width="2"/>
            <path d="M0,10 l0,10" fill="none" stroke="black" stroke-linecap="square" stroke-linejoin="square" stroke-width="2"/>
            <path d="M10,10 l0,10" fill="none" stroke="black" stroke-linecap="square" stroke-linejoin="square" stroke-width="2"/>
            <path d="M20,10 l10,0" fill="none" stroke="black" stroke-linecap="square" stroke-linejoin="square" stroke-width="2"/>
            <path d="M30,10 l0,10" fill="none" stroke="black" stroke-linecap="square" stroke-linejoin="square" stroke-width="2"/>
            <path d="M0,30 l10,0" fill="none" stroke="black" stroke-linecap="square" stroke-linejoin="square" stroke-width="2"/>
            <path d="M0,20 l0,10" fill="none" stroke="black" stroke-linecap="square" stroke-linejoin="square" stroke-width="2"/>
            <path d="M10,30 l10,0" fill="none" stroke="black" stroke-linecap="square" stroke-linejoin="square" stroke-width="2"/>
            <path d="M10,20 l10,0" fill="none" stroke="black" stroke-linecap="square" stroke-linejoin="square" stroke-width="2"/>
            <path d="M30,20 l0,10" fill="none" stroke="black" stroke-linecap="square" stroke-linejoin="square" stroke-width="2"/>
            <path d="M20,30 l10,0" fill="none" stroke="black" stroke-linecap="square" stroke-linejoin="square" stroke-width="2"/>
        </svg> */}
        <div className="maze" dangerouslySetInnerHTML={{__html: maze}} />
      </>  
    );
}