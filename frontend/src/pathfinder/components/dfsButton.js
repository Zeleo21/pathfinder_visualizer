import { useEffect, useState } from "react";
import axios from "axios";
import {dfs} from '../../services/axios';


export default function DisplayDfs({setMaze}) {

    const DisplayDfs = () => {
        dfs().then((data) => {
            setMaze(data.data);
        })
    }

    return (
        <>
            <button type="button" class="btn btn-primary" id="dfs" onClick={DisplayDfs}>dfs</button>
        </>
    );
}