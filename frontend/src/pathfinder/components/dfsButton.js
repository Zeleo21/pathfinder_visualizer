import { useEffect, useRef, useState } from "react";
import axios from "axios";
import {dfs} from '../../services/axios';


export default function DisplayDfs({socket, setStop}) {

  function handleClick() {
    console.log("CLICKED");
    socket.send("dfs");
    setStop(true);
  }
    return (
        <>
            <button type="button" class="btn btn-primary" id="dfs" onClick={handleClick}>dfs</button>
        </>
    );
}