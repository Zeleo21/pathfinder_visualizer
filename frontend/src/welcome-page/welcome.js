import './../App.css';
import Spline from '@splinetool/react-spline';
import GetStartedButton from './components/GetStartedButton';

export default function Welcome() {
    return (
    <div className="Welcome-page">
        <div className="content-wrapper">
          <GetStartedButton></GetStartedButton>
          <Spline scene="https://prod.spline.design/Y3EOcJ4BYJ-IFVCZ/scene.splinecode" />
        </div>
      </div>
    );
}