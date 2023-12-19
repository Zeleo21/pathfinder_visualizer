import { useEffect, useState } from "react";
import axios from "axios";
import './maze.css';
import Navbar from '../misc/Navbar';
import {getMaze, dfs} from '../services/axios';
import { useRef } from 'react';
import DisplayDfs from "./components/dfsButton";
import StopButton from "./components/stopButton";

const baseURL = process.env.BACKEND_URL;

export default function DisplayMaze() {
    const [maze, setMaze] = useState(null);
    const [renderMaze, setRenderMaze] = useState(false);
    const [socket, setSocket] = useState(null);
    const [stop, setStop] = useState(false);

    const refreshMaze = () => {
        setRenderMaze(!renderMaze);
    }

    useEffect(() => {
        getMaze().then((data) => {
            setMaze(data.data); 
        });
        
        console.log("connected");
        if(!socket) {
            connect();
        }
                // Reset both actions after processing
    }, [renderMaze]);

    function connect() {
        if (socket && socket.readyState === WebSocket.OPEN) {
          console.log('WebSocket connection is already open');
          return;
        }
      
        const newSocket = new WebSocket('ws://localhost:8080/maze/dfs');
        console.log("new socket");

        newSocket.addEventListener('open', (event) => {
            console.log("Sending a new message");
          newSocket.send('Hello, server!');
          setSocket(newSocket);
        });

        newSocket.addEventListener('message', (event) => {
          console.log('Message from server:', event.data);
          if(event.data === "Hello, server!") {
            return;
          }
          setMaze(event.data);
        })
      
        newSocket.addEventListener('close', (event) => {
          console.log('WebSocket connection closed');
          setSocket(null);
        });
      }

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
                    <DisplayDfs socket={socket} setStop={setStop}></DisplayDfs>
                    {stop && <StopButton setStop={setStop} socket={socket}></StopButton>}
                </div>
            </div>
        </div>
      </>  
    );
}