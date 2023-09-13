import { useState } from 'react'
import './Setup.css'
import setuptop from "/img/setup/setup-top.png"

function App() {
  const [crx, setCrx] = useState('create-chrome-ext')

  return (
    <main>
      <div id='setup-top'>
        <img src="/img/setup/setup-top.png" alt="setup-top"/>
        <h3 id='header-top'>Superbridge</h3>
      </div>
      <div id='setup-bottom'>
        <div id='setup-bottom-text'>
          <img src="/img/setup/wallet-icon.png" alt="wallet-icon" id='wallet-icon' />
          <h2>Setup your wallet</h2>
          <p>Get started on your journey by either creating or importing a wallet.</p>
        </div>
        <div id='setup-bottom-button'>
          <button id='btn-wallet'>Create new wallet</button>
          <button id='btn-seed'>Import using seed phrase</button>
        </div>
      </div>
    </main>
  )
}

export default App
