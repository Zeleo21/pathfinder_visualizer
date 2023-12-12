import { useEffect, useState } from "react";
import axios from "axios";
import './maze.css';
import Navbar from '../misc/Navbar';

const baseURL = process.env.BACKEND_URL;

export default function DisplayMaze() {
    const [maze, setMaze] = useState(null);
    const [render, setRender] = useState(false);

    const refresh = () => {
        setRender(!render);
    }

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
    }, [render]);

    if(!maze) {
        return null;
    }
    //console.log(maze);
    return (
      <>
        <div>
            <Navbar></Navbar>
        </div>
        <div class="container-fluid d-flex flex-column justify-content-between">
            <div class="row">
                <div class="col">
                    <button type="button" class="btn btn-primary" id="reset" onClick={refresh}>Reset</button>
                </div>
                <div class="col-md-6 d-flex flex-column">
                <div className="welcomeText text-center">
                    <h1>Welcome To Pathfinder</h1>
                 </div>
                    <div className="maze" dangerouslySetInnerHTML={{__html: maze}}>
                </div>
                </div>
                <div class="col">
                    
                </div>
            </div>
        </div>
      </>  
    );
}