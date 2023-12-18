import { useEffect, useRef, useState } from "react";
import axios from "axios";
import {dfs} from '../../services/axios';


export default function DisplayDfs({setMaze}) {
    const [socket, setSocket] = useState(null);

    function connect() {
        if (socket && socket.readyState === WebSocket.OPEN) {
          console.log('WebSocket connection is already open');
          return;
        }
      
        const newSocket = new WebSocket('ws://localhost:8080/maze/dfs');
      
        newSocket.addEventListener('open', (event) => {
          newSocket.send('Hello, server!');
          setSocket(newSocket);
        });
      
        newSocket.addEventListener('close', (event) => {
          console.log('WebSocket connection closed');
          setSocket(null);
        });
      }

    useEffect(() => {
        return () => {
          if (socket) {
            socket.close();
            setSocket(null);
          }
        };
      }, [socket]);

    return (
        <>
            <button type="button" class="btn btn-primary" id="dfs" onClick={connect}>dfs</button>
        </>
    );
}