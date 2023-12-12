import { BrowserRouter, Routes, Route } from 'react-router-dom';
import Welcome from './welcome-page/welcome';
import DisplayMaze from './pathfinder/displayMaze';
export default function App() {
return (
  <BrowserRouter>
    <Routes>
      <Route path="/" element={<Welcome />} />
      <Route path="/pathfinder" element={<DisplayMaze />} />
    </Routes>
  </BrowserRouter>
  );
}
