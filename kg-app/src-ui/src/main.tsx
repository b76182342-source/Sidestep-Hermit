import React from 'react'
import ReactDOM from 'react-dom/client'

import './styles/theme.css'
import './styles/globals.css'
import './styles/layout.css'
import './styles/components.css'
import './styles/player.css'

import App from './App'

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
)
