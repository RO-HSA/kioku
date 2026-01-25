import React from 'react';
import ReactDOM from 'react-dom/client';
import App from './App';

import { createTheme, ThemeProvider } from '@mui/material/styles';
import GlobalStyles from '@mui/material/GlobalStyles';

const theme = createTheme({
  cssVariables: {
    colorSchemeSelector: 'data'
  },
  colorSchemes: { light: true, dark: true }
});

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
  <React.StrictMode>
    <ThemeProvider theme={theme}>
      <GlobalStyles styles="@layer theme, base, mui, components, utilities;" />
      <App />
    </ThemeProvider>
  </React.StrictMode>
);
