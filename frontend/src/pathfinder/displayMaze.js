import { useEffect, useState } from "react";
import axios from "axios";
import './maze.css';
import Navbar from '../misc/Navbar';
import {getMaze, dfs} from '../services/axios';
import { useRef } from 'react';
import DisplayDfs from "./components/dfsButton";

const baseURL = process.env.BACKEND_URL;

export default function DisplayMaze() {
    const [maze, setMaze] = useState(null);
    const [renderMaze, setRenderMaze] = useState(false);

    const refreshMaze = () => {
        setRenderMaze(!renderMaze);
    }

    useEffect(() => {
        getMaze().then((data) => {
            setMaze(data.data); 
        });
                // Reset both actions after processing
    }, [renderMaze]);

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
                    <button type="button" class="btn btn-primary" id="reset" onClick={refreshMaze}>Reset</button>
                </div>
                <div class="col-md-6 d-flex flex-column">
                <div className="welcomeText text-center">
                    <h1>Welcome To Pathfinder</h1>
                 </div>
                    <div className="maze" dangerouslySetInnerHTML={{__html: maze}}>
                </div>
                </div>
                <div class="col">
                    <DisplayDfs setMaze={setMaze}></DisplayDfs>
                </div>
            </div>
        </div>
      </>  
    );
}