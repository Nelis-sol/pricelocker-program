import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import './index.css'
import { Wallet } from './WalletAdapter.tsx'

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <Wallet />
  </StrictMode>,
)
