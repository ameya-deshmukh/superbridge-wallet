import { useState } from 'react'
import './Successful.css'

function App() {
  const [crx, setCrx] = useState('create-chrome-ext')

  return (
    <main>
      <img src="/img/successful/successful-top.png" alt="successful-top" id='success-top'/>
      <div>
        <img src="/img/successful/successful-icon.png" alt="successful-icon" id='success-icon'/>
        <h3>CONGRATULATIONS</h3>
        <p>Your account has been successfully created.</p>
      </div>
      <button>Proceed</button>
    </main>
  )
}

export default App
