import { useEffect, useState } from 'react'
import './Popup.css'
import landingtop from "/img/landing/landing-top.png"
import landingbottom from "/img/landing/landing-bottom.png"

function App() {
  const [crx, setCrx] = useState('create-chrome-ext')
  useEffect(() => {
    setTimeout(() => {
      window.location.href = "/setup.html"
    }, 3000)
  }, [])

  return (
    <main>
      <img id='landingtop' src={landingtop} alt='landing-top'/>
      <h3>Superbridge</h3>
      <img id='landingbottom' src={landingbottom} alt='landing-bottom'/>
    </main>
  )
}

export default App
