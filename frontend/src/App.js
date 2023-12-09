import './App.css';
//import NavBar from './misc/Navbar';
import Spline from '@splinetool/react-spline';
import GetStartedButton from './welcome-page/components/GetStartedButton';
export default function App() {
  return (
    <div className="Welcome-page">
      <div className="content-wrapper">
        <GetStartedButton></GetStartedButton>
        <Spline scene="https://prod.spline.design/Mh6polbJfdV2zwSH/scene.splinecode">
        </Spline>
      </div>
    </div>
  );
}
