import React from 'react'
import ReactDOM from 'react-dom/client'
import App from './App.tsx'
import 'sanitize.css';
import './constants/colors.css';
import './constants/fonts.css';

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
);
