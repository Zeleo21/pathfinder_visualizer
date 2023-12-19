import axios from "axios";


export const getMaze = () => {
    const body = JSON.stringify({
        "width": 10,
        "height": 10
    })
    return axios.post("http://localhost:8080/maze", body, {
        headers: {
            'Content-Type': 'application/json',
        }
    })
}


export const dfs = () => {
    return axios.get("http://localhost:8080/maze/dfs", {
        headers: {
            'Content-Type': 'application/json',
        }
    })
}