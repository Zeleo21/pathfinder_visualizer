import './button.css';

export default function GetStartedButton() {
    return (
        <div className='GetStartedButton'>
                <a href="pathfinder">
                    <svg className="icon-arrow before">
                        <use xlinkHref="#arrow"></use>
                            </svg>
                                <span className="label">Get Started</span>
                            <svg className="icon-arrow after">
                        <use xlinkHref="#arrow"></use>
                    </svg>
             </a>

            <svg style={{ display: 'none' }}>
                <defs>
                    <symbol id="arrow" viewBox="0 0 35 15">
                        <title>Arrow</title>
                        <path d="M27.172 5L25 2.828 27.828 0 34.9 7.071l-7.07 7.071L25 11.314 27.314 9H0V5h27.172z" />
                    </symbol>
                </defs>
            </svg>
        </div>
    )
}