



export default function StopButton({setStop, socket}) {

    function handleClick() {
        console.log("CLICKED");
        socket.send("stop")
        setStop(false);
    }
    return (
        <>
            <button type="button" class="btn btn-primary" id="stop" onClick={handleClick}>stop</button>
        </>
    );
}